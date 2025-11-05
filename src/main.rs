mod analyzer;
mod config;
mod error;
mod mcp;
mod parser;
mod rules;

use analyzer::Rule;
use clap::{Parser, Subcommand};
use config::AnalyzerConfig;
use error::{Diagnostic, Result};
use mcp::{start_mcp_server, McpServer};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "dart-re-analyzer")]
#[command(about = "A high-performance Rust-based Dart/Flutter analyzer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze Dart/Flutter project
    Analyze {
        /// Path to the Dart/Flutter project
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Only run style rules
        #[arg(long)]
        style_only: bool,

        /// Only run runtime rules
        #[arg(long)]
        runtime_only: bool,

        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,

        /// Configuration file path
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Start MCP server for error fetching
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "9000")]
        port: u16,

        /// Path to the Dart/Flutter project to watch
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Configuration file path
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Generate default configuration file
    InitConfig {
        /// Output path for the configuration file
        #[arg(default_value = "analyzer_config.json")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            path,
            style_only,
            runtime_only,
            format,
            config,
        } => {
            let config = load_config(config)?;
            let diagnostics = analyze_project(&path, &config, style_only, runtime_only)?;

            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&diagnostics)
                        .expect("Failed to serialize diagnostics");
                    println!("{}", json);
                }
                _ => {
                    print_diagnostics(&diagnostics);
                }
            }

            // Exit with error code if there are errors
            let has_errors = diagnostics
                .iter()
                .any(|d| matches!(d.severity, error::Severity::Error));
            if has_errors {
                std::process::exit(1);
            }
        }
        Commands::Serve { port, path, config } => {
            let config = load_config(config)?;
            let mcp = Arc::new(McpServer::new());

            // Initial analysis
            let diagnostics = analyze_project(&path, &config, false, false)?;
            mcp.update_diagnostics(diagnostics).await;

            println!("Starting MCP server on port {}...", port);
            start_mcp_server(port, mcp).await?;
        }
        Commands::InitConfig { output } => {
            let config = AnalyzerConfig::default();
            config.save_to_file(&output)?;
            println!("Configuration file created at: {}", output.display());
        }
    }

    Ok(())
}

fn load_config(config_path: Option<PathBuf>) -> Result<AnalyzerConfig> {
    if let Some(path) = config_path {
        AnalyzerConfig::load_from_file(&path)
    } else {
        // Try to find config in current directory
        let default_paths = ["analyzer_config.json", ".dart_analyzer_config.json"];
        for path_str in &default_paths {
            let path = PathBuf::from(path_str);
            if path.exists() {
                return AnalyzerConfig::load_from_file(&path);
            }
        }
        Ok(AnalyzerConfig::default())
    }
}

fn analyze_project(
    path: &Path,
    config: &AnalyzerConfig,
    style_only: bool,
    runtime_only: bool,
) -> Result<Vec<Diagnostic>> {
    println!("Analyzing Dart files in: {}", path.display());

    // Find all Dart files
    let files = parser::find_dart_files(path)?;
    println!("Found {} Dart files", files.len());

    // Select rules based on flags
    let rules: Vec<Arc<dyn Rule>> = if style_only {
        rules::get_style_rules()
    } else if runtime_only {
        rules::get_runtime_rules()
    } else {
        rules::get_all_rules()
    };

    println!("Running {} rules", rules.len());

    // Run analysis
    let diagnostics = if config.parallel {
        analyze_parallel(&files, &rules)
    } else {
        analyze_sequential(&files, &rules)
    };

    println!("Analysis complete. Found {} issues", diagnostics.len());

    Ok(diagnostics)
}

fn analyze_parallel(files: &[parser::DartFile], rules: &[Arc<dyn Rule>]) -> Vec<Diagnostic> {
    files
        .par_iter()
        .flat_map(|file| {
            let path = PathBuf::from(&file.path);
            rules
                .iter()
                .flat_map(|rule| rule.check(&path, &file.content).unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn analyze_sequential(files: &[parser::DartFile], rules: &[Arc<dyn Rule>]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for file in files {
        let path = PathBuf::from(&file.path);
        for rule in rules {
            if let Ok(file_diagnostics) = rule.check(&path, &file.content) {
                diagnostics.extend(file_diagnostics);
            }
        }
    }

    diagnostics
}

fn print_diagnostics(diagnostics: &[Diagnostic]) {
    use std::collections::HashMap;

    if diagnostics.is_empty() {
        println!("\n✓ No issues found!");
        return;
    }

    // Group by file
    let mut by_file: HashMap<String, Vec<&Diagnostic>> = HashMap::new();
    for diag in diagnostics {
        by_file
            .entry(diag.location.file.clone())
            .or_default()
            .push(diag);
    }

    println!("\nIssues found:\n");

    for (file, diags) in by_file.iter() {
        println!("{}:", file);
        for diag in diags {
            let severity_icon = match diag.severity {
                error::Severity::Error => "✗",
                error::Severity::Warning => "⚠",
                error::Severity::Info => "ℹ",
            };

            println!(
                "  {} [{}:{}] {} ({}): {}",
                severity_icon,
                diag.location.line,
                diag.location.column,
                diag.category,
                diag.rule_id,
                diag.message
            );

            if let Some(ref suggestion) = diag.suggestion {
                println!("    💡 {}", suggestion);
            }
        }
        println!();
    }

    // Print summary
    let errors = diagnostics
        .iter()
        .filter(|d| matches!(d.severity, error::Severity::Error))
        .count();
    let warnings = diagnostics
        .iter()
        .filter(|d| matches!(d.severity, error::Severity::Warning))
        .count();
    let info = diagnostics
        .iter()
        .filter(|d| matches!(d.severity, error::Severity::Info))
        .count();

    println!("Summary:");
    println!(
        "  {} errors, {} warnings, {} info messages",
        errors, warnings, info
    );
}
