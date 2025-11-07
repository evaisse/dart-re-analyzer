# Dart Test Project

This is a test Dart project designed to validate the functionality of dart-re-analyzer. It contains intentional code violations to ensure all analyzer rules are working correctly.

## Purpose

This project serves as an integration test for dart-re-analyzer, ensuring that:
- All style rules are detected correctly
- All runtime rules are detected correctly
- Various output formats work as expected
- Configuration options work properly

## Test Files

### `lib/style_rules_test.dart`
Tests style rules:
- `camel_case_class_names` - Ensures class names use CamelCase
- `line_length` - Enforces maximum line length
- `private_field_underscore` - Checks private fields start with underscore

### `lib/runtime_rules_test.dart`
Tests runtime safety rules:
- `avoid_dynamic` - Detects usage of `dynamic` type
- `avoid_empty_catch` - Catches empty catch blocks
- `avoid_print` - Warns about print statements
- `avoid_null_check_on_nullable` - Detects unsafe null assertion operators

### `lib/unused_imports_test.dart`
Tests import analysis:
- `unused_import` - Identifies unused imports

### `lib/BadFileName.dart`
Tests file naming conventions:
- `snake_case_file_names` - Ensures file names use snake_case

## Running Tests

From the repository root:

```bash
./test_analyzer.sh
```

This script:
1. Builds dart-re-analyzer in release mode
2. Runs the analyzer with various configurations
3. Validates that all expected rule violations are detected
4. Tests JSON output format
5. Tests configuration file generation
6. Tests custom configuration options

## CI Integration

This test project is integrated into the CI pipeline via the `integration-test` job in `.github/workflows/rust-ci.yml`, ensuring that every commit is validated against real Dart code.
