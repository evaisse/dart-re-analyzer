# GitHub Copilot Instructions for dart-re-analyzer

This project is a high-performance Rust-based Dart/Flutter code analyzer with MCP server support.

## Project Overview
- **Language**: Rust (primary), with knowledge of Dart syntax patterns
- **Purpose**: Analyze Dart/Flutter code for style and runtime issues
- **Architecture**: Modular design with parser, analyzer, rules, and MCP server components

## Development Guidelines

### Rust Code Style
- Follow Rust idioms and best practices
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Ensure all tests pass with `cargo test`
- Add tests for new functionality

### Code Organization
- **parser**: File discovery and loading logic
- **analyzer**: Core analysis coordination and rule trait
- **rules**: Implementation of style and runtime rules
- **config**: Configuration management
- **mcp**: Model Context Protocol server implementation
- **error**: Error types and diagnostic structures

### When Adding New Rules
1. Implement the `Rule` trait in the appropriate rules module
2. Add tests in the `tests/` directory
3. Update README.md with rule documentation
4. Consider performance impact (use efficient regex patterns)

### Dart Code Patterns
This analyzer checks Dart code for:
- **Style issues**: naming conventions, line length, file organization
- **Runtime safety**: dynamic types, null checks, empty catches, unused imports

When working with Dart patterns, refer to test files for examples.

## Testing
- Run `cargo test` to execute all tests
- Tests use tempfile for creating test Dart files
- Each rule should have positive and negative test cases

## Build and Run
- Build: `cargo build --release`
- Test: `cargo test`
- Run analyzer: `cargo run -- analyze <path>`
- Start MCP server: `cargo run -- serve --port 9000 <path>`

## Language Server Configuration
- **Rust**: rust-analyzer is configured for this workspace
- **Dart**: Dart language server is available for understanding Dart patterns
