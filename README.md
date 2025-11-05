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

### Run with debug output
```bash
RUST_LOG=debug cargo run -- analyze .
```

## Architecture

The analyzer is structured into several modules:

- **parser**: Dart file discovery and loading
- **analyzer**: Rule trait and analysis coordination
- **rules**: Style and runtime rule implementations
- **config**: Configuration management
- **mcp**: Model Context Protocol server
- **error**: Error types and diagnostic structures

### Current Implementation

The current implementation uses regex-based pattern matching for rule detection. This approach provides:
- **Pros**: Fast, lightweight, easy to add new rules
- **Cons**: Limited to syntactic patterns, can produce false positives

### Future Enhancements

For more accurate analysis, future versions could:
1. Integrate AST (Abstract Syntax Tree) parsing using tree-sitter or analyzer_plugin
2. Add semantic analysis for better import usage detection
3. Support for type inference and flow analysis
4. Integration with Dart Analysis Server for IDE-quality diagnostics
5. Watch mode for continuous analysis
6. Incremental analysis for large projects

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
