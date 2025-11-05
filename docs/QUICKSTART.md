# Quick Start Guide

Get started with dart-re-analyzer in minutes!

## Installation

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

Optional: Add to PATH:
```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:/path/to/dart-re-analyzer/target/release"
```

## Quick Examples

### 1. Analyze Your First Project

```bash
# Navigate to your Dart/Flutter project
cd /path/to/your/flutter/project

# Run the analyzer
dart-re-analyzer analyze .
```

You'll see output like:
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
  "style_rules": {
    "enabled": true,
    "disabled_rules": ["line_length"]
  },
  "runtime_rules": {
    "enabled": true,
    "disabled_rules": []
  },
  "max_line_length": 100
}
```

### 3. Get JSON Output

For CI/CD integration or programmatic processing:

```bash
dart-re-analyzer analyze . --format json > results.json
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
        run: dart-re-analyzer analyze . --format json
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

## Understanding Output

### Severity Levels

- **✗ Error**: Must be fixed. Causes exit code 1.
- **⚠ Warning**: Should be fixed. Important issues.
- **ℹ Info**: Consider fixing. Best practice suggestions.

### Categories

- **style**: Code conventions and formatting
- **runtime**: Potential runtime errors and unsafe patterns

## Next Steps

1. Read the [Rules Reference](docs/RULES.md) to understand all rules
2. Check out [MCP Server Guide](docs/MCP_SERVER.md) for advanced integrations
3. Customize your configuration based on your team's needs
4. Set up CI/CD integration for automated checks

## Getting Help

- Check documentation in the `docs/` directory
- Review example configuration in `analyzer_config.example.json`
- Report issues on GitHub

## Performance Tips

- Use `--parallel true` (default) for large codebases
- Exclude generated files in configuration (*.g.dart, *.freezed.dart)
- Run style and runtime checks separately if needed
- Use MCP server for repeated queries instead of re-analyzing

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
