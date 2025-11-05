use anyhow::{Context, Result};
use tree_sitter::{Node, Parser, Tree};

/// Initialize a Tree-sitter parser configured for Dart
pub fn create_dart_parser() -> Result<Parser> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_dart::language())
        .context("Failed to load Dart grammar for tree-sitter")?;
    Ok(parser)
}

/// Parse Dart source code into a concrete syntax tree (CST)
pub fn parse_dart(source: &str) -> Result<Tree> {
    let mut parser = create_dart_parser()?;
    parser
        .parse(source, None)
        .context("Failed to parse Dart source code")
}

/// Represents a token extracted from the tree-sitter CST
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: String,
    pub text: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: Point,
    pub end_point: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl From<tree_sitter::Point> for Point {
    fn from(p: tree_sitter::Point) -> Self {
        Point {
            row: p.row,
            column: p.column,
        }
    }
}

/// Extract all tokens from a parse tree by walking leaf nodes
pub fn extract_tokens(tree: &Tree, source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let root = tree.root_node();
    extract_tokens_recursive(root, source, &mut tokens);
    tokens
}

fn extract_tokens_recursive(node: Node, source: &str, tokens: &mut Vec<Token>) {
    if node.child_count() == 0 {
        // This is a leaf node, extract it as a token
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            tokens.push(Token {
                kind: node.kind().to_string(),
                text: text.to_string(),
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
                start_point: node.start_position().into(),
                end_point: node.end_position().into(),
            });
        }
    } else {
        // Recurse into children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            extract_tokens_recursive(child, source, tokens);
        }
    }
}

/// Typed wrapper for specific Dart constructs
#[derive(Debug)]
pub struct DartClass<'a> {
    pub name: String,
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug)]
pub struct DartMethod<'a> {
    pub name: String,
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug)]
pub struct DartImport<'a> {
    pub uri: String,
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Extract all class declarations from the CST
pub fn extract_classes<'a>(tree: &'a Tree, source: &str) -> Vec<DartClass<'a>> {
    let mut classes = Vec::new();
    let root = tree.root_node();
    extract_classes_recursive(root, source, &mut classes);
    classes
}

fn extract_classes_recursive<'a>(node: Node<'a>, source: &str, classes: &mut Vec<DartClass<'a>>) {
    if node.kind() == "class_definition" {
        // Find the class name
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                classes.push(DartClass {
                    name: name.to_string(),
                    node,
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                });
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_classes_recursive(child, source, classes);
    }
}

/// Extract all method declarations from the CST
pub fn extract_methods<'a>(tree: &'a Tree, source: &str) -> Vec<DartMethod<'a>> {
    let mut methods = Vec::new();
    let root = tree.root_node();
    extract_methods_recursive(root, source, &mut methods);
    methods
}

fn extract_methods_recursive<'a>(node: Node<'a>, source: &str, methods: &mut Vec<DartMethod<'a>>) {
    if node.kind() == "method_signature" || node.kind() == "function_signature" {
        // Find the method name
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                methods.push(DartMethod {
                    name: name.to_string(),
                    node,
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                });
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_methods_recursive(child, source, methods);
    }
}

/// Extract all import statements from the CST
pub fn extract_imports<'a>(tree: &'a Tree, source: &str) -> Vec<DartImport<'a>> {
    let mut imports = Vec::new();
    let root = tree.root_node();
    extract_imports_recursive(root, source, &mut imports);
    imports
}

fn extract_imports_recursive<'a>(node: Node<'a>, source: &str, imports: &mut Vec<DartImport<'a>>) {
    if node.kind() == "import_or_export" || node.kind() == "import_specification" {
        // Try to find the URI string
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "configurable_uri" || child.kind() == "uri" {
                if let Ok(uri) = child.utf8_text(source.as_bytes()) {
                    imports.push(DartImport {
                        uri: uri.to_string(),
                        node,
                        start_byte: node.start_byte(),
                        end_byte: node.end_byte(),
                    });
                    break;
                }
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_imports_recursive(child, source, imports);
    }
}

/// Walk the entire tree and print the structure (for debugging)
pub fn print_tree(tree: &Tree, source: &str) {
    let root = tree.root_node();
    print_node(root, source, 0);
}

fn print_node(node: Node, source: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let text = node
        .utf8_text(source.as_bytes())
        .unwrap_or("<error>")
        .lines()
        .next()
        .unwrap_or("");
    let preview = if text.len() > 50 {
        format!("{}...", &text[..50])
    } else {
        text.to_string()
    };

    println!(
        "{}{} [{}..{}] {:?}",
        indent,
        node.kind(),
        node.start_byte(),
        node.end_byte(),
        preview
    );

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_node(child, source, depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_class() {
        let source = r#"
class MyClass {
    int x = 1;
    void myMethod() {
        print(x);
    }
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let root = tree.root_node();
        assert_eq!(root.kind(), "program");
        assert!(!root.has_error());
    }

    #[test]
    fn test_extract_tokens() {
        let source = "class A { int x = 1; }";
        let tree = parse_dart(source).expect("Failed to parse");
        let tokens = extract_tokens(&tree, source);

        // Should have multiple tokens including keywords, identifiers, etc.
        assert!(!tokens.is_empty());

        // Find the 'class' keyword token
        let has_class = tokens.iter().any(|t| t.text == "class");
        assert!(has_class, "Should find 'class' keyword");
    }

    #[test]
    fn test_extract_classes() {
        let source = r#"
class FirstClass {
    int x;
}

class SecondClass {
    String name;
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let classes = extract_classes(&tree, source);

        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0].name, "FirstClass");
        assert_eq!(classes[1].name, "SecondClass");
    }

    #[test]
    fn test_extract_imports() {
        let source = r#"
import 'dart:core';
import 'package:flutter/material.dart';

class MyClass {}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let imports = extract_imports(&tree, source);

        assert!(!imports.is_empty());
    }

    #[test]
    fn test_error_recovery() {
        // Test with incomplete/invalid code - tree-sitter should still parse
        let source = "class { incomplete";
        let tree = parse_dart(source).expect("Failed to parse");
        let root = tree.root_node();

        // Tree-sitter should parse it but mark errors
        assert_eq!(root.kind(), "program");
        // Note: has_error() checks if there are error nodes in the tree
    }

    #[test]
    fn test_token_positions() {
        let source = "class A {}";
        let tree = parse_dart(source).expect("Failed to parse");
        let tokens = extract_tokens(&tree, source);

        for token in &tokens {
            // All tokens should have valid positions
            assert!(token.end_byte >= token.start_byte);
            assert_eq!(&source[token.start_byte..token.end_byte], token.text);
        }
    }
}
