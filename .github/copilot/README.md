# GitHub Copilot Configuration

This directory contains configuration for GitHub Copilot to work effectively with this Rust-based Dart analyzer project.

## Files

### `mcp.json`
Configures Model Context Protocol (MCP) servers for GitHub Copilot:
- **rust-analyzer**: Provides Rust language intelligence for code navigation, completion, and analysis
- **dart-analyzer**: Provides Dart language intelligence for understanding Dart code patterns

### `instructions.md`
Contains project-specific guidance for GitHub Copilot to better understand the codebase and development practices.

## Setup

### Prerequisites
1. **Rust toolchain** with rust-analyzer
   ```bash
   rustup component add rust-analyzer
   ```

2. **Dart SDK** (optional, for Dart language support)
   ```bash
   # Install Dart SDK from https://dart.dev/get-dart
   ```

3. **VS Code Extensions** (automatically recommended when opening the project):
   - GitHub Copilot
   - GitHub Copilot Chat
   - rust-analyzer
   - Dart (optional)

### Using GitHub Copilot with this Project

Once you open this project in VS Code with GitHub Copilot installed:

1. The MCP servers will automatically be available to GitHub Copilot
2. Copilot will use rust-analyzer for Rust code intelligence
3. Copilot will reference the project instructions when making suggestions

### MCP Server Benefits

The configured MCP servers provide:
- **Code completion** based on project context
- **Semantic understanding** of Rust and Dart syntax
- **Type information** for better suggestions
- **Navigation** through code definitions and references
- **Diagnostics** integration with the language servers

## Troubleshooting

If MCP servers are not working:
1. Ensure rust-analyzer is installed: `rustup component add rust-analyzer`
2. Restart VS Code
3. Check the Output panel for GitHub Copilot logs
4. Verify that rust-analyzer is in your PATH: `which rust-analyzer`

## Additional Resources
- [GitHub Copilot Documentation](https://docs.github.com/en/copilot)
- [MCP Protocol Specification](https://spec.modelcontextprotocol.io/)
- [rust-analyzer User Manual](https://rust-analyzer.github.io/)
