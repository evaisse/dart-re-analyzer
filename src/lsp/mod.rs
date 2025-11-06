use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::analyzer::Rule;
use crate::config::AnalyzerConfig;
use crate::error::Diagnostic;
use crate::parser;
use crate::rules;

/// LSP message header
#[derive(Debug)]
struct LspMessage {
    #[allow(dead_code)]
    content_length: usize,
    content: String,
}

/// LSP Proxy that forwards messages between client and Dart Analysis Server,
/// injecting additional diagnostics from dart-re-analyzer
pub struct LspProxy {
    dart_process: Option<Child>,
    dart_stdin: Option<ChildStdin>,
    dart_stdout: Option<BufReader<ChildStdout>>,
    dart_binary: Option<String>,
    config: AnalyzerConfig, // TODO: Use config.exclude_patterns for file filtering
    rules: Vec<Arc<dyn Rule>>,
    workspace_root: PathBuf,
    diagnostics_cache: Arc<Mutex<HashMap<String, Vec<Diagnostic>>>>,
}

impl LspProxy {
    /// Create a new LSP proxy
    pub fn new(
        dart_binary: Option<String>,
        config: AnalyzerConfig,
        workspace_root: PathBuf,
    ) -> Self {
        let rules = rules::get_all_rules();

        Self {
            dart_process: None,
            dart_stdin: None,
            dart_stdout: None,
            dart_binary,
            config,
            rules,
            workspace_root,
            diagnostics_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start the Dart Analysis Server process
    pub fn start_dart_server(&mut self) -> Result<()> {
        let dart_cmd = self
            .dart_binary
            .clone()
            .unwrap_or_else(|| "dart".to_string());

        let mut child = Command::new(&dart_cmd)
            .arg("language-server")
            .arg("--protocol=lsp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context(format!(
                "Failed to start Dart Analysis Server. Make sure '{}' is in your PATH.",
                dart_cmd
            ))?;

        let stdin = child.stdin.take().context("Failed to capture stdin")?;
        let stdout = child.stdout.take().context("Failed to capture stdout")?;

        self.dart_stdin = Some(stdin);
        self.dart_stdout = Some(BufReader::new(stdout));
        self.dart_process = Some(child);

        eprintln!("Dart Analysis Server started successfully");
        Ok(())
    }

    /// Read an LSP message from a reader
    ///
    /// Note: This uses blocking I/O with read_exact(). In production environments with
    /// unreliable connections, consider adding timeout mechanisms or using async I/O.
    fn read_message<R: BufRead>(reader: &mut R) -> Result<Option<LspMessage>> {
        let mut content_length: Option<usize> = None;
        let mut line = String::new();

        // Read headers
        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line)?;

            if bytes_read == 0 {
                return Ok(None); // EOF
            }

            let line = line.trim();

            if line.is_empty() {
                break; // End of headers
            }

            if let Some(value) = line.strip_prefix("Content-Length: ") {
                content_length = Some(value.parse()?);
            }
        }

        let content_length = content_length.context("Missing Content-Length header")?;

        // Read content
        let mut content = vec![0u8; content_length];
        reader.read_exact(&mut content)?;

        Ok(Some(LspMessage {
            content_length,
            content: String::from_utf8(content)?,
        }))
    }

    /// Write an LSP message to a writer
    fn write_message<W: Write>(writer: &mut W, content: &str) -> Result<()> {
        write!(
            writer,
            "Content-Length: {}\r\n\r\n{}",
            content.len(),
            content
        )?;
        writer.flush()?;
        Ok(())
    }

    /// Convert dart-re-analyzer diagnostic to LSP diagnostic format
    fn diagnostic_to_lsp(diag: &Diagnostic) -> Value {
        let severity = match diag.severity {
            crate::error::Severity::Error => 1,   // Error
            crate::error::Severity::Warning => 2, // Warning
            crate::error::Severity::Info => 3,    // Information
        };

        json!({
            "range": {
                "start": {
                    "line": diag.location.line.saturating_sub(1), // LSP is 0-indexed
                    "character": diag.location.column.saturating_sub(1)
                },
                "end": {
                    "line": diag.location.line.saturating_sub(1),
                    "character": diag.location.column.saturating_sub(1) + 1
                }
            },
            "severity": severity,
            "code": diag.rule_id,
            "source": "dart-re-analyzer",
            "message": diag.message,
        })
    }

    /// Run the LSP proxy loop
    pub async fn run(&mut self) -> Result<()> {
        // Start dart server
        self.start_dart_server()?;

        // Take ownership of the streams
        let mut dart_stdin = self.dart_stdin.take().context("Dart stdin not available")?;
        let dart_stdout = self
            .dart_stdout
            .take()
            .context("Dart stdout not available")?;

        let (client_tx, mut client_rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let (server_tx, mut server_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

        // Spawn thread to read from client (stdin)
        let client_tx_clone = client_tx.clone();
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            let mut reader = BufReader::new(stdin);
            loop {
                match LspProxy::read_message(&mut reader) {
                    Ok(Some(msg)) => {
                        if client_tx_clone.send(msg.content).is_err() {
                            break;
                        }
                    }
                    Ok(None) => break, // EOF
                    Err(e) => {
                        eprintln!("Error reading from client: {}", e);
                        break;
                    }
                }
            }
        });

        // Spawn thread to read from Dart server
        let server_tx_clone = server_tx.clone();
        let mut dart_stdout_reader = dart_stdout;
        std::thread::spawn(move || {
            loop {
                match LspProxy::read_message(&mut dart_stdout_reader) {
                    Ok(Some(msg)) => {
                        if server_tx_clone.send(msg.content).is_err() {
                            break;
                        }
                    }
                    Ok(None) => break, // EOF
                    Err(e) => {
                        eprintln!("Error reading from Dart server: {}", e);
                        break;
                    }
                }
            }
        });

        let mut initialized = false;
        let diagnostics_cache = Arc::clone(&self.diagnostics_cache);
        let workspace_root = self.workspace_root.clone();
        let rules = self.rules.clone();

        // Main event loop
        let mut stdout = std::io::stdout();
        loop {
            tokio::select! {
                // Message from client to server
                Some(content) = client_rx.recv() => {
                    // Parse message
                    if let Ok(msg) = serde_json::from_str::<Value>(&content) {
                        // Check for initialize request
                        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
                            if method == "initialize" {
                                eprintln!("LSP initialize request received");
                                // We'll run analysis after initialized
                            }
                        }

                        // Forward to Dart server
                        Self::write_message(&mut dart_stdin, &content)?;
                    }
                }

                // Message from server to client
                Some(content) = server_rx.recv() => {
                    // Parse message
                    if let Ok(mut msg) = serde_json::from_str::<Value>(&content) {
                        // Check for initialized notification
                        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
                            if method == "initialized" && !initialized {
                                initialized = true;
                                // Analyze workspace in background
                                let cache_clone = Arc::clone(&diagnostics_cache);
                                let workspace_clone = workspace_root.clone();
                                let rules_clone = rules.clone();
                                tokio::spawn(async move {
                                    if let Err(e) = Self::analyze_workspace_static(
                                        &workspace_clone,
                                        &rules_clone,
                                        cache_clone
                                    ).await {
                                        eprintln!("Error analyzing workspace: {}", e);
                                    }
                                });
                            }
                        }

                        // Inject our diagnostics if applicable
                        Self::inject_diagnostics_static(&mut msg, &diagnostics_cache).await?;

                        // Write to client
                        let modified_content = serde_json::to_string(&msg)?;
                        Self::write_message(&mut stdout, &modified_content)?;
                    } else {
                        // Forward as-is if we can't parse
                        Self::write_message(&mut stdout, &content)?;
                    }
                }

                else => {
                    break;
                }
            }
        }

        Ok(())
    }

    /// Static version of analyze_workspace
    async fn analyze_workspace_static(
        workspace_root: &Path,
        rules: &[Arc<dyn Rule>],
        cache: Arc<Mutex<HashMap<String, Vec<Diagnostic>>>>,
    ) -> Result<()> {
        eprintln!("Analyzing workspace: {}", workspace_root.display());

        let files = parser::find_dart_files(workspace_root)?;
        eprintln!("Found {} Dart files to analyze", files.len());

        let mut cache_lock = cache.lock().await;
        cache_lock.clear();

        for file in &files {
            let path = PathBuf::from(&file.path);
            let mut file_diagnostics = Vec::new();

            for rule in rules {
                if let Ok(diags) = rule.check(&path, &file.content) {
                    file_diagnostics.extend(diags);
                }
            }

            if !file_diagnostics.is_empty() {
                cache_lock.insert(file.path.clone(), file_diagnostics);
            }
        }

        eprintln!(
            "Analysis complete. Found diagnostics in {} files",
            cache_lock.len()
        );
        Ok(())
    }

    /// Static version of inject_diagnostics
    async fn inject_diagnostics_static(
        message: &mut Value,
        cache: &Arc<Mutex<HashMap<String, Vec<Diagnostic>>>>,
    ) -> Result<()> {
        // Check if this is a publishDiagnostics notification
        if let Some(method) = message.get("method").and_then(|m| m.as_str()) {
            if method == "textDocument/publishDiagnostics" {
                if let Some(params) = message.get_mut("params") {
                    if let Some(uri) = params.get("uri").and_then(|u| u.as_str()) {
                        // Convert URI to file path
                        let file_path = uri.strip_prefix("file://").unwrap_or(uri);

                        // Get cached diagnostics for this file
                        let cache_lock = cache.lock().await;
                        if let Some(our_diagnostics) = cache_lock.get(file_path) {
                            // Get existing diagnostics array
                            let diagnostics_array =
                                params.get_mut("diagnostics").and_then(|d| d.as_array_mut());

                            if let Some(diags) = diagnostics_array {
                                // Add our diagnostics
                                for our_diag in our_diagnostics {
                                    diags.push(LspProxy::diagnostic_to_lsp(our_diag));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Clone for LspProxy {
    /// Creates a new LspProxy with the same configuration but no process handles.
    /// The cloned instance will need start_dart_server() called before use.
    /// This is primarily used for spawning background analysis tasks.
    fn clone(&self) -> Self {
        Self {
            dart_process: None,
            dart_stdin: None,
            dart_stdout: None,
            dart_binary: self.dart_binary.clone(),
            config: self.config.clone(),
            rules: self.rules.clone(),
            workspace_root: self.workspace_root.clone(),
            diagnostics_cache: Arc::clone(&self.diagnostics_cache),
        }
    }
}

impl Drop for LspProxy {
    fn drop(&mut self) {
        if let Some(mut child) = self.dart_process.take() {
            let _ = child.kill();
        }
    }
}
