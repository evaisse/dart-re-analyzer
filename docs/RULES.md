# Rules Reference

This document describes all available analyzer rules.

## Implementation Note

**Current Approach**: The analyzer uses regex-based pattern matching for rule detection. This provides fast analysis suitable for most use cases but has some limitations:

- **False Positives**: Some rules may flag correct code (e.g., unused imports used only in type annotations)
- **False Negatives**: Complex patterns may be missed (e.g., dynamic usage in complex expressions)
- **Limited Context**: Cannot perform semantic analysis or type inference

**Future Improvements**: Future versions may integrate AST parsing (tree-sitter or Dart analyzer) for more accurate detection.

For production use, consider:
1. Running alongside official Dart analyzer for comprehensive coverage
2. Tuning configuration to disable rules that produce too many false positives for your codebase
3. Using as a quick pre-commit check with full analysis in CI/CD

---

## Style Rules

Style rules focus on code conventions, naming patterns, and file organization.

### camel_case_class_names

**Category**: Style  
**Severity**: Warning

Ensures that class names follow CamelCase convention (start with uppercase letter).

**Bad:**
```dart
class myClass {  // ❌
  // ...
}

class my_class {  // ❌
  // ...
}
```

**Good:**
```dart
class MyClass {  // ✓
  // ...
}

class MyAwesomeClass {  // ✓
  // ...
}
```

### snake_case_file_names

**Category**: Style  
**Severity**: Warning

Ensures that Dart file names follow snake_case convention (lowercase with underscores).

**Bad:**
```
MyFile.dart       // ❌
myFile.dart       // ❌
My-File.dart      // ❌
```

**Good:**
```
my_file.dart      // ✓
my_awesome_file.dart  // ✓
widget.dart       // ✓
```

### private_field_underscore

**Category**: Style  
**Severity**: Warning

Ensures that private fields start with an underscore.

**Bad:**
```dart
class MyClass {
  String privateField;  // ❌ if meant to be private
}
```

**Good:**
```dart
class MyClass {
  String _privateField;  // ✓
  String publicField;    // ✓
}
```

### line_length

**Category**: Style  
**Severity**: Info

Ensures that lines don't exceed a maximum length (default: 120 characters).

**Bad:**
```dart
String reallyLongVariableName = "This is a really long line that exceeds the maximum line length and should be broken up into multiple lines";  // ❌
```

**Good:**
```dart
String reallyLongVariableName = 
    "This is a long line that has been broken up " +
    "into multiple lines for better readability";  // ✓
```

**Configuration:**
```json
{
  "max_line_length": 100
}
```

## Runtime Rules

Runtime rules focus on preventing runtime errors and identifying unsafe code patterns.

### avoid_dynamic

**Category**: Runtime  
**Severity**: Warning

Detects usage of the `dynamic` type which bypasses Dart's type safety.

**Bad:**
```dart
dynamic data = fetchData();  // ❌
void process(dynamic item) {  // ❌
  print(item.unknownMethod());  // No compile-time checking
}
```

**Good:**
```dart
Object? data = fetchData();  // ✓
void process(Object item) {  // ✓
  if (item is String) {
    print(item.length);
  }
}
```

**Why:** Using `dynamic` defeats the purpose of Dart's static type system and can lead to runtime errors that could have been caught at compile time.

### avoid_empty_catch

**Category**: Runtime  
**Severity**: Error

Detects empty catch blocks that silently swallow exceptions.

**Bad:**
```dart
try {
  riskyOperation();
} catch (e) {}  // ❌ Silent failure
```

**Good:**
```dart
try {
  riskyOperation();
} catch (e) {
  print('Error: $e');  // ✓ At least log it
  // or rethrow
}

try {
  riskyOperation();
} catch (e) {
  // Handle specific error
  handleError(e);  // ✓
}
```

**Why:** Empty catch blocks hide errors and make debugging difficult. At minimum, log the exception.

### unused_import

**Category**: Runtime  
**Severity**: Warning

Detects import statements that are not used in the file.

**Bad:**
```dart
import 'dart:async';  // ❌ Not used
import 'dart:io';     // ❌ Not used

void main() {
  print('Hello');
}
```

**Good:**
```dart
import 'dart:async';  // ✓ Used below

void main() async {
  await Future.delayed(Duration(seconds: 1));
  print('Hello');
}
```

**Why:** Unused imports clutter the code, increase compilation time, and can cause confusion.

### avoid_print

**Category**: Runtime  
**Severity**: Info

Detects usage of `print()` statements in production code.

**Bad:**
```dart
void processData() {
  print('Processing...');  // ❌
  // ...
  print('Done');  // ❌
}
```

**Good:**
```dart
import 'package:logger/logger.dart';

final logger = Logger();

void processData() {
  logger.d('Processing...');  // ✓
  // ...
  logger.i('Done');  // ✓
}

// Or use developer.log
import 'dart:developer' as developer;

void processData() {
  developer.log('Processing...');  // ✓
}
```

**Why:** `print()` statements are not suitable for production as they can't be controlled, filtered, or disabled. Use a proper logging framework.

### avoid_null_check_on_nullable

**Category**: Runtime  
**Severity**: Warning

Detects usage of the null assertion operator (`!`) which can cause runtime errors.

**Bad:**
```dart
void process(String? value) {
  print(value!.length);  // ❌ Throws if value is null
}

String? getData() => null;
var result = getData()!.toUpperCase();  // ❌
```

**Good:**
```dart
void process(String? value) {
  // Use null-aware operator
  print(value?.length);  // ✓
  
  // Or null check
  if (value != null) {  // ✓
    print(value.length);
  }
  
  // Or provide default
  print(value?.length ?? 0);  // ✓
}
```

**Why:** The null assertion operator (`!`) throws a runtime exception if the value is null. Use null-safe alternatives instead.

## Disabling Rules

You can disable specific rules in your configuration:

```json
{
  "style_rules": {
    "enabled": true,
    "disabled_rules": ["line_length"]
  },
  "runtime_rules": {
    "enabled": true,
    "disabled_rules": ["avoid_print"]
  }
}
```

Or disable entire categories:

```json
{
  "style_rules": {
    "enabled": false
  },
  "runtime_rules": {
    "enabled": true
  }
}
```

## Command Line Filters

Use command line flags to run only specific rule categories:

```bash
# Only style rules
dart-re-analyzer analyze . --style-only

# Only runtime rules
dart-re-analyzer analyze . --runtime-only
```
