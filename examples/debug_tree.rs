use dart_re_analyzer::treesitter::{parse_dart, print_tree};

fn main() {
    // Test fields
    let source = r#"
class MyClass {
    static final int staticField = 10;
    String name;
    final double value = 3.14;
}
"#;

    println!("=== FIELDS TEST ===");
    let tree = parse_dart(source).unwrap();
    print_tree(&tree, source);

    // Test variables
    let source2 = r#"
void main() {
    var x = 10;
    final String name = 'test';
    int count = 0;
}
"#;

    println!("\n\n=== VARIABLES TEST ===");
    let tree2 = parse_dart(source2).unwrap();
    print_tree(&tree2, source2);
}
