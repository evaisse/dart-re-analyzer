# Documentation Website

This directory contains the source files for the dart-re-analyzer documentation website, hosted on GitHub Pages.

## 🌐 Live Website

The documentation is available at: [https://evaisse.github.io/dart-re-analyzer](https://evaisse.github.io/dart-re-analyzer)

## 📁 Structure

```
docs/
├── _config.yml          # Jekyll configuration
├── _layouts/            # Custom layouts
│   └── default.html     # Main layout with navigation
├── index.md             # Homepage
├── quickstart.md        # Quick start guide
├── rules.md             # Complete rules reference
├── mcp.md              # MCP server documentation
├── lsp.md              # LSP integration guide
├── lsp-proxy.md        # LSP proxy setup
└── README.md           # This file
```

## 🛠️ Local Development

To preview the website locally:

1. Install Jekyll:
```bash
gem install bundler jekyll
```

2. Create a Gemfile in the docs directory:
```ruby
source 'https://rubygems.org'
gem 'github-pages', group: :jekyll_plugins
```

3. Install dependencies:
```bash
cd docs
bundle install
```

4. Serve the site:
```bash
bundle exec jekyll serve
```

5. Open your browser to `http://localhost:4000/dart-re-analyzer/`

## 📝 Editing Content

All content files are in Markdown format with YAML front matter:

```markdown
---
layout: default
title: Page Title
---

# Content here
```

### Adding a New Page

1. Create a new `.md` file in the `docs/` directory
2. Add YAML front matter with layout and title
3. Write your content in Markdown
4. Update navigation in `_layouts/default.html` if needed

### Updating Existing Pages

Simply edit the corresponding `.md` file and commit your changes. GitHub Pages will automatically rebuild the site.

## 🎨 Styling

The site uses the GitHub Pages default theme (Cayman). Custom styling can be added by creating:
- `assets/css/style.scss` for additional CSS

## 🚀 Deployment

The site is automatically deployed by GitHub Pages when changes are pushed to the main branch.

### Configuration

GitHub Pages is configured to:
- Source: `docs` folder from the main branch
- Theme: Cayman (minimal, no additional dependencies)
- Custom domain: Not configured (uses github.io)

### Enabling GitHub Pages

If not already enabled:

1. Go to repository Settings
2. Navigate to Pages section
3. Under "Source", select:
   - Branch: `main`
   - Folder: `/docs`
4. Click Save

The site will be available at `https://evaisse.github.io/dart-re-analyzer/`

## 📚 Documentation Pages

### Home (`index.md`)
- Project overview
- Key features
- Quick examples
- Links to detailed guides

### Quick Start (`quickstart.md`)
- Installation instructions
- Basic usage examples
- Common use cases
- CI/CD integration

### Rules (`rules.md`)
- Complete reference for all analysis rules
- Style rules (4 rules)
- Runtime rules (5 rules)
- Configuration examples

### MCP Server (`mcp.md`)
- MCP server API documentation
- Example clients (Python, Node.js)
- Integration tips
- Performance notes

### LSP Integration (`lsp.md`)
- LSP architecture
- Type information
- Semantic analysis
- Testing guide

### LSP Proxy (`lsp-proxy.md`)
- Proxy setup
- IDE configuration (VS Code, IntelliJ, Neovim, Emacs)
- Troubleshooting
- Performance tips

## 🔗 Internal Links

Use relative links between pages:
```markdown
[Quick Start Guide](quickstart)
[View All Rules](rules)
```

## 📊 Navigation

The main navigation is defined in `_layouts/default.html` and includes:
- Home
- Quick Start
- Rules
- MCP Server
- LSP
- LSP Proxy
- GitHub (external link)

## 🔍 SEO

Each page should have:
- Descriptive title in front matter
- Clear headings hierarchy (H1 → H6)
- Meaningful content

## 🧪 Testing

Before committing changes:

1. Preview locally with Jekyll
2. Check all internal links work
3. Verify code examples are correct
4. Ensure formatting is consistent

## 📦 Dependencies

The site uses minimal dependencies:
- Jekyll (GitHub Pages default)
- Cayman theme (GitHub Pages default)
- No custom plugins
- No additional gems

This keeps the site lightweight and easy to maintain.

## 🤝 Contributing

To contribute to the documentation:

1. Fork the repository
2. Make changes to files in the `docs/` directory
3. Test locally if possible
4. Submit a pull request

## 📄 License

The documentation is part of the dart-re-analyzer project and is licensed under MIT.
