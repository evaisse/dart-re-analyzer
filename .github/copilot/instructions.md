# GitHub Copilot Instructions for dart-re-analyzer

This project is a high-performance Rust-based Dart/Flutter code analyzer with MCP server support.

## Project Overview
- **Language**: Rust (primary), with knowledge of Dart syntax patterns
- **Purpose**: Analyze Dart/Flutter code for style and runtime issues
- **Architecture**: Modular design with parser, analyzer, rules, and MCP server components
- **Performance Focus**: Parallel processing, low memory footprint, efficient regex/AST-based analysis

## Development Guidelines

### Rust Code Style
- Follow Rust idioms and best practices
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Ensure all tests pass with `cargo test`
- Add tests for new functionality

### Code Organization
- **parser**: File discovery and loading logic (uses `walkdir` for efficient traversal)
- **analyzer**: Core analysis coordination and rule trait definition
- **rules**: Implementation of style and runtime rules (both regex and tree-sitter based)
- **config**: Configuration management (JSON-based with serde)
- **mcp**: Model Context Protocol server implementation (async with tokio)
- **treesitter**: Tree-sitter parsing, queries, and typed wrappers for AST analysis
- **lsp**: Language Server Protocol integration and semantic analysis foundation
- **error**: Error types and diagnostic structures (using thiserror)

## Key Dependencies and Their Purpose
- **tree-sitter + tree-sitter-dart**: AST parsing with error recovery for precise code analysis
- **rayon**: Parallel processing of files for performance on large codebases
- **regex**: Fast pattern matching for simple rule checks
- **tokio**: Async runtime for MCP server
- **serde + serde_json**: Configuration and JSON-RPC message handling
- **clap**: CLI argument parsing
- **walkdir**: Efficient recursive directory traversal
- **lsp-types + lsp-server**: Language Server Protocol implementation
- **anyhow + thiserror**: Error handling (anyhow for application errors, thiserror for library errors)

### When Adding New Rules
1. Implement the `Rule` trait in the appropriate rules module (`rules/style.rs` or `rules/runtime.rs`)
2. Choose the right analysis approach:
   - **Regex**: For simple pattern matching (e.g., naming conventions)
   - **Tree-sitter**: For structural analysis (e.g., AST node types, complex patterns)
   - **Hybrid**: Combine both for optimal performance
3. Add tests in the `tests/` directory with both positive and negative cases
4. Update README.md and `docs/RULES.md` with rule documentation
5. Consider performance impact:
   - Prefer tree-sitter queries over traversing the entire AST
   - Use efficient regex patterns (avoid backtracking)
   - Test with large files to ensure no performance regression

## Common Pitfalls and Gotchas
- **Tree-sitter parsing**: Always handle parse errors gracefully; tree-sitter is error-tolerant
- **Parallel processing**: Rules must be thread-safe (use immutable data or proper synchronization)
- **Regex patterns**: Multiline patterns need `(?m)` flag; be careful with `.` not matching newlines by default
- **File paths**: Always use absolute paths when working with files; config excludes are glob patterns
- **Dart syntax edge cases**: 
  - Generic types with nested `<>` can be complex to parse with regex
  - String interpolation `${}` needs special handling
  - Null-safety syntax (`?`, `!`) requires careful pattern matching
- **MCP server**: JSON-RPC requires exact message format; always validate request/response structure
- **LSP integration**: The dart analysis server expects specific message sequences for initialization

### Dart Code Patterns
This analyzer checks Dart code for:
- **Style issues**: naming conventions, line length, file organization
- **Runtime safety**: dynamic types, null checks, empty catches, unused imports

When working with Dart patterns, refer to test files for examples.

### Dart Syntax Quick Reference for Rule Development
- **Class declaration**: `class ClassName { }`
- **Method declaration**: `void methodName() { }`  
- **Field declaration**: `final String fieldName;`
- **Dynamic type**: `dynamic variable;` or `var x;` (without type)
- **Null safety**: `String?` (nullable), `String!` (null assertion)
- **Imports**: `import 'package:name/file.dart';`
- **Private members**: Prefix with underscore `_privateMember`
- **Catch blocks**: `try { } catch (e) { }`

## Example Workflows

### Adding a New Style Rule
```bash
# 1. Create the rule in src/rules/style.rs
# 2. Implement the Rule trait with check() method
# 3. Add to get_style_rules() function
# 4. Create test in tests/style_rules_test.rs
# 5. Run tests
cargo test
# 6. Test on real project
cargo run -- analyze test_project/
```

### Debugging a Rule
```bash
# Run with debug logging
RUST_LOG=debug cargo run -- analyze <path>
# Run specific test
cargo test <test_name> -- --nocapture
# Use the tree-sitter demo to inspect AST
cargo run --example treesitter_demo
```

### Performance Testing
```bash
# Build release version
cargo build --release
# Test on large project
time ./target/release/dart-re-analyzer analyze <large-flutter-project>
# Compare with parallel vs sequential
# Check analyzer_config.json: "parallel": true/false
```

## Testing
- Run `cargo test` to execute all tests
- Tests use tempfile for creating test Dart files
- Each rule should have positive and negative test cases
- Integration tests in `test_analyzer.sh` run the analyzer on `test_project/`

### Test Organization
- **Unit tests**: In-module tests for individual functions
- **Integration tests**: In `tests/` directory for end-to-end scenarios
- **Rule tests**: Verify both detection (true positives) and non-detection (false positives)

## Build and Run
- Build: `cargo build --release`
- Test: `cargo test`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Format: `cargo fmt`
- Run analyzer: `cargo run -- analyze <path>`
- Start MCP server: `cargo run -- serve --port 9000 <path>`
- Run LSP proxy: `cargo run -- language-server <path>`

## Performance Considerations
- **Parallel processing**: Enabled by default via rayon; scales with CPU cores
- **Memory efficiency**: Analyzer processes files in batches, not all at once
- **Regex optimization**: Compiled regex patterns are cached per rule
- **Tree-sitter efficiency**: Uses zero-copy parsing; incremental parsing available
- **Expected performance**: < 2 seconds for 1000+ files, < 100MB memory usage

## Troubleshooting

### Build Issues
- **tree-sitter compilation**: Requires C compiler (gcc/clang); on Windows, use MSVC
- **Missing dependencies**: Run `cargo clean && cargo build` to force rebuild
- **Version conflicts**: Check Cargo.lock is committed and up to date

### Runtime Issues
- **No files found**: Check exclude_patterns in config don't accidentally exclude target files
- **Slow analysis**: Ensure `parallel: true` in config; check if running in debug mode
- **False positives**: Regex rules may need refinement; consider using tree-sitter for complex patterns
- **LSP connection**: Verify Dart SDK is in PATH; check dart language-server works standalone

### Development Tips
- Use `cargo check` for fast compilation feedback during development
- Use `cargo clippy` to catch common mistakes before committing
- Run `cargo fmt` before committing to maintain consistent style
- Use `RUST_LOG=debug` for detailed logging when debugging issues
- Test with both small and large Dart projects to verify scalability

## Language Server Configuration
- **Rust**: rust-analyzer is configured for this workspace
- **Dart**: Dart language server is available for understanding Dart patterns
