# Tree-sitter Integration Guide

## Overview

The dart-re-analyzer now includes full Tree-sitter integration for complete Dart syntax tree analysis. This provides:

- **Complete tokenization** with precise byte offsets and line/column positions
- **Error-tolerant parsing** that works with incomplete or invalid code
- **Full concrete syntax tree (CST)** access
- **Zero-copy parsing** for excellent performance
- **Typed wrappers** for common Dart constructs (classes, methods, imports)

## What is Tree-sitter?

Tree-sitter is a fast, incremental parsing library that provides:
- Robust parsing with excellent error recovery
- Full fidelity concrete syntax tree (preserves all whitespace, comments, etc.)
- Fast incremental re-parsing for IDE-like workflows
- Battle-tested grammars including a mature Dart grammar

## Quick Start

### Parsing Dart Code

```rust
use dart_re_analyzer::treesitter::parse_dart;

let source = r#"
class MyClass {
    int x = 1;
    void myMethod() => print(x);
}
"#;

let tree = parse_dart(source)?;
let root = tree.root_node();
println!("Root kind: {}", root.kind()); // "program"
```

### Extracting Tokens

Tree-sitter doesn't expose a separate token stream, but we provide token extraction by walking leaf nodes:

```rust
use dart_re_analyzer::treesitter::extract_tokens;

let tokens = extract_tokens(&tree, source);
for token in tokens {
    println!("{:?} at line {} col {}: {:?}", 
             token.kind, 
             token.start_point.row, 
             token.start_point.column,
             token.text);
}
```

### Extracting Classes

```rust
use dart_re_analyzer::treesitter::extract_classes;

let classes = extract_classes(&tree, source);
for class in classes {
    println!("Class: {} at bytes [{}..{}]", 
             class.name,
             class.start_byte,
             class.end_byte);
}
```

### Extracting Imports

```rust
use dart_re_analyzer::treesitter::extract_imports;

let imports = extract_imports(&tree, source);
for import in imports {
    println!("Import: {}", import.uri);
}
```

## Architecture

The Tree-sitter integration is organized as follows:

```
src/treesitter/mod.rs
├── Core Functions
│   ├── create_dart_parser()  - Initialize parser with Dart grammar
│   └── parse_dart()          - Parse source to CST
│
├── Token Extraction
│   ├── Token struct          - Represents a single token with position info
│   ├── Point struct          - Line/column position
│   └── extract_tokens()      - Walk tree and extract all leaf nodes
│
├── Typed Wrappers
│   ├── DartClass            - Class declaration wrapper
│   ├── DartMethod           - Method/function wrapper
│   ├── DartImport           - Import statement wrapper
│   ├── extract_classes()    - Find all class declarations
│   ├── extract_methods()    - Find all method declarations
│   └── extract_imports()    - Find all import statements
│
└── Debugging
    └── print_tree()         - Print entire CST structure
```

## Use Cases

### 1. Structural Analysis

Extract high-level program structure:

```rust
let tree = parse_dart(source)?;
let classes = extract_classes(&tree, source);
let methods = extract_methods(&tree, source);
let imports = extract_imports(&tree, source);

println!("Found {} classes, {} methods, {} imports", 
         classes.len(), methods.len(), imports.len());
```

### 2. Token-Level Processing

Process individual tokens with precise position information:

```rust
let tokens = extract_tokens(&tree, source);
let keywords = tokens.iter()
    .filter(|t| matches!(t.text.as_str(), "class" | "void" | "import"))
    .count();
```

### 3. Error Recovery

Tree-sitter parses even incomplete code, marking error nodes:

```rust
let incomplete = "class { incomplete";
let tree = parse_dart(incomplete)?;

if tree.root_node().has_error() {
    println!("Code has syntax errors but was still parsed");
}
```

### 4. Code Intelligence

Build code intelligence features like:
- Symbol extraction
- Code navigation
- Syntax highlighting
- Code folding
- Outline view

## Performance Characteristics

Tree-sitter is highly optimized:

- **Zero-copy parsing**: Uses byte slices from the original source
- **Incremental re-parsing**: Can update trees with edits (not yet exposed)
- **Thread-safe**: Parser can be cloned and used across threads
- **Memory efficient**: Tree nodes are lightweight references

Typical performance:
- Small files (<100KB): <1ms
- Medium files (100KB-1MB): 1-10ms
- Large files (>1MB): 10-100ms

## Comparison: Tree-sitter vs Regex Rules

| Feature | Tree-sitter | Regex Rules |
|---------|-------------|-------------|
| Speed | Very Fast (~1ms) | Faster (sub-ms) |
| Accuracy | High (AST-based) | Medium (pattern-based) |
| Context Awareness | Full | Limited |
| Error Recovery | Excellent | None |
| Semantic Analysis | No (syntax only) | No |
| Memory Usage | ~100KB per file | Minimal |
| False Positives | Very Low | Medium |

**Recommendation**: Use Tree-sitter for structural analysis and complex rules. Keep regex rules for simple, fast checks.

## Integration with Existing Rules

The Tree-sitter module can be used alongside existing regex-based rules:

```rust
// Example: Enhanced class name checker using Tree-sitter
use dart_re_analyzer::treesitter::{parse_dart, extract_classes};

fn check_class_names_precise(source: &str) -> Result<Vec<Diagnostic>> {
    let tree = parse_dart(source)?;
    let classes = extract_classes(&tree, source);
    
    let mut diagnostics = Vec::new();
    for class in classes {
        if !class.name.chars().next().unwrap().is_uppercase() {
            diagnostics.push(Diagnostic {
                message: format!("Class '{}' should start with uppercase", class.name),
                // ... precise location from Tree-sitter
            });
        }
    }
    Ok(diagnostics)
}
```

## Advanced Usage

### Walking the Tree Manually

For custom analysis, walk the tree directly:

```rust
use tree_sitter::TreeCursor;

fn find_all_nodes_of_kind<'a>(tree: &'a Tree, kind: &str) -> Vec<tree_sitter::Node<'a>> {
    let mut nodes = Vec::new();
    let mut cursor = tree.walk();
    
    fn walk<'a>(cursor: &mut TreeCursor<'a>, kind: &str, nodes: &mut Vec<tree_sitter::Node<'a>>) {
        if cursor.node().kind() == kind {
            nodes.push(cursor.node());
        }
        
        if cursor.goto_first_child() {
            walk(cursor, kind, nodes);
            while cursor.goto_next_sibling() {
                walk(cursor, kind, nodes);
            }
            cursor.goto_parent();
        }
    }
    
    walk(&mut cursor, kind, &mut nodes);
    nodes
}
```

### Understanding Node Kinds

Common Dart node kinds:
- `program` - Root node
- `class_definition` - Class declaration
- `method_signature` - Method declaration
- `function_signature` - Top-level function
- `import_or_export` - Import/export statement
- `identifier` - Variable/function names
- `type_identifier` - Type names
- `comment` - Comments (doc comments, line comments, block comments)
- `string_literal` - String literals
- `ERROR` - Parse error node

To see all node kinds in a file, use `print_tree()`.

## Future Enhancements

Potential improvements for future versions:

1. **Incremental Parsing**: Expose edit/re-parse API for IDE workflows
2. **Tree-sitter Queries**: Use declarative query language for pattern matching
3. **Semantic Layer**: Correlate with Dart Analysis Server for type information
4. **More Typed Wrappers**: Add wrappers for fields, variables, expressions, etc.
5. **LSP Integration**: Use positions for LSP textDocument/definition, etc.

## Examples

See `examples/treesitter_demo.rs` for a complete working example that demonstrates:
- Parsing a Flutter widget
- Extracting classes, imports, and tokens
- Token statistics and display
- Error handling

Run it with:
```bash
cargo run --example treesitter_demo
```

## Testing

The Tree-sitter module includes comprehensive tests:

```bash
# Run only tree-sitter tests
cargo test treesitter

# Run all tests
cargo test
```

Current test coverage:
- Basic parsing (valid and invalid code)
- Token extraction with position verification
- Class extraction with multiple classes
- Import extraction
- Error recovery with incomplete code

## Resources

- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Tree-sitter Dart Grammar](https://github.com/UserNobody14/tree-sitter-dart)
- [Tree-sitter Rust Bindings](https://docs.rs/tree-sitter/)

## Limitations

Current limitations to be aware of:

1. **Syntax Only**: Tree-sitter provides syntax trees, not semantic analysis
2. **No Type Information**: Cannot determine types without Dart Analyzer integration
3. **Grammar Coverage**: Some newer Dart language features may not be in grammar yet
4. **Node Kind Discovery**: Need to inspect trees to find the right node kinds for your use case

For semantic analysis (types, resolution, null-safety), consider integrating the Dart Analysis Server via LSP as described in the original issue.
