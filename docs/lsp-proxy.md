---
layout: default
title: LSP Proxy
---

# LSP Proxy Integration

The dart-re-analyzer can act as a transparent proxy for the Dart Analysis Server, injecting additional diagnostics from dart-re-analyzer into your IDE or editor.

## Table of Contents
- [Overview](#overview)
- [How It Works](#how-it-works)
- [Usage](#usage)
- [IDE Configuration](#ide-configuration)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

---

## Overview

The LSP proxy feature allows you to:
- ✅ Use dart-re-analyzer diagnostics directly in your IDE
- ✅ Get real-time feedback on code quality issues
- ✅ Keep the full power of Dart Analysis Server for semantic analysis
- ✅ Seamlessly integrate both analyzers without configuration changes in your IDE

---

## How It Works

```
IDE/Editor <--LSP--> dart-re-analyzer <--LSP--> Dart Analysis Server
                     (LSP Proxy)
                         ↓
                    Injects additional
                    diagnostics
```

When you use the LSP proxy:
1. Your IDE connects to dart-re-analyzer instead of directly to the Dart Analysis Server
2. dart-re-analyzer forwards all LSP messages to the real Dart Analysis Server
3. When the Dart Analysis Server sends diagnostics, dart-re-analyzer injects its own diagnostics
4. Your IDE receives both sets of diagnostics as if they came from a single analyzer

---

## Usage

### Basic Usage

Start the LSP proxy in your project directory:

```bash
dart-re-analyzer language-server
```

Or specify a project path:

```bash
dart-re-analyzer language-server /path/to/project
```

### With Custom Configuration

```bash
dart-re-analyzer language-server --config analyzer_config.json
```

### Custom Dart Binary Path

If `dart` is not in your PATH, or you want to use a specific Dart SDK:

```bash
dart-re-analyzer language-server --dart-binary /path/to/dart
```

---

## IDE Configuration

### VS Code

1. Install the Dart extension if you haven't already
2. Add to your `.vscode/settings.json`:

```json
{
  "dart.analysisServerPath": "/path/to/dart-re-analyzer",
  "dart.analysisServerArgs": ["language-server"]
}
```

Replace `/path/to/dart-re-analyzer` with the actual path to the dart-re-analyzer binary.

### IntelliJ IDEA / Android Studio

1. Go to **Settings** → **Languages & Frameworks** → **Dart**
2. In the "Dart Analysis Server" section, enable "Use custom analysis server"
3. Set the path to dart-re-analyzer with the `language-server` command

### Neovim with nvim-lspconfig

Add to your Neovim configuration:

```lua
local lspconfig = require('lspconfig')

lspconfig.dartls.setup({
  cmd = { '/path/to/dart-re-analyzer', 'language-server' },
  root_dir = lspconfig.util.root_pattern('pubspec.yaml'),
})
```

### Emacs with lsp-mode

Add to your Emacs configuration:

```elisp
(use-package lsp-dart
  :custom
  (lsp-dart-server-command '("/path/to/dart-re-analyzer" "language-server")))
```

---

## Diagnostic Format

Diagnostics from dart-re-analyzer will appear with:
- **Source**: `dart-re-analyzer`
- **Code**: The rule ID (e.g., `avoid_dynamic`, `camel_case_class_names`)
- **Severity**: Error, Warning, or Info based on the rule

This makes it easy to distinguish dart-re-analyzer diagnostics from Dart Analysis Server diagnostics.

---

## Configuration

The LSP proxy respects your `analyzer_config.json` settings:

```json
{
  "enabled": true,
  "exclude_patterns": [
    ".dart_tool/**",
    "build/**"
  ],
  "style_rules": {
    "enabled": true,
    "disabled_rules": []
  },
  "runtime_rules": {
    "enabled": true,
    "disabled_rules": ["avoid_print"]
  },
  "max_line_length": 120,
  "parallel": true
}
```

### Generate Configuration

```bash
dart-re-analyzer init-config
```

This creates a default `analyzer_config.json` file in the current directory.

---

## Performance

The LSP proxy is designed for efficiency:

- ✅ Analyzes your workspace once on initialization
- ✅ Caches diagnostics for fast injection
- ✅ Forwards all messages with minimal overhead
- ✅ Runs analysis in parallel for better performance

**Performance Metrics:**

For large projects (1000+ files):
- Initial analysis: 2-5 seconds
- Subsequent diagnostic updates: Instant (cached)
- Memory overhead: ~50-100MB

---

## Troubleshooting

### "Failed to start Dart Analysis Server"

Make sure `dart` is in your PATH or specify the path with `--dart-binary`:

```bash
which dart  # Check if dart is available
dart-re-analyzer language-server --dart-binary /usr/local/bin/dart
```

### Diagnostics Not Showing

1. Check that your IDE is connecting to dart-re-analyzer
2. Look at stderr output for error messages:
   ```bash
   dart-re-analyzer language-server 2> /tmp/lsp-errors.log
   ```
3. Verify your configuration file is valid JSON

### Duplicate Diagnostics

If you see duplicate diagnostics, make sure you're not running both:
- The standalone dart-re-analyzer analyzer
- The LSP proxy

Use one or the other, not both.

### Slow Performance

For large projects:
1. Exclude generated files in your configuration:
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
2. Consider disabling rules you don't need
3. Use `--parallel true` (enabled by default)

---

## Advanced: Protocol Details

The LSP proxy implements the Language Server Protocol v3.17:
- Supports all standard LSP messages (initialize, textDocument/*, etc.)
- Forwards all requests/responses transparently
- Only modifies `textDocument/publishDiagnostics` notifications to inject diagnostics
- Maintains state for diagnostic caching

---

## Limitations

1. **Semantic Analysis**: dart-re-analyzer provides syntactic analysis only. For semantic analysis (type inference, null safety flow analysis), the Dart Analysis Server is still used.

2. **Real-time Updates**: Currently, diagnostics are analyzed once on initialization. File changes don't trigger re-analysis (coming in a future version).

3. **Dart SDK Required**: You must have the Dart SDK installed and accessible.

---

## Future Enhancements

Planned improvements:
- 🔜 Watch mode for file changes
- 🔜 Incremental analysis for better performance
- 🔜 Configuration hot-reload
- 🔜 More granular diagnostic filtering

---

## Example: Complete Setup

Here's a complete example for VS Code:

1. **Build dart-re-analyzer:**
   ```bash
   git clone https://github.com/evaisse/dart-re-analyzer.git
   cd dart-re-analyzer
   cargo build --release
   ```

2. **Create configuration:**
   ```bash
   cd /path/to/your/flutter/project
   /path/to/dart-re-analyzer/target/release/dart-re-analyzer init-config
   ```

3. **Configure VS Code** (`.vscode/settings.json`):
   ```json
   {
     "dart.analysisServerPath": "/path/to/dart-re-analyzer/target/release/dart-re-analyzer",
     "dart.analysisServerArgs": ["language-server"]
   }
   ```

4. **Restart VS Code** and open a Dart file to see diagnostics!

---

[← LSP Integration](lsp) | [Back to Home →](index)
