---
layout: default
title: Home
---

# Dart Re-Analyzer

A high-performance Rust-based Dart/Flutter analyzer focused on code quality, conventions, and runtime safety. Built with speed, parallel processing, and memory efficiency in mind for large codebases.

## 🎯 Key Features

### Analysis Rules
- **[Style Rules](rules#style-rules)**: Code conventions, naming patterns, and file organization
- **[Runtime Rules](rules#runtime-rules)**: Prevent runtime errors and unsafe code patterns

### Integration Options
- **[MCP Server](mcp)**: Model Context Protocol server for programmatic access
- **[LSP Integration](lsp)**: Language Server Protocol for IDE integration
- **[LSP Proxy](lsp-proxy)**: Transparent proxy for Dart Analysis Server

### Performance
- ⚡ Parallel processing using Rayon
- 🚀 Fast file scanning
- 💾 Low memory footprint
- 📦 Configurable execution modes

## Quick Start

```bash
# Install from source
git clone https://github.com/evaisse/dart-re-analyzer.git
cd dart-re-analyzer
cargo build --release

# Analyze a project
dart-re-analyzer analyze /path/to/project

# Generate configuration
dart-re-analyzer init-config
```

[Read the Quick Start Guide →](quickstart)

## Available Rules

### Style Rules (4)
- `camel_case_class_names` - Ensures class names use CamelCase
- `snake_case_file_names` - Ensures file names use snake_case
- `private_field_underscore` - Checks private fields start with underscore
- `line_length` - Enforces maximum line length

### Runtime Rules (5)
- `avoid_dynamic` - Detects usage of `dynamic` type
- `avoid_empty_catch` - Catches empty catch blocks
- `unused_import` - Identifies unused imports
- `avoid_print` - Warns about print statements
- `avoid_null_check_on_nullable` - Detects unsafe null assertion operators

[View All Rules Documentation →](rules)

## Integration Examples

### Command Line
```bash
# Run only style rules
dart-re-analyzer analyze . --style-only

# Get JSON output
dart-re-analyzer analyze . --format json

# Use custom config
dart-re-analyzer analyze . --config my_config.json
```

### MCP Server
```bash
# Start the server
dart-re-analyzer serve --port 9000 /path/to/project

# Query from another process
echo '{"method": "get_stats", "params": {}}' | nc localhost 9000
```

[Learn More About MCP →](mcp)

### LSP Proxy
```bash
# Use as Dart Analysis Server proxy
dart-re-analyzer language-server /path/to/project
```

Configure your IDE to use dart-re-analyzer as the analysis server to get diagnostics directly in your editor.

[Learn More About LSP →](lsp)

## Example Output

```
Issues found:

./lib/my_file.dart:
  ⚠ [4:7] style (camel_case_class_names): Class name 'myClass' should use CamelCase
    💡 Rename to 'MyClass'
  ✗ [15:7] runtime (avoid_empty_catch): Empty catch block swallows exceptions
    💡 Handle the exception or at least log it
  ⚠ [6:3] runtime (avoid_dynamic): Avoid using 'dynamic' type
    💡 Use a specific type or Object? instead

Summary:
  1 errors, 2 warnings, 0 info messages
```

## Performance Metrics

For a typical Flutter project with 1000+ files:
- Analysis time: < 2 seconds
- Memory usage: < 100MB
- Scales linearly with project size

## Documentation

- [Quick Start Guide](quickstart) - Get started in minutes
- [Rules Reference](rules) - Complete rule documentation
- [MCP Server Guide](mcp) - Programmatic access
- [LSP Integration](lsp) - IDE integration guide
- [LSP Proxy Guide](lsp-proxy) - Transparent proxy setup

## Contributing

Contributions are welcome! Areas for improvement:
- Additional style rules
- More runtime safety checks
- AST-based analysis improvements
- IDE integrations
- Watch mode for continuous analysis

## License

MIT

## Author

Emmanuel Vaisse

[View on GitHub](https://github.com/evaisse/dart-re-analyzer)
