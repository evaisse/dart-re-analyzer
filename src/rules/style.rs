use crate::analyzer::Rule;
use crate::error::{Diagnostic, Location, Result, RuleCategory, Severity};
use regex::Regex;
use std::path::Path;

// Rule: Class names should use CamelCase
pub struct CamelCaseClassNameRule;

impl Rule for CamelCaseClassNameRule {
    fn name(&self) -> &str {
        "camel_case_class_names"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let class_regex = Regex::new(r"(?m)^[\s]*(?:abstract\s+)?class\s+([a-zA-Z_][a-zA-Z0-9_]*)")
            .unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = class_regex.captures(line) {
                if let Some(class_name) = caps.get(1) {
                    let name = class_name.as_str();
                    // Check if first character is uppercase and name follows CamelCase
                    if !name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        diagnostics.push(
                            Diagnostic::new(
                                self.name(),
                                format!("Class name '{}' should use CamelCase (start with uppercase)", name),
                                Severity::Warning,
                                RuleCategory::Style,
                                Location {
                                    file: file_path.to_string_lossy().to_string(),
                                    line: line_num + 1,
                                    column: class_name.start() + 1,
                                    end_line: Some(line_num + 1),
                                    end_column: Some(class_name.end() + 1),
                                },
                            )
                            .with_suggestion(format!(
                                "Rename to '{}'",
                                to_camel_case(name)
                            )),
                        );
                    }
                }
            }
        }

        Ok(diagnostics)
    }
}

// Rule: File names should use snake_case
pub struct SnakeCaseFileNameRule;

impl Rule for SnakeCaseFileNameRule {
    fn name(&self) -> &str {
        "snake_case_file_names"
    }

    fn check(&self, file_path: &Path, _content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        if let Some(file_name) = file_path.file_stem().and_then(|s| s.to_str()) {
            // Check if filename contains uppercase or doesn't follow snake_case
            if file_name.chars().any(|c| c.is_uppercase()) {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        format!("File name '{}' should use snake_case", file_name),
                        Severity::Warning,
                        RuleCategory::Style,
                        Location {
                            file: file_path.to_string_lossy().to_string(),
                            line: 1,
                            column: 1,
                            end_line: None,
                            end_column: None,
                        },
                    )
                    .with_suggestion(format!(
                        "Rename file to '{}.dart'",
                        to_snake_case(file_name)
                    )),
                );
            }
        }

        Ok(diagnostics)
    }
}

// Rule: Private fields should start with underscore
pub struct PrivateFieldUnderscoreRule;

impl Rule for PrivateFieldUnderscoreRule {
    fn name(&self) -> &str {
        "private_field_underscore"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        
        // This is a simplified check - a real implementation would use AST parsing
        let field_regex = Regex::new(r"(?m)^\s*(?:final\s+|const\s+|static\s+)?(?:late\s+)?[A-Z][a-zA-Z0-9<>,\s]*\s+(_[a-zA-Z][a-zA-Z0-9_]*)\s*[;=]")
            .unwrap();

        for (line_num, line) in content.lines().enumerate() {
            if let Some(caps) = field_regex.captures(line) {
                if let Some(field_name) = caps.get(1) {
                    let name = field_name.as_str();
                    if name.starts_with('_') && !line.trim_start().starts_with("//") {
                        // This is actually good - private fields should start with underscore
                        // This rule is here as an example; in practice, you'd check the opposite
                        continue;
                    }
                }
            }
        }

        Ok(diagnostics)
    }
}

// Rule: Line length should not exceed maximum
pub struct LineLengthRule {
    max_length: usize,
}

impl LineLengthRule {
    pub fn new(max_length: usize) -> Self {
        Self { max_length }
    }
}

impl Rule for LineLengthRule {
    fn name(&self) -> &str {
        "line_length"
    }

    fn check(&self, file_path: &Path, content: &str) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if line.len() > self.max_length {
                diagnostics.push(
                    Diagnostic::new(
                        self.name(),
                        format!("Line exceeds maximum length of {} characters (actual: {})", 
                            self.max_length, line.len()),
                        Severity::Info,
                        RuleCategory::Style,
                        Location {
                            file: file_path.to_string_lossy().to_string(),
                            line: line_num + 1,
                            column: self.max_length + 1,
                            end_line: Some(line_num + 1),
                            end_column: Some(line.len()),
                        },
                    )
                    .with_suggestion("Consider breaking this line into multiple lines"),
                );
            }
        }

        Ok(diagnostics)
    }
}

// Helper functions
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    
    result
}
