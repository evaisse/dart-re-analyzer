---
layout: default
title: Quick Start
---

# Quick Start Guide

Get started with dart-re-analyzer in minutes!

## Table of Contents
- [Installation](#installation)
- [Quick Examples](#quick-examples)
- [Common Use Cases](#common-use-cases)
- [Understanding Output](#understanding-output)
- [Next Steps](#next-steps)

---

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/evaisse/dart-re-analyzer.git
cd dart-re-analyzer
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be at `target/release/dart-re-analyzer`

### Optional: Add to PATH

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:/path/to/dart-re-analyzer/target/release"
```

---

## Quick Examples

### 1. Analyze Your First Project

```bash
# Navigate to your Dart/Flutter project
cd /path/to/your/flutter/project

# Run the analyzer
dart-re-analyzer analyze .
```

**Example Output:**
```
Analyzing Dart files in: .
Found 42 Dart files
Running 9 rules
Analysis complete. Found 15 issues

Issues found:

lib/main.dart:
  ⚠ [10:5] runtime (avoid_dynamic): Avoid using 'dynamic' type
    💡 Use a specific type or Object? instead
  
  ✗ [25:7] runtime (avoid_empty_catch): Empty catch block swallows exceptions
    💡 Handle the exception or at least log it

Summary:
  2 errors, 10 warnings, 3 info messages
```

### 2. Generate Configuration

Create a configuration file to customize the analyzer:

```bash
dart-re-analyzer init-config
```

This creates `analyzer_config.json`. Edit it to disable specific rules:

```json
{
  "enabled": true,
  "exclude_patterns": [
    ".dart_tool/**",
    "build/**",
    "**/*.g.dart"
  ],
  "style_rules": {
    "enabled": true,
    "disabled_rules": ["line_length"]
  },
  "runtime_rules": {
    "enabled": true,
    "disabled_rules": []
  },
  "max_line_length": 100,
  "parallel": true
}
```

### 3. Get JSON Output

For CI/CD integration or programmatic processing:

```bash
dart-re-analyzer analyze . --format json > results.json
```

**JSON Output Example:**
```json
{
  "diagnostics": [
    {
      "rule_id": "avoid_dynamic",
      "message": "Avoid using 'dynamic' type",
      "severity": "Warning",
      "category": "Runtime",
      "location": {
        "file": "lib/main.dart",
        "line": 10,
        "column": 5
      }
    }
  ],
  "summary": {
    "total": 15,
    "errors": 2,
    "warnings": 10,
    "info": 3
  }
}
```

### 4. Filter by Category

Run only style checks:
```bash
dart-re-analyzer analyze . --style-only
```

Run only runtime checks:
```bash
dart-re-analyzer analyze . --runtime-only
```

### 5. Start MCP Server

For continuous monitoring or IDE integration:

```bash
dart-re-analyzer serve --port 9000 .
```

Then query it from another terminal:
```bash
echo '{"method": "get_stats", "params": {}}' | nc localhost 9000
```

---

## Common Use Cases

### CI/CD Integration

Add to your GitHub Actions workflow:

```yaml
name: Dart Analysis
on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dart-re-analyzer
        run: |
          git clone https://github.com/evaisse/dart-re-analyzer.git
          cd dart-re-analyzer
          cargo build --release
          sudo cp target/release/dart-re-analyzer /usr/local/bin/
      
      - name: Run Analysis
        run: |
          dart-re-analyzer analyze . --format json
          if [ $? -ne 0 ]; then
            echo "Analysis found errors!"
            exit 1
          fi
```

### Pre-commit Hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
dart-re-analyzer analyze . --runtime-only
if [ $? -ne 0 ]; then
  echo "Analysis failed. Fix errors before committing."
  exit 1
fi
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

### VS Code Integration

#### Option 1: Task Runner

Create a task in `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Dart Re-Analyzer",
      "type": "shell",
      "command": "dart-re-analyzer",
      "args": ["analyze", "${workspaceFolder}"],
      "problemMatcher": [],
      "group": {
        "kind": "build",
        "isDefault": false
      }
    }
  ]
}
```

#### Option 2: LSP Proxy

Use dart-re-analyzer as a proxy for the Dart Analysis Server. See the [LSP Proxy Guide](lsp-proxy) for details.

---

## Understanding Output

### Severity Levels

- **✗ Error**: Must be fixed. Causes exit code 1.
- **⚠ Warning**: Should be fixed. Important issues.
- **ℹ Info**: Consider fixing. Best practice suggestions.

### Categories

- **style**: Code conventions and formatting
  - `camel_case_class_names`
  - `snake_case_file_names`
  - `private_field_underscore`
  - `line_length`

- **runtime**: Potential runtime errors and unsafe patterns
  - `avoid_dynamic`
  - `avoid_empty_catch`
  - `unused_import`
  - `avoid_print`
  - `avoid_null_check_on_nullable`

[View Complete Rules Documentation →](rules)

### Exit Codes

- `0`: No errors found (warnings and info may be present)
- `1`: One or more errors found

---

## Performance Tips

### 1. Exclude Generated Files

Add to your configuration:
```json
{
  "exclude_patterns": [
    "**/*.g.dart",
    "**/*.freezed.dart",
    ".dart_tool/**",
    "build/**"
  ]
}
```

### 2. Use Parallel Processing

Enabled by default, but you can explicitly set it:
```json
{
  "parallel": true
}
```

### 3. Run Targeted Checks

```bash
# Only check runtime rules (faster)
dart-re-analyzer analyze . --runtime-only

# Only check specific directories
dart-re-analyzer analyze lib/
```

### 4. Use MCP Server for Repeated Queries

Instead of re-analyzing:
```bash
# Start server once
dart-re-analyzer serve --port 9000 . &

# Query multiple times
echo '{"method": "get_stats", "params": {}}' | nc localhost 9000
echo '{"method": "get_errors", "params": {"category": "runtime"}}' | nc localhost 9000
```

---

## Example Project Analysis

Here's what a typical analysis session looks like:

```bash
$ dart-re-analyzer analyze examples/flutter_app

Analyzing Dart files in: examples/flutter_app
Found 156 Dart files
Running 9 rules
Analysis complete. Found 23 issues

Issues found:

lib/models/user.dart:
  ⚠ [15:3] runtime (avoid_dynamic): Avoid using 'dynamic' type
    💡 Use a specific type or Object? instead

lib/services/api_service.dart:
  ✗ [42:5] runtime (avoid_empty_catch): Empty catch block swallows exceptions
    💡 Handle the exception or at least log it
  ⚠ [105:7] runtime (avoid_print): Avoid using 'print' in production code
    💡 Use a proper logging library

lib/screens/HomeScreen.dart:
  ⚠ [1:1] style (snake_case_file_names): File name 'HomeScreen' should use snake_case
    💡 Rename file to 'home_screen.dart'

Summary:
  1 errors, 18 warnings, 4 info messages

$ echo $?
1  # Non-zero exit code due to errors
```

---

## Next Steps

1. **[Read the Rules Reference](rules)** - Understand all available rules
2. **[Try the MCP Server](mcp)** - Set up programmatic access
3. **[Configure LSP Proxy](lsp-proxy)** - Get IDE integration
4. **Customize your configuration** - Tailor rules to your team's needs
5. **Set up CI/CD integration** - Automate quality checks

---

## Getting Help

- 📚 Check documentation in the [`docs/` directory](https://github.com/evaisse/dart-re-analyzer/tree/main/docs)
- 🔧 Review example configuration in [`analyzer_config.example.json`](https://github.com/evaisse/dart-re-analyzer/blob/main/analyzer_config.example.json)
- 🐛 Report issues on [GitHub](https://github.com/evaisse/dart-re-analyzer/issues)
- ⭐ Star the project if you find it useful!

---

[← Back to Home](index)
