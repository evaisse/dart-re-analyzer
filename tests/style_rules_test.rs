use dart_re_analyzer::analyzer::Rule;
use dart_re_analyzer::error::{RuleCategory, Severity};
use dart_re_analyzer::rules::style::*;
use std::path::Path;

#[test]
fn test_camel_case_class_name_valid() {
    let rule = CamelCaseClassNameRule;
    let content = "class MyClass {\n  void test() {}\n}\n";
    let path = Path::new("test.dart");

    let diagnostics = rule.check(path, content).unwrap();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn test_camel_case_class_name_invalid() {
    let rule = CamelCaseClassNameRule;
    let content = "class myClass {\n  void test() {}\n}\n";
    let path = Path::new("test.dart");

    let diagnostics = rule.check(path, content).unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "camel_case_class_names");
    assert!(matches!(diagnostics[0].severity, Severity::Warning));
    assert!(matches!(diagnostics[0].category, RuleCategory::Style));
}

#[test]
fn test_snake_case_file_name_valid() {
    let rule = SnakeCaseFileNameRule;
    let content = "";
    let path = Path::new("my_test_file.dart");

    let diagnostics = rule.check(path, content).unwrap();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn test_snake_case_file_name_invalid() {
    let rule = SnakeCaseFileNameRule;
    let content = "";
    let path = Path::new("MyTestFile.dart");

    let diagnostics = rule.check(path, content).unwrap();
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id, "snake_case_file_names");
}

#[test]
fn test_line_length_valid() {
    let rule = LineLengthRule::new(80);
    let content = "class Test {\n  void short() {}\n}\n";
    let path = Path::new("test.dart");

    let diagnostics = rule.check(path, content).unwrap();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn test_line_length_invalid() {
    let rule = LineLengthRule::new(50);
    let content = "class Test {\n  void thisIsAReallyLongMethodNameThatExceedsTheLimit() {}\n}\n";
    let path = Path::new("test.dart");

    let diagnostics = rule.check(path, content).unwrap();
    assert!(!diagnostics.is_empty());
    assert_eq!(diagnostics[0].rule_id, "line_length");
    assert!(matches!(diagnostics[0].severity, Severity::Info));
}
