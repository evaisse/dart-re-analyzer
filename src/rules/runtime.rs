use crate::analyzer::Rule;
use crate::error::{Diagnostic, Location, Result, RuleCategory, Severity};
use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

// Static regex patterns compiled once
fn dynamic_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\bdynamic\b").expect("Invalid regex pattern"))
}

fn catch_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"catch\s*\([^)]*\)\s*\{\s*\}").expect("Invalid regex pattern")
    })
}

fn import_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r#"(?m)^import\s+['"]([^'"]+)['"](?:\s+as\s+(\w+))?;"#)
            .expect("Invalid regex pattern")
    })
}

fn print_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\bprint\s*\(").expect("Invalid regex pattern"))
}

fn null_check_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\!\.").expect("Invalid regex pattern")
    })
}

// Rule: Avoid using dynamic type
pub struct AvoidDynamicRule;

impl Rule for AvoidDynamicRule {
    fn name(&self) -> &str {
        "avoid_dynamic"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            // Skip comments
            if line.trim_start().starts_with("//") {
                continue;
            }

            for mat in dynamic_regex().find_iter(line) {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        "Avoid using 'dynamic' type as it bypasses type safety",
                        Severity::Warning,
                        RuleCategory::Runtime,
                        Location {
                            file: file_path.to_string_lossy().to_string(),
                            line: line_num + 1,
                            column: mat.start() + 1,
                            end_line: Some(line_num + 1),
                            end_column: Some(mat.end() + 1),
                        },
                    )
                    .with_suggestion("Use a specific type or Object? instead"),
                );
            }
        }

        Ok(diagnostics)
    }
}

// Rule: Avoid empty catch blocks
pub struct AvoidEmptyCatchRule;

impl Rule for AvoidEmptyCatchRule {
    fn name(&self) -> &str {
        "avoid_empty_catch"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(mat) = catch_regex().find(line) {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        "Empty catch block swallows exceptions silently",
                        Severity::Error,
                        RuleCategory::Runtime,
                        Location {
                            file: file_path.to_string_lossy().to_string(),
                            line: line_num + 1,
                            column: mat.start() + 1,
                            end_line: Some(line_num + 1),
                            end_column: Some(mat.end() + 1),
                        },
                    )
                    .with_suggestion("Handle the exception or at least log it"),
                );
            }
        }

        Ok(diagnostics)
    }
}

// Rule: Detect unused imports
// Note: This is a simplified heuristic-based implementation
// A full implementation would require semantic analysis with an AST
pub struct UnusedImportRule;

impl Rule for UnusedImportRule {
    fn name(&self) -> &str {
        "unused_import"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        
        let mut imports = Vec::new();
        
        // Collect all imports
        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = import_regex().captures(line) {
                let import_path = caps.get(1).unwrap().as_str();
                let alias = caps.get(2).map(|m| m.as_str());
                
                // Extract the imported symbols to check
                let symbol_to_check = if let Some(alias) = alias {
                    alias.to_string()
                } else if let Some(last_part) = import_path.split('/').last() {
                    last_part.trim_end_matches(".dart").to_string()
                } else {
                    continue;
                };
                
                imports.push((line_num, symbol_to_check, line.to_string()));
            }
        }
        
        // Check if each import is used in the file
        // Note: This is a simple heuristic that checks for literal string matches
        // It may produce false positives for:
        // - Imports used only in type annotations
        // - Imports used through qualified access
        // - Transitive dependencies
        for (line_num, symbol, import_line) in imports {
            let mut is_used = false;
            
            // Check if symbol appears elsewhere in the file
            for (check_line_num, line) in content.lines().enumerate() {
                if check_line_num == line_num {
                    continue; // Skip the import line itself
                }
                
                if line.contains(&symbol) {
                    is_used = true;
                    break;
                }
            }
            
            if !is_used {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        format!("Import '{}' is unused", symbol),
                        Severity::Warning,
                        RuleCategory::Runtime,
                        Location {
                            file: file_path.to_string_lossy().to_string(),
                            line: line_num + 1,
                            column: 1,
                            end_line: Some(line_num + 1),
                            end_column: Some(import_line.len()),
                        },
                    )
                    .with_suggestion("Remove this unused import"),
                );
            }
        }

        Ok(diagnostics)
    }
}

// Rule: Avoid using print statements (use logging instead)
pub struct AvoidPrintRule;

impl Rule for AvoidPrintRule {
    fn name(&self) -> &str {
        "avoid_print"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            // Skip comments
            if line.trim_start().starts_with("//") {
                continue;
            }

            for mat in print_regex().find_iter(line) {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        "Avoid using 'print' in production code",
                        Severity::Info,
                        RuleCategory::Runtime,
                        Location {
                            file: file_path.to_string_lossy().to_string(),
                            line: line_num + 1,
                            column: mat.start() + 1,
                            end_line: Some(line_num + 1),
                            end_column: Some(mat.end() + 1),
                        },
                    )
                    .with_suggestion("Use a proper logging library like logger or developer.log"),
                );
            }
        }

        Ok(diagnostics)
    }
}

// Rule: Avoid null check operator on nullable types without null checking
pub struct AvoidNullCheckOnNullableRule;

impl Rule for AvoidNullCheckOnNullableRule {
    fn name(&self) -> &str {
        "avoid_null_check_on_nullable"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            // Skip comments
            if line.trim_start().starts_with("//") {
                continue;
            }

            for mat in null_check_regex().find_iter(line) {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        "Using null assertion operator (!) can cause runtime errors if value is null",
                        Severity::Warning,
                        RuleCategory::Runtime,
                        Location {
                            file: file_path.to_string_lossy().to_string(),
                            line: line_num + 1,
                            column: mat.start() + 1,
                            end_line: Some(line_num + 1),
                            end_column: Some(mat.end() + 1),
                        },
                    )
                    .with_suggestion("Use null-aware operators (?., ??) or null checks instead"),
                );
            }
        }

        Ok(diagnostics)
    }
}
