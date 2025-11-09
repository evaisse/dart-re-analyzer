---
layout: default
title: LSP Integration
---

# LSP Integration Guide

The LSP (Language Server Protocol) module provides integration capabilities for semantic analysis in dart-re-analyzer.

## Table of Contents
- [Overview](#overview)
- [Architecture](#architecture)
- [Current Status](#current-status)
- [Usage Examples](#usage-examples)
- [Integration with Tree-sitter](#integration-with-tree-sitter)
- [Semantic Analysis Use Cases](#semantic-analysis-use-cases)

---

## Overview

The LSP integration enables semantic analysis capabilities that go beyond syntax-level parsing:

- **Type resolution and inference** - Determine the actual types of variables and expressions
- **Null-safety flow analysis** - Track nullable types through control flow
- **Symbol resolution across files** - Find definitions and references across the project
- **IDE-quality diagnostics** - Get comprehensive error messages and warnings

---

## Architecture

The LSP integration is designed with a clean separation of concerns:

```
src/lsp/
├── mod.rs     - Core types and traits
│   ├── SemanticAnalyzer trait - Interface for semantic operations
│   ├── MockSemanticAnalyzer   - Testing implementation
│   ├── SymbolInfo            - Symbol metadata
│   ├── TypeInfo              - Type information
│   └── SemanticDiagnostic    - Diagnostic messages
│
└── client.rs  - LSP client implementation
    ├── DartAnalysisServerClient - Communicates with analysis server
    ├── DartAnalysisServerConfig - Configuration
    └── find_dart_sdk()           - SDK discovery helper
```

---

## Current Status

### ✅ Completed
- Core type definitions (`SymbolInfo`, `TypeInfo`, `SemanticDiagnostic`)
- `SemanticAnalyzer` trait defining the interface
- Mock implementation for testing and demonstration
- Stub client showing intended architecture
- Comprehensive test coverage

### 🚧 Future Work
The following would be needed for a production LSP implementation:

1. **Process Management**
   - Start/stop Dart Analysis Server subprocess
   - Handle server crashes and restarts
   - Monitor server health

2. **Communication Protocol**
   - JSON-RPC serialization/deserialization
   - Request/response correlation via IDs
   - Async message handling
   - Notification processing

3. **LSP Methods**
   - `initialize` - Start server session
   - `textDocument/didOpen` - Register open files
   - `textDocument/didChange` - Send edits
   - `textDocument/hover` - Get hover information
   - `textDocument/definition` - Find definitions
   - `textDocument/references` - Find references
   - `textDocument/diagnostic` - Get diagnostics

4. **State Management**
   - Track open documents
   - Synchronize document changes
   - Cache responses when appropriate

---

## Usage Examples

### Using the Mock Analyzer

```rust
use dart_re_analyzer::lsp::{
    MockSemanticAnalyzer, SemanticAnalyzer, SemanticDiagnostic,
    DiagnosticSeverity,
};
use std::path::PathBuf;

let mut analyzer = MockSemanticAnalyzer::new();
let file = PathBuf::from("lib/main.dart");

// Add a diagnostic
analyzer.add_diagnostic(
    file.clone(),
    SemanticDiagnostic {
        message: "Undefined name 'unknownVar'".to_string(),
        severity: DiagnosticSeverity::Error,
        file: file.clone(),
        start_line: 5,
        start_column: 10,
        end_line: 5,
        end_column: 20,
        code: Some("undefined_identifier".to_string()),
        fixes: vec!["Import package".to_string()],
    },
);

// Get diagnostics
let diagnostics = analyzer.get_diagnostics(&file)?;
for diagnostic in diagnostics {
    println!("{}: {}", diagnostic.severity, diagnostic.message);
}

// Resolve type at a position
let type_info = analyzer.resolve_type(&file, 10, 5)?;
if let Some(ty) = type_info {
    println!("Type: {}{}", ty.name, if ty.is_nullable { "?" } else { "" });
}
```

### Type Information

```rust
use dart_re_analyzer::lsp::TypeInfo;

// Simple type
let string_type = TypeInfo {
    name: "String".to_string(),
    is_nullable: false,
    type_arguments: vec![],
    is_function: false,
    return_type: None,
    parameter_types: vec![],
};

// Generic type: List<String>
let list_type = TypeInfo {
    name: "List".to_string(),
    is_nullable: false,
    type_arguments: vec![string_type],
    is_function: false,
    return_type: None,
    parameter_types: vec![],
};
```

### Symbol Information

```rust
use dart_re_analyzer::lsp::{SymbolInfo, SymbolKind};
use std::path::PathBuf;

let class_symbol = SymbolInfo {
    name: "MyWidget".to_string(),
    kind: SymbolKind::Class,
    resolved_type: Some("MyWidget".to_string()),
    is_nullable: false,
    definition_file: PathBuf::from("lib/widgets/my_widget.dart"),
    definition_line: 15,
    definition_column: 6,
};

// Use symbol info for navigation
println!("Go to: {}:{}:{}", 
    class_symbol.definition_file.display(),
    class_symbol.definition_line,
    class_symbol.definition_column
);
```

---

## Integration with Tree-sitter

The LSP module complements Tree-sitter parsing:

| Feature | Tree-sitter | LSP |
|---------|-------------|-----|
| Syntax Analysis | ✅ Full | ❌ N/A |
| Type Information | ❌ No | ✅ Full |
| Cross-file Analysis | ❌ No | ✅ Full |
| Null Safety | ❌ No | ✅ Full |
| Parse Speed | ⚡ Very Fast | 🐢 Slower |
| Error Recovery | ✅ Excellent | ⚠️ Varies |
| Setup Complexity | ⚡ Simple | 🔧 Complex |

**Recommended Hybrid Approach:**
1. Use Tree-sitter for structural analysis (classes, methods, imports)
2. Use LSP for semantic analysis (types, null safety, cross-file references)
3. Combine both for comprehensive analysis

---

## Semantic Analysis Use Cases

### 1. Type Safety Validation

```dart
// Example Dart code
void processData(String? nullableString) {
  print(nullableString.length);  // ⚠️ LSP detects potential null error
}
```

The LSP analyzer would report:
- **Diagnostic**: "The property 'length' can't be unconditionally accessed..."
- **Severity**: Error
- **Fix**: "Add a null check"

### 2. Unused Code Detection

```dart
import 'dart:math';  // ⚠️ Unused import

void main() {
  print('Hello');
}
```

The LSP analyzer would report:
- **Diagnostic**: "Unused import: 'dart:math'"
- **Severity**: Info
- **Fix**: "Remove unused import"

### 3. Type Inference

```dart
var items = ['a', 'b', 'c'];  // LSP infers: List<String>
```

Query type at position to get:
- **Type**: `List<String>`
- **Is Nullable**: false
- **Type Arguments**: `[String]`

### 4. Cross-file Navigation

```dart
// File: lib/models/user.dart
class User {
  String name;
}

// File: lib/main.dart
void greet(User user) {  // Click on User -> Go to definition
  print(user.name);
}
```

LSP provides:
- **Definition**: `lib/models/user.dart:1:6`
- **References**: All uses of `User` class

---

## Diagnostic Severity Levels

```rust
pub enum DiagnosticSeverity {
    Error,    // Compilation errors, must be fixed
    Warning,  // Potential issues, should be reviewed
    Info,     // Informational messages
    Hint,     // Suggestions for improvement
}
```

---

## Symbol Kinds

```rust
pub enum SymbolKind {
    Class,         // Classes
    Function,      // Top-level functions
    Method,        // Class methods
    Field,         // Class fields
    Variable,      // Local variables
    Parameter,     // Function/method parameters
    TypeParameter, // Generic type parameters
    Enum,          // Enum types
    Mixin,         // Mixins
    Extension,     // Extensions
}
```

---

## Testing

The module includes comprehensive tests:

```bash
# Run LSP module tests
cargo test lsp

# Run specific test
cargo test lsp::tests::test_mock_analyzer_resolve_type
```

Current test coverage:
- ✅ Mock analyzer type resolution
- ✅ Diagnostic management
- ✅ Symbol info creation
- ✅ Type info with generics
- ✅ Diagnostic severity comparison
- ✅ Client configuration
- ✅ Client creation

---

## Performance Considerations

Full LSP integration has performance implications:

- **Startup**: Analysis server takes 1-3 seconds to initialize
- **Analysis**: Initial analysis can take 5-30 seconds for large projects
- **Incremental**: Changes are analyzed in ~100-500ms
- **Memory**: Server can use 100-500MB RAM

**Optimization strategies:**
1. Start server lazily (only when semantic analysis is needed)
2. Cache analysis results
3. Use incremental synchronization
4. Implement request debouncing
5. Limit concurrent analysis requests

---

## Resources

- [Language Server Protocol Specification](https://microsoft.github.io/language-server-protocol/)
- [Dart Analysis Server Documentation](https://github.com/dart-lang/sdk/tree/main/pkg/analysis_server)
- [LSP Types for Rust](https://docs.rs/lsp-types/)

---

[← Back to Home](index) | [LSP Proxy Guide →](lsp-proxy)
