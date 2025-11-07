# dart-re-analyzer

A high-performance Rust-based Dart/Flutter analyzer focused on code quality, conventions, and runtime safety. Built with speed, parallel processing, and memory efficiency in mind for large codebases.

## Features

### 🎯 Two Rule Categories

#### Style Rules
Focus on code conventions, naming, and file organization:
- **camel_case_class_names**: Ensures class names use CamelCase
- **snake_case_file_names**: Ensures file names use snake_case
- **private_field_underscore**: Checks private fields start with underscore
- **line_length**: Enforces maximum line length (default: 120 characters)

#### Runtime Rules
Focus on avoiding runtime errors and unsafe code patterns:
- **avoid_dynamic**: Detects usage of `dynamic` type that bypasses type safety
- **avoid_empty_catch**: Catches empty catch blocks that swallow exceptions
- **unused_import**: Identifies unused imports
- **avoid_print**: Warns about print statements in production code
- **avoid_null_check_on_nullable**: Detects unsafe null assertion operators

### 🚀 Performance Features
- **Parallel processing** using Rayon for efficient multi-core utilization
- **Fast file scanning** with optimized Dart file discovery
- **Low memory footprint** compared to traditional analyzers
- Configurable parallel/sequential execution modes

### 🌐 MCP Server
Built-in Model Context Protocol (MCP) server for programmatic access to analyzer results:
- Query all errors or filter by category, severity, or file
- Get statistics about code issues
- JSON-RPC interface for easy integration

### 🔌 LSP Proxy
Use dart-re-analyzer as a transparent proxy for the Dart Analysis Server:
- Inject dart-re-analyzer diagnostics directly into your IDE
- Works with VS Code, IntelliJ, Neovim, Emacs, and more
- No IDE configuration changes needed
- Combines dart-re-analyzer rules with Dart Analysis Server's semantic analysis

## Installation

### From Source
```bash
git clone https://github.com/evaisse/dart-re-analyzer.git
cd dart-re-analyzer
cargo build --release
```

The binary will be available at `target/release/dart-re-analyzer`

## Usage

### Basic Analysis
Analyze a Dart/Flutter project:
```bash
dart-re-analyzer analyze /path/to/project
```

Analyze current directory:
```bash
dart-re-analyzer analyze .
```

### Filtered Analysis
Run only style rules:
```bash
dart-re-analyzer analyze . --style-only
```

Run only runtime rules:
```bash
dart-re-analyzer analyze . --runtime-only
```

### Output Formats
Get results as JSON:
```bash
dart-re-analyzer analyze . --format json
```

Human-readable output (default):
```bash
dart-re-analyzer analyze . --format text
```

### Configuration
Generate a default configuration file:
```bash
dart-re-analyzer init-config
```

This creates `analyzer_config.json` with default settings:
```json
{
  "enabled": true,
  "exclude_patterns": [
    ".dart_tool/**",
    "build/**",
    ".pub/**",
    "packages/**"
  ],
  "style_rules": {
    "enabled": true,
    "disabled_rules": []
  },
  "runtime_rules": {
    "enabled": true,
    "disabled_rules": []
  },
  "max_line_length": 120,
  "parallel": true
}
```

Use a custom configuration:
```bash
dart-re-analyzer analyze . --config my_config.json
```

### MCP Server
Start the MCP server for programmatic access:
```bash
dart-re-analyzer serve --port 9000 /path/to/project
```

### LSP Proxy
Use as a Dart Analysis Server proxy in your IDE:
```bash
dart-re-analyzer language-server /path/to/project
```

This allows you to get dart-re-analyzer diagnostics directly in your IDE alongside the Dart Analysis Server's diagnostics. See [LSP Proxy Guide](docs/LSP_PROXY.md) for IDE-specific setup instructions.

#### MCP API Examples

Get all errors:
```json
{"method": "get_all_errors", "params": {}}
```

Get filtered errors:
```json
{
  "method": "get_errors", 
  "params": {
    "category": "runtime",
    "severity": "error",
    "file": "main.dart"
  }
}
```

Get statistics:
```json
{"method": "get_stats", "params": {}}
```

Response format:
```json
{
  "success": true,
  "data": {
    "total": 10,
    "errors": 1,
    "warnings": 7,
    "info": 2,
    "style_issues": 3,
    "runtime_issues": 7,
    "files_with_issues": 2
  }
}
```

## Tree-sitter AST Analysis

The analyzer now includes full Tree-sitter integration for precise syntax tree analysis:

```rust
use dart_re_analyzer::treesitter::{parse_dart, extract_classes, extract_tokens};

// Parse Dart code into a full syntax tree
let tree = parse_dart(source)?;

// Extract all classes with precise locations
let classes = extract_classes(&tree, source);
for class in classes {
    println!("Class: {} at bytes [{}..{}]", 
             class.name, class.start_byte, class.end_byte);
}

// Extract all tokens for detailed analysis
let tokens = extract_tokens(&tree, source);
println!("Total tokens: {}", tokens.len());
```

**Features:**
- Complete tokenization with line/column positions
- Error-tolerant parsing (works with incomplete code)
- Zero-copy parsing for performance
- Typed wrappers for classes, methods, imports
- Full concrete syntax tree access

**See the [Tree-sitter Guide](docs/TREESITTER.md) for comprehensive documentation.**

**Run the demo:**
```bash
cargo run --example treesitter_demo
```

## Advanced Capabilities

### 🔍 Tree-sitter Queries

Use declarative pattern matching for complex analysis:

```rust
use dart_re_analyzer::treesitter::{parse_dart, query_tree, queries};

let tree = parse_dart(source)?;

// Find all classes
let matches = query_tree(&tree, source, queries::CLASSES)?;

// Find dynamic type usage
let dynamics = query_tree(&tree, source, queries::DYNAMIC_TYPES)?;

// Custom query: find all string literals
let query = r#"(string_literal) @string"#;
let strings = query_tree(&tree, source, query)?;
```

**Pre-defined queries:**
- `CLASSES`, `METHODS`, `FIELDS`, `IMPORTS`
- `DYNAMIC_TYPES`, `PRINT_CALLS`, `EMPTY_CATCH`
- `NULL_ASSERTIONS`, `TYPED_VARIABLES`, `TYPE_PARAMETERS`

### ⚡ Incremental Parsing

Efficiently re-parse code after edits:

```rust
use dart_re_analyzer::treesitter::{IncrementalParser, Edit};

let mut parser = IncrementalParser::new()?;
parser.parse("class MyClass {}")?;

// Insert text without full re-parse
let edit = Edit::insert(13, 15, 0, 13);
parser.reparse(edit, "class MyClass extends Object {}")?;
```

**Benefits:**
- 10-100x faster than full re-parse for small changes
- Perfect for watch mode and IDE integration
- Maintains parse tree state across edits

### 🔬 Semantic Analysis (LSP)

Foundation for IDE-quality analysis:

```rust
use dart_re_analyzer::lsp::{MockSemanticAnalyzer, SemanticAnalyzer};

let analyzer = MockSemanticAnalyzer::new();

// Resolve types
let type_info = analyzer.resolve_type(&file, line, col)?;

// Get diagnostics
let diagnostics = analyzer.get_diagnostics(&file)?;

// Find definitions and references
let definition = analyzer.find_definition(&file, line, col)?;
let references = analyzer.find_references(&file, line, col)?;
```

**Capabilities:**
- Type resolution and inference
- Null-safety flow analysis  
- Cross-file symbol resolution
- IDE-quality diagnostics

**See the [LSP Integration Guide](docs/LSP.md) for details.**

### 📦 Extended Typed Wrappers

Extract rich structural information:

```rust
use dart_re_analyzer::treesitter::{
    extract_fields, extract_variables, extract_type_annotations,
    extract_type_parameters, extract_expressions
};

// Get all fields with modifiers
let fields = extract_fields(&tree, source);
for field in fields {
    println!("{}: static={}, final={}, const={}", 
        field.name, field.is_static, field.is_final, field.is_const);
}

// Get type information
let types = extract_type_annotations(&tree, source);
for ty in types {
    println!("Type: {}{}", ty.type_name, if ty.is_nullable { "?" } else { "" });
}
```

## Example Output

```
Issues found:

/path/to/project/lib/my_file.dart:
  ⚠ [4:7] style (camel_case_class_names): Class name 'myClass' should use CamelCase (start with uppercase)
    💡 Rename to 'MyClass'
  ✗ [15:7] runtime (avoid_empty_catch): Empty catch block swallows exceptions silently
    💡 Handle the exception or at least log it
  ⚠ [6:3] runtime (avoid_dynamic): Avoid using 'dynamic' type as it bypasses type safety
    💡 Use a specific type or Object? instead

Summary:
  1 errors, 2 warnings, 0 info messages
```

## Configuration Options

### exclude_patterns
List of glob patterns to exclude from analysis:
```json
"exclude_patterns": [
  ".dart_tool/**",
  "build/**",
  "test/**/*.g.dart"
]
```

### Disable specific rules
```json
"style_rules": {
  "enabled": true,
  "disabled_rules": ["line_length"]
},
"runtime_rules": {
  "enabled": true,
  "disabled_rules": ["avoid_print"]
}
```

### Adjust line length
```json
"max_line_length": 100
```

### Control parallel processing
```json
"parallel": true
```

## Development

### Build
```bash
cargo build
```

### Run tests
```bash
cargo test
```

### Run integration tests
Test dart-re-analyzer on a real Dart project:
```bash
./test_analyzer.sh
```

This script runs the analyzer on `test_project/` which contains Dart code samples designed to test all analyzer rules and features.

### Run with debug output
```bash
RUST_LOG=debug cargo run -- analyze .
```

### GitHub Copilot Integration

This project is configured to work with GitHub Copilot and includes MCP (Model Context Protocol) server configurations for enhanced AI assistance.

**Configured Language Servers:**
- **rust-analyzer**: Provides Rust language intelligence
- **dart-analyzer**: Provides Dart language support for understanding patterns

**Setup:**
1. Open this project in VS Code
2. Install recommended extensions (GitHub Copilot, rust-analyzer, etc.)
3. The MCP servers will automatically enhance Copilot's understanding of the codebase

See [`.github/copilot/README.md`](.github/copilot/README.md) for detailed configuration information.

## Architecture

The analyzer is structured into several modules:

- **parser**: Dart file discovery and loading
- **analyzer**: Rule trait and analysis coordination
- **rules**: Style and runtime rule implementations
- **config**: Configuration management
- **mcp**: Model Context Protocol server
- **treesitter**: Tree-sitter parsing and queries
- **lsp**: Language Server Protocol integration
- **error**: Error types and diagnostic structures

### Current Implementation

The analyzer includes multiple complementary analysis approaches:

#### 1. Tree-sitter AST Analysis
- **Full concrete syntax tree** with complete tokenization
- **Error-tolerant parsing** that works with incomplete code
- **Declarative queries** for pattern matching
- **Incremental parsing** for efficient re-analysis
- **Typed wrappers** for classes, methods, fields, variables, types, expressions
- **See [Tree-sitter Guide](docs/TREESITTER.md)** for detailed usage

#### 2. LSP Semantic Analysis (Foundation)
- **Type resolution** and inference interface
- **Symbol information** structures
- **Semantic diagnostics** with fixes
- **Cross-file navigation** support
- **See [LSP Integration Guide](docs/LSP.md)** for detailed usage

#### 3. Regex-based Pattern Matching
- **Fast and lightweight** for simple pattern detection
- **Easy to add new rules** without complex parsing
- **Good for straightforward checks** like naming conventions
- **Lower memory overhead** compared to full parsing

**Hybrid Strategy**: Use the right tool for each job - Tree-sitter for structure, LSP for semantics, regex for simple patterns.

### Recent Improvements (v0.2)

✅ **Tree-sitter Queries** - Declarative pattern language for maintainable rules
✅ **Incremental Parsing** - Fast re-parse on edits for watch mode
✅ **Extended Typed Wrappers** - Fields, variables, expressions, type annotations, generics
✅ **LSP Foundation** - Architecture for semantic analysis integration

### Future Enhancements

Potential improvements for future versions:
1. ✅ ~~Integrate AST parsing using tree-sitter~~ **DONE!**
2. ✅ ~~Incremental parsing API~~ **DONE!**
3. ✅ ~~Tree-sitter queries~~ **DONE!**
4. ✅ ~~More typed wrappers~~ **DONE!**
5. 🚧 Full Dart Analysis Server LSP integration (process management, JSON-RPC)
6. 🔜 Watch mode for continuous analysis using incremental parsing
7. 🔜 Rule implementations using tree-sitter queries


## Performance

The analyzer is designed for large codebases:
- Uses parallel processing via Rayon
- Efficient regex-based pattern matching
- Minimal memory allocations
- Fast file system traversal

For a typical Flutter project with 1000+ files, expect:
- Analysis time: < 2 seconds
- Memory usage: < 100MB
- Scales linearly with project size

## Contributing

Contributions are welcome! Areas for improvement:
- Additional style rules
- More runtime safety checks
- AST-based analysis (replacing regex patterns)
- IDE integrations
- Watch mode for continuous analysis

## License

MIT

## Author

Emmanuel Vaisse
