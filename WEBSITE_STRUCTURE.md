# Documentation Website Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    dart-re-analyzer Website                  │
│              https://evaisse.github.io/dart-re-analyzer/     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                        Home (index.md)                       │
│  • Project overview                                          │
│  • Key features summary                                      │
│  • Quick examples                                            │
│  • Rule count (4 style + 5 runtime)                          │
│  • Integration options                                       │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
    ┌─────────────┐  ┌──────────────┐  ┌──────────────┐
    │ Quick Start │  │    Rules     │  │ Integrations │
    │   (391 L)   │  │   (382 L)    │  │              │
    └─────────────┘  └──────────────┘  └──────────────┘
           │               │                    │
           │               │         ┌──────────┼──────────┐
           │               │         ▼          ▼          ▼
           │               │    ┌────────┐ ┌─────┐  ┌──────────┐
           │               │    │  MCP   │ │ LSP │  │LSP Proxy │
           │               │    │(288 L) │ │(350)│  │ (296 L)  │
           │               │    └────────┘ └─────┘  └──────────┘
           │               │
           ▼               ▼
    ┌─────────────────────────────────────────────┐
    │           Page Contents                      │
    ├─────────────────────────────────────────────┤
    │ Quick Start (quickstart.md):                │
    │  • Installation from source                  │
    │  • Basic usage examples                      │
    │  • CI/CD integration                         │
    │  • Pre-commit hooks                          │
    │  • VS Code tasks                             │
    │  • Performance tips                          │
    ├─────────────────────────────────────────────┤
    │ Rules (rules.md):                            │
    │  Style Rules (4):                            │
    │   • camel_case_class_names                   │
    │   • snake_case_file_names                    │
    │   • private_field_underscore                 │
    │   • line_length                              │
    │  Runtime Rules (5):                          │
    │   • avoid_dynamic                            │
    │   • avoid_empty_catch                        │
    │   • unused_import                            │
    │   • avoid_print                              │
    │   • avoid_null_check_on_nullable             │
    ├─────────────────────────────────────────────┤
    │ MCP Server (mcp.md):                         │
    │  • Starting the server                       │
    │  • API methods (3):                          │
    │    - get_all_errors                          │
    │    - get_errors (filtered)                   │
    │    - get_stats                               │
    │  • Example clients:                          │
    │    - Python                                  │
    │    - Node.js                                 │
    │    - curl/netcat                             │
    ├─────────────────────────────────────────────┤
    │ LSP Integration (lsp.md):                    │
    │  • Architecture overview                     │
    │  • Current implementation status             │
    │  • Usage examples                            │
    │  • Type & symbol information                 │
    │  • Integration with Tree-sitter              │
    │  • Semantic analysis use cases               │
    ├─────────────────────────────────────────────┤
    │ LSP Proxy (lsp-proxy.md):                    │
    │  • How proxy works                           │
    │  • IDE configuration:                        │
    │    - VS Code                                 │
    │    - IntelliJ IDEA                           │
    │    - Neovim                                  │
    │    - Emacs                                   │
    │  • Troubleshooting guide                     │
    │  • Performance tips                          │
    └─────────────────────────────────────────────┘

Navigation Menu (on every page):
┌─────────────────────────────────────────────────────────────┐
│ Home | Quick Start | Rules | MCP Server | LSP | LSP Proxy  │
└─────────────────────────────────────────────────────────────┘

Footer (on every page):
┌─────────────────────────────────────────────────────────────┐
│ View on GitHub • Report an Issue • License: MIT             │
│ © 2025 Emmanuel Vaisse                                       │
└─────────────────────────────────────────────────────────────┘

Technical Details:
┌─────────────────────────────────────────────────────────────┐
│ • Theme: Cayman (GitHub Pages default)                       │
│ • No custom dependencies                                     │
│ • Custom navigation layout                                   │
│ • Markdown with YAML front matter                           │
│ • Responsive design                                          │
│ • Total: ~3,800 lines of documentation                       │
└─────────────────────────────────────────────────────────────┘
```

## Files Created

### Configuration & Layout
- `_config.yml` - Jekyll configuration (32 lines)
- `_layouts/default.html` - Custom layout with navigation (41 lines)
- `Gemfile` - Ruby dependencies for local development
- `.gitignore` - Updated to exclude Jekyll build artifacts

### Documentation Pages
- `index.md` - Homepage (145 lines)
- `quickstart.md` - Quick start guide (391 lines)
- `rules.md` - Complete rules reference (382 lines)
- `mcp.md` - MCP server documentation (288 lines)
- `lsp.md` - LSP integration guide (350 lines)
- `lsp-proxy.md` - LSP proxy setup (296 lines)
- `README.md` - Docs folder documentation (200 lines)

### Total
- **7 documentation pages**
- **~3,800 lines** of content
- **9 rules documented** (4 style + 5 runtime)
- **Multiple integration guides** (CLI, MCP, LSP, LSP Proxy)
- **Code examples** in Dart, Python, Node.js, Bash, Lua, Elisp
