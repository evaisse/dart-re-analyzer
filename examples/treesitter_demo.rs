// Example demonstrating Tree-sitter parsing capabilities for Dart

use dart_re_analyzer::treesitter::{
    parse_dart, extract_tokens, extract_classes, extract_imports, print_tree
};

fn main() {
    let dart_code = r#"
import 'dart:core';
import 'package:flutter/material.dart';

/// A sample Flutter widget
class MyHomePage extends StatefulWidget {
    final String title;
    
    MyHomePage({Key? key, required this.title}) : super(key: key);
    
    @override
    State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
    int _counter = 0;
    
    void _incrementCounter() {
        setState(() {
            _counter++;
        });
    }
    
    @override
    Widget build(BuildContext context) {
        return Scaffold(
            appBar: AppBar(
                title: Text(widget.title),
            ),
            body: Center(
                child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: <Widget>[
                        Text('You have pushed the button this many times:'),
                        Text(
                            '$_counter',
                            style: Theme.of(context).textTheme.headline4,
                        ),
                    ],
                ),
            ),
            floatingActionButton: FloatingActionButton(
                onPressed: _incrementCounter,
                tooltip: 'Increment',
                child: Icon(Icons.add),
            ),
        );
    }
}

void main() {
    runApp(MyApp());
}

class MyApp extends StatelessWidget {
    @override
    Widget build(BuildContext context) {
        return MaterialApp(
            title: 'Flutter Demo',
            theme: ThemeData(
                primarySwatch: Colors.blue,
            ),
            home: MyHomePage(title: 'Flutter Demo Home Page'),
        );
    }
}
    "#;

    println!("=== Dart Tree-sitter Parsing Demo ===\n");

    // Parse the code
    println!("1. Parsing Dart source code...");
    let tree = match parse_dart(dart_code) {
        Ok(tree) => {
            println!("   ✓ Successfully parsed {} bytes", dart_code.len());
            tree
        }
        Err(e) => {
            eprintln!("   ✗ Parse error: {}", e);
            return;
        }
    };

    let root = tree.root_node();
    println!("   Root node: {}", root.kind());
    println!("   Has errors: {}\n", root.has_error());

    // Extract classes
    println!("2. Extracting class declarations...");
    let classes = extract_classes(&tree, dart_code);
    println!("   Found {} classes:", classes.len());
    for class in &classes {
        println!("     - {} at byte range [{}..{}]", 
                 class.name, class.start_byte, class.end_byte);
    }
    println!();

    // Extract imports
    println!("3. Extracting import statements...");
    let imports = extract_imports(&tree, dart_code);
    println!("   Found {} imports:", imports.len());
    for import in &imports {
        println!("     - {}", import.uri);
    }
    println!();

    // Extract and count tokens
    println!("4. Extracting all tokens...");
    let tokens = extract_tokens(&tree, dart_code);
    println!("   Total tokens: {}", tokens.len());
    
    // Show some token statistics
    let keywords: Vec<_> = tokens.iter()
        .filter(|t| matches!(t.text.as_str(), "class" | "void" | "import" | "extends" | "return" | "final"))
        .collect();
    println!("   Keywords found: {}", keywords.len());
    
    let identifiers: Vec<_> = tokens.iter()
        .filter(|t| t.kind == "identifier")
        .collect();
    println!("   Identifiers: {}", identifiers.len());
    println!();

    // Show first 20 tokens
    println!("5. First 20 tokens:");
    for (i, token) in tokens.iter().take(20).enumerate() {
        println!("   {:2}. {:15} {:?} at line {}", 
                 i + 1, token.kind, token.text, token.start_point.row);
    }
    println!();

    // Optional: Uncomment to print the full tree structure
    // println!("6. Full syntax tree structure:");
    // print_tree(&tree, dart_code);

    println!("=== Demo Complete ===");
    println!("\nThis demonstrates:");
    println!("  ✓ Complete tokenization with position information");
    println!("  ✓ Extraction of specific constructs (classes, imports)");
    println!("  ✓ Error-tolerant parsing");
    println!("  ✓ Full access to the concrete syntax tree");
}
