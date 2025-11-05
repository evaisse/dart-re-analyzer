# Examples

This directory contains example programs demonstrating different features of dart-re-analyzer.

## Tree-sitter Demo

**File**: `treesitter_demo.rs`

Demonstrates the Tree-sitter AST parsing capabilities:
- Complete tokenization with position information
- Class extraction from Dart source code
- Import statement extraction
- Error-tolerant parsing

**Run it:**
```bash
cargo run --example treesitter_demo
```

**What it shows:**
- Parsing a complete Flutter widget (MyHomePage example)
- Extracting structural elements (classes, imports)
- Token-level analysis with statistics
- Precise byte positions and line/column info

## Adding More Examples

To add a new example:

1. Create a new `.rs` file in this directory
2. Add it to `Cargo.toml` under `[[example]]` section (optional, auto-detected)
3. Use the library: `use dart_re_analyzer::...;`
4. Run with: `cargo run --example <name>`

Example structure:
```rust
use dart_re_analyzer::treesitter::parse_dart;

fn main() {
    let source = "class A {}";
    let tree = parse_dart(source).unwrap();
    println!("Parsed successfully!");
}
```

See the [Tree-sitter Guide](../docs/TREESITTER.md) for more API documentation.
