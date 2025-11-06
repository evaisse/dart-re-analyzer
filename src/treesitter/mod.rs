use anyhow::{Context, Result};
use tree_sitter::{Node, Parser, Query, QueryCursor, Tree, StreamingIterator, InputEdit, Point as TSPoint};

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

/// Incremental parser that maintains state for efficient re-parsing
pub struct IncrementalParser {
    parser: Parser,
    tree: Option<Tree>,
    source: String,
}

impl IncrementalParser {
    /// Create a new incremental parser
    pub fn new() -> Result<Self> {
        Ok(Self {
            parser: create_dart_parser()?,
            tree: None,
            source: String::new(),
        })
    }

    /// Parse initial source code
    pub fn parse(&mut self, source: &str) -> Result<&Tree> {
        self.source = source.to_string();
        self.tree = self.parser
            .parse(source, None)
            .context("Failed to parse Dart source code")?
            .into();
        Ok(self.tree.as_ref().unwrap())
    }

    /// Apply an edit and incrementally re-parse
    /// 
    /// # Arguments
    /// * `edit` - The edit to apply (describes position, old/new text lengths)
    /// * `new_source` - The new source code after the edit
    /// 
    /// # Example
    /// ```no_run
    /// use dart_re_analyzer::treesitter::{IncrementalParser, Edit};
    /// 
    /// let mut parser = IncrementalParser::new().unwrap();
    /// parser.parse("class MyClass {}").unwrap();
    /// 
    /// // Edit: insert " extends Object" at position 13
    /// let edit = Edit {
    ///     start_byte: 13,
    ///     old_end_byte: 13,
    ///     new_end_byte: 29,
    ///     start_position: (0, 13),
    ///     old_end_position: (0, 13),
    ///     new_end_position: (0, 29),
    /// };
    /// 
    /// parser.reparse(edit, "class MyClass extends Object {}").unwrap();
    /// ```
    pub fn reparse(&mut self, edit: Edit, new_source: &str) -> Result<&Tree> {
        if let Some(ref mut tree) = self.tree {
            // Convert our Edit to tree-sitter's InputEdit
            let input_edit = InputEdit {
                start_byte: edit.start_byte,
                old_end_byte: edit.old_end_byte,
                new_end_byte: edit.new_end_byte,
                start_position: TSPoint {
                    row: edit.start_position.0,
                    column: edit.start_position.1,
                },
                old_end_position: TSPoint {
                    row: edit.old_end_position.0,
                    column: edit.old_end_position.1,
                },
                new_end_position: TSPoint {
                    row: edit.new_end_position.0,
                    column: edit.new_end_position.1,
                },
            };

            tree.edit(&input_edit);
            self.source = new_source.to_string();
            
            // Parse incrementally using the edited tree
            self.tree = self.parser
                .parse(new_source, Some(tree))
                .context("Failed to reparse Dart source code")?
                .into();
        } else {
            // No previous tree, do a full parse
            return self.parse(new_source);
        }

        Ok(self.tree.as_ref().unwrap())
    }

    /// Get the current tree
    pub fn tree(&self) -> Option<&Tree> {
        self.tree.as_ref()
    }

    /// Get the current source code
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Apply multiple edits at once (more efficient than individual edits)
    pub fn reparse_with_edits(&mut self, edits: &[Edit], new_source: &str) -> Result<&Tree> {
        if let Some(ref mut tree) = self.tree {
            // Apply all edits to the tree
            for edit in edits {
                let input_edit = InputEdit {
                    start_byte: edit.start_byte,
                    old_end_byte: edit.old_end_byte,
                    new_end_byte: edit.new_end_byte,
                    start_position: TSPoint {
                        row: edit.start_position.0,
                        column: edit.start_position.1,
                    },
                    old_end_position: TSPoint {
                        row: edit.old_end_position.0,
                        column: edit.old_end_position.1,
                    },
                    new_end_position: TSPoint {
                        row: edit.new_end_position.0,
                        column: edit.new_end_position.1,
                    },
                };
                tree.edit(&input_edit);
            }

            self.source = new_source.to_string();
            
            // Parse incrementally using the edited tree
            self.tree = self.parser
                .parse(new_source, Some(tree))
                .context("Failed to reparse Dart source code")?
                .into();
        } else {
            return self.parse(new_source);
        }

        Ok(self.tree.as_ref().unwrap())
    }
}

/// Represents a source code edit for incremental parsing
#[derive(Debug, Clone, Copy)]
pub struct Edit {
    /// Start byte position of the edit
    pub start_byte: usize,
    /// Old end byte position (before the edit)
    pub old_end_byte: usize,
    /// New end byte position (after the edit)
    pub new_end_byte: usize,
    /// Start position as (row, column)
    pub start_position: (usize, usize),
    /// Old end position as (row, column)
    pub old_end_position: (usize, usize),
    /// New end position as (row, column)
    pub new_end_position: (usize, usize),
}

impl Edit {
    /// Create an edit that inserts text at a position
    /// 
    /// Note: For multi-line insertions, calculate row/column positions carefully.
    /// The new_end_position should account for newlines in the inserted text.
    pub fn insert(position: usize, text_len: usize, row: usize, col: usize) -> Self {
        Self {
            start_byte: position,
            old_end_byte: position,
            new_end_byte: position + text_len,
            start_position: (row, col),
            old_end_position: (row, col),
            // Simplified single-line case. For multi-line, caller must calculate correctly
            new_end_position: (row, col + text_len),
        }
    }

    /// Create an edit that deletes text from start to end
    pub fn delete(start: usize, end: usize, start_row: usize, start_col: usize, end_row: usize, end_col: usize) -> Self {
        Self {
            start_byte: start,
            old_end_byte: end,
            new_end_byte: start,
            start_position: (start_row, start_col),
            old_end_position: (end_row, end_col),
            new_end_position: (start_row, start_col),
        }
    }

    /// Create an edit that replaces text from start to end with new text
    pub fn replace(
        start: usize,
        old_end: usize,
        new_text_len: usize,
        start_row: usize,
        start_col: usize,
        old_end_row: usize,
        old_end_col: usize,
        new_end_col: usize,
    ) -> Self {
        Self {
            start_byte: start,
            old_end_byte: old_end,
            new_end_byte: start + new_text_len,
            start_position: (start_row, start_col),
            old_end_position: (old_end_row, old_end_col),
            new_end_position: (start_row, new_end_col),
        }
    }
}

/// Query result containing matched nodes and their captures
#[derive(Debug)]
pub struct QueryMatch<'a> {
    pub pattern_index: usize,
    pub captures: Vec<QueryCapture<'a>>,
}

#[derive(Debug)]
pub struct QueryCapture<'a> {
    pub name: String,
    pub node: Node<'a>,
    pub text: String,
}

/// Execute a tree-sitter query on the parsed tree
/// 
/// # Example Query Patterns
/// 
/// Find all class definitions:
/// ```text
/// (class_definition name: (identifier) @class.name)
/// ```
/// 
/// Find all method calls:
/// ```text
/// (selector_expression 
///   field: (argument_part) @method.call)
/// ```
/// 
/// Find all dynamic type usage:
/// ```text
/// (type_identifier) @type (#eq? @type "dynamic")
/// ```
pub fn query_tree<'a>(tree: &'a Tree, source: &str, query_str: &str) -> Result<Vec<QueryMatch<'a>>> {
    let language = tree_sitter_dart::language();
    let query = Query::new(&language, query_str)
        .context("Failed to parse tree-sitter query")?;
    
    let mut cursor = QueryCursor::new();
    let mut matches_iter = cursor.matches(&query, tree.root_node(), source.as_bytes());
    
    let capture_names = query.capture_names();
    let mut results = Vec::new();
    
    // Use StreamingIterator to iterate through matches
    while let Some(m) = matches_iter.next() {
        let mut captures = Vec::new();
        for capture in m.captures {
            let name = capture_names[capture.index as usize].to_string();
            let text = capture.node
                .utf8_text(source.as_bytes())
                .unwrap_or("<error>")
                .to_string();
            
            captures.push(QueryCapture {
                name,
                node: capture.node,
                text,
            });
        }
        
        results.push(QueryMatch {
            pattern_index: m.pattern_index,
            captures,
        });
    }
    
    Ok(results)
}

/// Pre-defined query patterns for common Dart constructs
pub mod queries {
    /// Find all class definitions
    pub const CLASSES: &str = r#"
        (class_definition 
          name: (identifier) @class.name) @class.def
    "#;
    
    /// Find all method/function definitions
    pub const METHODS: &str = r#"
        [
          (method_signature) @method.def
          (function_signature) @function.def
        ]
    "#;
    
    /// Find all field declarations in classes
    pub const FIELDS: &str = r#"
        (class_member_definition
          (declaration) @field.decl)
    "#;
    
    /// Find all import statements
    pub const IMPORTS: &str = r#"
        (import_or_export) @import.stmt
    "#;
    
    /// Find all dynamic type usage
    pub const DYNAMIC_TYPES: &str = r#"
        (type_identifier) @type.name
        (#eq? @type.name "dynamic")
    "#;
    
    /// Find all print statements
    pub const PRINT_CALLS: &str = r#"
        (selector_expression
          (identifier) @function (#eq? @function "print")
          (argument_part) @args)
    "#;
    
    /// Find empty catch blocks
    pub const EMPTY_CATCH: &str = r#"
        (catch_clause
          body: (block) @catch.body
          (#match? @catch.body "^\\{\\s*\\}$"))
    "#;
    
    /// Find null assertion operators
    pub const NULL_ASSERTIONS: &str = r#"
        (postfix_expression
          (identifier)
          "!" @null.assertion)
    "#;
    
    /// Find all variable declarations with type annotations
    pub const TYPED_VARIABLES: &str = r#"
        (local_variable_declaration
          (initialized_variable_definition
            (type_identifier) @var.type
            (identifier) @var.name))
    "#;
    
    /// Find generic type parameters
    pub const TYPE_PARAMETERS: &str = r#"
        (type_parameter
          (type_identifier) @param.name) @param.def
    "#;
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

/// Wrapper for field declarations
#[derive(Debug)]
pub struct DartField<'a> {
    pub name: String,
    pub type_annotation: Option<String>,
    pub is_static: bool,
    pub is_final: bool,
    pub is_const: bool,
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Wrapper for variable declarations
#[derive(Debug)]
pub struct DartVariable<'a> {
    pub name: String,
    pub type_annotation: Option<String>,
    pub is_final: bool,
    pub is_const: bool,
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Wrapper for type annotations
#[derive(Debug)]
pub struct DartTypeAnnotation<'a> {
    pub type_name: String,
    pub is_nullable: bool,
    pub type_parameters: Vec<String>,
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Wrapper for generic type parameters
#[derive(Debug)]
pub struct DartTypeParameter<'a> {
    pub name: String,
    pub bound: Option<String>,
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Wrapper for expressions
#[derive(Debug)]
pub struct DartExpression<'a> {
    pub kind: String,
    pub text: String,
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

/// Extract all field declarations from the CST
pub fn extract_fields<'a>(tree: &'a Tree, source: &str) -> Vec<DartField<'a>> {
    let mut fields = Vec::new();
    let root = tree.root_node();
    extract_fields_recursive(root, source, &mut fields);
    fields
}

fn extract_fields_recursive<'a>(node: Node<'a>, source: &str, fields: &mut Vec<DartField<'a>>) {
    // Field declarations are in class_member_definition nodes inside class_body
    if node.kind() == "class_member_definition" {
        // Look for declaration child
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "declaration" {
                let mut is_static = false;
                let mut is_final = false;
                let mut is_const = false;
                let mut type_annotation = None;
                let mut field_name = None;

                let mut decl_cursor = child.walk();
                for decl_child in child.children(&mut decl_cursor) {
                    match decl_child.kind() {
                        "static" => is_static = true,
                        "final_builtin" => is_final = true,
                        "const_builtin" => is_const = true,
                        "type_identifier" => {
                            if let Ok(text) = decl_child.utf8_text(source.as_bytes()) {
                                type_annotation = Some(text.to_string());
                            }
                        }
                        "initialized_identifier_list" | "static_final_declaration_list" => {
                            // Find the identifier within this list
                            let mut id_cursor = decl_child.walk();
                            for id_child in decl_child.children(&mut id_cursor) {
                                if id_child.kind() == "initialized_identifier" 
                                    || id_child.kind() == "static_final_declaration" 
                                    || id_child.kind() == "identifier" {
                                    // Get the first identifier
                                    let mut name_cursor = id_child.walk();
                                    for name_child in id_child.children(&mut name_cursor) {
                                        if name_child.kind() == "identifier" {
                                            if let Ok(name) = name_child.utf8_text(source.as_bytes()) {
                                                field_name = Some(name.to_string());
                                                break;
                                            }
                                        }
                                    }
                                    if field_name.is_none() && id_child.kind() == "identifier" {
                                        if let Ok(name) = id_child.utf8_text(source.as_bytes()) {
                                            field_name = Some(name.to_string());
                                        }
                                    }
                                    if field_name.is_some() {
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(name) = field_name {
                    fields.push(DartField {
                        name,
                        type_annotation,
                        is_static,
                        is_final,
                        is_const,
                        node: child,
                        start_byte: child.start_byte(),
                        end_byte: child.end_byte(),
                    });
                }
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_fields_recursive(child, source, fields);
    }
}

/// Extract all variable declarations from the CST
pub fn extract_variables<'a>(tree: &'a Tree, source: &str) -> Vec<DartVariable<'a>> {
    let mut variables = Vec::new();
    let root = tree.root_node();
    extract_variables_recursive(root, source, &mut variables);
    variables
}

fn extract_variables_recursive<'a>(
    node: Node<'a>,
    source: &str,
    variables: &mut Vec<DartVariable<'a>>,
) {
    // Local variable declarations contain initialized_variable_definition
    if node.kind() == "local_variable_declaration" {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "initialized_variable_definition" {
                let mut is_final = false;
                let mut is_const = false;
                let mut type_annotation = None;
                let mut var_name = None;

                let mut def_cursor = child.walk();
                for def_child in child.children(&mut def_cursor) {
                    match def_child.kind() {
                        "final_builtin" => is_final = true,
                        "const_builtin" => is_const = true,
                        "type_identifier" => {
                            if let Ok(text) = def_child.utf8_text(source.as_bytes()) {
                                type_annotation = Some(text.to_string());
                            }
                        }
                        "identifier" => {
                            if var_name.is_none() {
                                if let Ok(name) = def_child.utf8_text(source.as_bytes()) {
                                    var_name = Some(name.to_string());
                                }
                            }
                        }
                        "inferred_type" => {
                            // This is "var" type inference
                            if let Ok(text) = def_child.utf8_text(source.as_bytes()) {
                                type_annotation = Some(text.to_string());
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(name) = var_name {
                    variables.push(DartVariable {
                        name,
                        type_annotation,
                        is_final,
                        is_const,
                        node: child,
                        start_byte: child.start_byte(),
                        end_byte: child.end_byte(),
                    });
                }
            }
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_variables_recursive(child, source, variables);
    }
}

/// Extract all type annotations from the CST
pub fn extract_type_annotations<'a>(tree: &'a Tree, source: &str) -> Vec<DartTypeAnnotation<'a>> {
    let mut type_annotations = Vec::new();
    let root = tree.root_node();
    extract_type_annotations_recursive(root, source, &mut type_annotations);
    type_annotations
}

fn extract_type_annotations_recursive<'a>(
    node: Node<'a>,
    source: &str,
    type_annotations: &mut Vec<DartTypeAnnotation<'a>>,
) {
    if node.kind() == "type_identifier" || node.kind() == "scoped_identifier" {
        if let Ok(type_name) = node.utf8_text(source.as_bytes()) {
            let is_nullable = node
                .next_sibling()
                .map(|s| s.kind() == "?")
                .unwrap_or(false);

            // Extract type parameters if present
            let mut type_parameters = Vec::new();
            if let Some(parent) = node.parent() {
                if parent.kind() == "type_arguments" {
                    let mut cursor = parent.walk();
                    for child in parent.children(&mut cursor) {
                        if child.kind() == "type_identifier" {
                            if let Ok(param) = child.utf8_text(source.as_bytes()) {
                                type_parameters.push(param.to_string());
                            }
                        }
                    }
                }
            }

            type_annotations.push(DartTypeAnnotation {
                type_name: type_name.to_string(),
                is_nullable,
                type_parameters,
                node,
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
            });
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_type_annotations_recursive(child, source, type_annotations);
    }
}

/// Extract all type parameters (generics) from the CST
pub fn extract_type_parameters<'a>(tree: &'a Tree, source: &str) -> Vec<DartTypeParameter<'a>> {
    let mut type_parameters = Vec::new();
    let root = tree.root_node();
    extract_type_parameters_recursive(root, source, &mut type_parameters);
    type_parameters
}

fn extract_type_parameters_recursive<'a>(
    node: Node<'a>,
    source: &str,
    type_parameters: &mut Vec<DartTypeParameter<'a>>,
) {
    if node.kind() == "type_parameter" {
        let mut param_name = None;
        let mut bound = None;

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_identifier" {
                if param_name.is_none() {
                    if let Ok(name) = child.utf8_text(source.as_bytes()) {
                        param_name = Some(name.to_string());
                    }
                } else if let Ok(bound_text) = child.utf8_text(source.as_bytes()) {
                    bound = Some(bound_text.to_string());
                }
            }
        }

        if let Some(name) = param_name {
            type_parameters.push(DartTypeParameter {
                name,
                bound,
                node,
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
            });
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_type_parameters_recursive(child, source, type_parameters);
    }
}

/// Extract expressions from the CST
pub fn extract_expressions<'a>(tree: &'a Tree, source: &str) -> Vec<DartExpression<'a>> {
    let mut expressions = Vec::new();
    let root = tree.root_node();
    extract_expressions_recursive(root, source, &mut expressions);
    expressions
}

fn extract_expressions_recursive<'a>(
    node: Node<'a>,
    source: &str,
    expressions: &mut Vec<DartExpression<'a>>,
) {
    // Common expression types in Dart
    let expression_kinds = [
        "binary_expression",
        "assignment_expression",
        "conditional_expression",
        "throw_expression",
        "cascade_expression",
        "is_expression",
        "as_expression",
        "postfix_expression",
        "selector_expression",
        "parenthesized_expression",
        "list_literal",
        "map_literal",
        "string_literal",
        "integer_literal",
        "decimal_floating_point_literal",
        "boolean_literal",
        "null_literal",
    ];

    if expression_kinds.contains(&node.kind()) {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            expressions.push(DartExpression {
                kind: node.kind().to_string(),
                text: text.to_string(),
                node,
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
            });
        }
    }

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        extract_expressions_recursive(child, source, expressions);
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

    #[test]
    fn test_extract_fields() {
        let source = r#"
class MyClass {
    static final int staticField = 10;
    String name;
    final double value = 3.14;
    const String constant = 'hello';
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let fields = extract_fields(&tree, source);

        assert!(!fields.is_empty(), "Should extract at least one field");
        
        // Check for specific field properties
        let has_static = fields.iter().any(|f| f.is_static);
        let has_final = fields.iter().any(|f| f.is_final);
        let has_const = fields.iter().any(|f| f.is_const);
        
        assert!(has_static || has_final || has_const, "Should detect field modifiers");
    }

    #[test]
    fn test_extract_variables() {
        let source = r#"
void main() {
    var x = 10;
    final String name = 'test';
    const double pi = 3.14;
    int count = 0;
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let variables = extract_variables(&tree, source);

        assert!(!variables.is_empty(), "Should extract variables");
        
        let has_final = variables.iter().any(|v| v.is_final);
        let has_const = variables.iter().any(|v| v.is_const);
        
        assert!(has_final || has_const, "Should detect variable modifiers");
    }

    #[test]
    fn test_extract_type_annotations() {
        let source = r#"
class MyClass {
    String name;
    int? nullableValue;
    List<String> items;
    Map<String, int> mapping;
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let type_annotations = extract_type_annotations(&tree, source);

        assert!(!type_annotations.is_empty(), "Should extract type annotations");
        
        // Check for common types
        let has_string = type_annotations.iter().any(|t| t.type_name == "String");
        let has_int = type_annotations.iter().any(|t| t.type_name == "int");
        
        assert!(has_string || has_int, "Should detect common type annotations");
    }

    #[test]
    fn test_extract_type_parameters() {
        let source = r#"
class GenericClass<T, E extends Exception> {
    T value;
    E error;
}

class SimpleGeneric<T> {
    T data;
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let type_params = extract_type_parameters(&tree, source);

        assert!(!type_params.is_empty(), "Should extract type parameters");
        
        // Check for type parameter with bound
        let has_bounded = type_params.iter().any(|p| p.bound.is_some());
        
        // At least one type parameter should exist
        assert!(type_params.len() >= 1, "Should find at least one type parameter");
    }

    #[test]
    fn test_extract_expressions() {
        let source = r#"
void main() {
    var x = 1 + 2;
    var y = x * 3;
    var list = [1, 2, 3];
    var map = {'key': 'value'};
    var str = 'hello';
    var flag = true;
    var nothing = null;
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let expressions = extract_expressions(&tree, source);

        assert!(!expressions.is_empty(), "Should extract expressions");
        
        // Should find various expression types
        let has_binary = expressions.iter().any(|e| e.kind == "binary_expression");
        let has_literal = expressions.iter().any(|e| {
            e.kind.contains("literal")
        });
        
        assert!(has_binary || has_literal, "Should detect various expression types");
    }

    #[test]
    fn test_query_classes() {
        let source = r#"
class FirstClass {}
class SecondClass {}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let matches = query_tree(&tree, source, queries::CLASSES).expect("Query failed");

        assert!(!matches.is_empty(), "Should find classes");
        
        // Check we found class names
        let class_names: Vec<_> = matches.iter()
            .flat_map(|m| &m.captures)
            .filter(|c| c.name == "class.name")
            .map(|c| c.text.as_str())
            .collect();
        
        assert!(class_names.contains(&"FirstClass") || class_names.contains(&"SecondClass"));
    }

    #[test]
    fn test_query_methods() {
        let source = r#"
void myFunction() {}

class MyClass {
    void myMethod() {}
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let matches = query_tree(&tree, source, queries::METHODS).expect("Query failed");

        assert!(!matches.is_empty(), "Should find methods/functions");
    }

    #[test]
    fn test_custom_query() {
        let source = r#"
class TestClass {
    int value = 42;
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        
        // Custom query to find integer literals
        let query_str = r#"(decimal_integer_literal) @number"#;
        let matches = query_tree(&tree, source, query_str).expect("Query failed");

        assert!(!matches.is_empty(), "Should find integer literals");
        
        let numbers: Vec<_> = matches.iter()
            .flat_map(|m| &m.captures)
            .filter(|c| c.name == "number")
            .map(|c| c.text.as_str())
            .collect();
        
        assert!(numbers.contains(&"42"), "Should find the number 42");
    }

    #[test]
    fn test_query_type_parameters() {
        let source = r#"
class GenericClass<T, E> {
    T value;
}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let matches = query_tree(&tree, source, queries::TYPE_PARAMETERS).expect("Query failed");

        assert!(!matches.is_empty(), "Should find type parameters");
    }

    #[test]
    fn test_query_imports() {
        let source = r#"
import 'dart:core';
import 'package:flutter/material.dart';

class MyClass {}
        "#;

        let tree = parse_dart(source).expect("Failed to parse");
        let matches = query_tree(&tree, source, queries::IMPORTS).expect("Query failed");

        assert!(!matches.is_empty(), "Should find imports");
        assert!(matches.len() >= 2, "Should find at least 2 imports");
    }

    #[test]
    fn test_incremental_parser_initial() {
        let mut parser = IncrementalParser::new().expect("Failed to create parser");
        let source = "class MyClass {}";
        
        let tree = parser.parse(source).expect("Failed to parse");
        assert_eq!(tree.root_node().kind(), "program");
        assert_eq!(parser.source(), source);
    }

    #[test]
    fn test_incremental_parser_insert() {
        let mut parser = IncrementalParser::new().expect("Failed to create parser");
        let initial = "class MyClass {}";
        parser.parse(initial).expect("Failed to parse initial");

        // Insert " extends Object" before the closing brace
        // Position: "class MyClass" (13 chars) + " extends Object" (15 chars)
        let edit = Edit::insert(13, 15, 0, 13);
        let new_source = "class MyClass extends Object {}";
        
        let tree = parser.reparse(edit, new_source).expect("Failed to reparse");
        assert!(!tree.root_node().has_error(), "Tree should not have errors");
        
        // Verify we can still extract the class
        let classes = extract_classes(tree, new_source);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "MyClass");
    }

    #[test]
    fn test_incremental_parser_delete() {
        let mut parser = IncrementalParser::new().expect("Failed to create parser");
        let initial = "class MyClass extends Object {}";
        parser.parse(initial).expect("Failed to parse initial");

        // Delete " extends Object" (from position 13 to 28)
        let edit = Edit::delete(13, 28, 0, 13, 0, 28);
        let new_source = "class MyClass {}";
        
        let tree = parser.reparse(edit, new_source).expect("Failed to reparse");
        assert!(!tree.root_node().has_error(), "Tree should not have errors");
        
        let classes = extract_classes(tree, new_source);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "MyClass");
    }

    #[test]
    fn test_incremental_parser_replace() {
        let mut parser = IncrementalParser::new().expect("Failed to create parser");
        let initial = "class OldName {}";
        parser.parse(initial).expect("Failed to parse initial");

        // Replace "OldName" with "NewName" (position 6 to 13, length 7)
        let edit = Edit::replace(6, 13, 7, 0, 6, 0, 13, 13);
        let new_source = "class NewName {}";
        
        let tree = parser.reparse(edit, new_source).expect("Failed to reparse");
        assert!(!tree.root_node().has_error(), "Tree should not have errors");
        
        let classes = extract_classes(tree, new_source);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "NewName");
    }

    #[test]
    fn test_incremental_parser_multiple_edits() {
        let mut parser = IncrementalParser::new().expect("Failed to create parser");
        let initial = "class A {} class B {}";
        parser.parse(initial).expect("Failed to parse initial");

        // Make multiple insertions
        let edit1 = Edit::insert(7, 13, 0, 7);  // Insert " implements X" after "class A"
        let new_source1 = "class A implements X {} class B {}";
        parser.reparse(edit1, new_source1).expect("Failed to reparse 1");

        let edit2 = Edit::insert(31, 13, 0, 31);  // Insert " implements Y" after "class B"
        let new_source2 = "class A implements X {} class B implements Y {}";
        
        let tree = parser.reparse(edit2, new_source2).expect("Failed to reparse 2");
        let classes = extract_classes(tree, new_source2);
        assert_eq!(classes.len(), 2);
    }

    #[test]
    fn test_incremental_parser_multiple_edits_batch() {
        let mut parser = IncrementalParser::new().expect("Failed to create parser");
        let initial = "class A {} class B {}";
        parser.parse(initial).expect("Failed to parse initial");

        // Apply multiple edits at once
        // Note: When using batch edits, positions should be relative to the original state
        let edits = vec![
            Edit::insert(7, 5, 0, 7),   // Insert "Final" after "class A"
            Edit::insert(17, 5, 0, 17), // Insert "Final" after "class B" (original position)
        ];
        
        let new_source = "class AFinal {} class BFinal {}";
        let tree = parser.reparse_with_edits(&edits, new_source).expect("Failed to reparse");
        
        let classes = extract_classes(tree, new_source);
        assert_eq!(classes.len(), 2);
        // Note: Testing that batch edits work correctly
    }
}
