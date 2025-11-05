use crate::error::{AnalyzerError, Diagnostic, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub success: bool,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

pub struct McpServer {
    diagnostics: Arc<RwLock<Vec<Diagnostic>>>,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            diagnostics: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn update_diagnostics(&self, diagnostics: Vec<Diagnostic>) {
        let mut store = self.diagnostics.write().await;
        *store = diagnostics;
    }

    pub async fn get_all_diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.read().await.clone()
    }

    pub async fn get_filtered_diagnostics(&self, query: &ErrorsQuery) -> Vec<Diagnostic> {
        let diagnostics = self.diagnostics.read().await;
        
        diagnostics
            .iter()
            .filter(|d| {
                if let Some(ref cat) = query.category {
                    if d.category.to_string() != *cat {
                        return false;
                    }
                }
                
                if let Some(ref sev) = query.severity {
                    if d.severity.to_string() != *sev {
                        return false;
                    }
                }
                
                if let Some(ref file) = query.file {
                    if !d.location.file.contains(file) {
                        return false;
                    }
                }
                
                true
            })
            .cloned()
            .collect()
    }

    pub async fn handle_request(&self, request: McpRequest) -> McpResponse {
        match request.method.as_str() {
            "get_all_errors" => {
                let diagnostics = self.get_all_diagnostics().await;
                McpResponse {
                    success: true,
                    data: serde_json::to_value(diagnostics).unwrap(),
                    error: None,
                }
            }
            "get_errors" => {
                match serde_json::from_value::<ErrorsQuery>(request.params) {
                    Ok(query) => {
                        let diagnostics = self.get_filtered_diagnostics(&query).await;
                        McpResponse {
                            success: true,
                            data: serde_json::to_value(diagnostics).unwrap(),
                            error: None,
                        }
                    }
                    Err(e) => McpResponse {
                        success: false,
                        data: serde_json::Value::Null,
                        error: Some(format!("Invalid query parameters: {}", e)),
                    },
                }
            }
            "get_stats" => {
                let diagnostics = self.get_all_diagnostics().await;
                let stats = DiagnosticsStats::from_diagnostics(&diagnostics);
                McpResponse {
                    success: true,
                    data: serde_json::to_value(stats).unwrap(),
                    error: None,
                }
            }
            _ => McpResponse {
                success: false,
                data: serde_json::Value::Null,
                error: Some(format!("Unknown method: {}", request.method)),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsStats {
    pub total: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
    pub style_issues: usize,
    pub runtime_issues: usize,
    pub files_with_issues: usize,
}

impl DiagnosticsStats {
    pub fn from_diagnostics(diagnostics: &[Diagnostic]) -> Self {
        use crate::error::{RuleCategory, Severity};
        use std::collections::HashSet;

        let total = diagnostics.len();
        let errors = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Error))
            .count();
        let warnings = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Warning))
            .count();
        let info = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Severity::Info))
            .count();
        let style_issues = diagnostics
            .iter()
            .filter(|d| matches!(d.category, RuleCategory::Style))
            .count();
        let runtime_issues = diagnostics
            .iter()
            .filter(|d| matches!(d.category, RuleCategory::Runtime))
            .count();

        let files: HashSet<_> = diagnostics.iter().map(|d| &d.location.file).collect();
        let files_with_issues = files.len();

        Self {
            total,
            errors,
            warnings,
            info,
            style_issues,
            runtime_issues,
            files_with_issues,
        }
    }
}

// Simple JSON-RPC server implementation
pub async fn start_mcp_server(
    port: u16,
    mcp: Arc<McpServer>,
) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpListener;

    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| AnalyzerError::McpServer(format!("Failed to bind to {}: {}", addr, e)))?;

    println!("MCP server listening on {}", addr);

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .map_err(|e| AnalyzerError::McpServer(format!("Failed to accept connection: {}", e)))?;

        let mcp_clone = mcp.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            while let Ok(n) = reader.read_line(&mut line).await {
                if n == 0 {
                    break;
                }

                if let Ok(request) = serde_json::from_str::<McpRequest>(&line) {
                    let response = mcp_clone.handle_request(request).await;
                    if let Ok(response_json) = serde_json::to_string(&response) {
                        let _ = writer.write_all(response_json.as_bytes()).await;
                        let _ = writer.write_all(b"\n").await;
                    }
                }

                line.clear();
            }
        });
    }
}
