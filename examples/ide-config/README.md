# IDE Configuration Examples

This directory contains example configurations for integrating dart-re-analyzer's LSP proxy with various IDEs and editors.

## VS Code

Create or update `.vscode/settings.json`:

```json
{
  "dart.analysisServerPath": "/path/to/dart-re-analyzer",
  "dart.analysisServerArgs": ["language-server"],
  "dart.showTodos": false
}
```

## VS Code Workspace Settings

For project-specific settings, create `.vscode/settings.json` in your project root:

```json
{
  "dart.analysisServerPath": "${workspaceFolder}/../dart-re-analyzer/target/release/dart-re-analyzer",
  "dart.analysisServerArgs": [
    "language-server",
    "--config",
    "analyzer_config.json"
  ]
}
```

## Neovim with nvim-lspconfig

Add to your `init.lua`:

```lua
require('lspconfig').dartls.setup({
  cmd = {
    '/path/to/dart-re-analyzer',
    'language-server',
    '--config',
    vim.fn.getcwd() .. '/analyzer_config.json'
  },
  root_dir = require('lspconfig.util').root_pattern('pubspec.yaml'),
  settings = {
    dart = {
      enableSdkFormatter = false,
    }
  }
})
```

## Emacs with lsp-mode

Add to your Emacs config:

```elisp
(use-package lsp-dart
  :ensure t
  :custom
  (lsp-dart-server-command 
    '("/path/to/dart-re-analyzer" 
      "language-server" 
      "--config" 
      "analyzer_config.json"))
  :hook
  (dart-mode . lsp))
```

## Sublime Text with LSP

Add to your LSP settings:

```json
{
  "clients": {
    "dartls": {
      "enabled": true,
      "command": [
        "/path/to/dart-re-analyzer",
        "language-server"
      ],
      "selector": "source.dart"
    }
  }
}
```

## Helix Editor

Add to your `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "dart"
language-server = { command = "/path/to/dart-re-analyzer", args = ["language-server"] }
```

## Kate/KTextEditor

Configure the LSP client plugin with:

```json
{
  "servers": {
    "dart": {
      "command": ["/path/to/dart-re-analyzer", "language-server"],
      "rootIndicationFileNames": ["pubspec.yaml"]
    }
  }
}
```

## Troubleshooting

### Binary Path Issues

If you built from source, the binary is at:
```
target/release/dart-re-analyzer
```

Or use the absolute path:
```bash
which dart-re-analyzer  # If installed globally
realpath target/release/dart-re-analyzer  # From build directory
```

### Testing Your Configuration

Test that the LSP proxy works:

```bash
# In your project directory
dart-re-analyzer language-server
```

Then send an LSP initialize request manually to verify it responds correctly.

### Viewing LSP Messages

Enable LSP logging in your editor to see the messages being exchanged:

**VS Code**: Set `"dart.analyzerDiagnosticsPort"` to see analysis server output

**Neovim**: Set `vim.lsp.set_log_level("debug")` to enable LSP logging
