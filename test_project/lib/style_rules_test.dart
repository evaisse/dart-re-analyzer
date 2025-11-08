// This file tests style rules: camel_case_class_names, line_length, private_field_underscore

// Good: CamelCase class name
class GoodClassName {
  // Good: private field with underscore
  String _privateField = 'private';
  
  // Good: public field
  String publicField = 'public';
  
  void printMessage() {
    print('This is a good class');
  }
}

// Bad: class name should start with uppercase (violates camel_case_class_names)
class badClassName {
  String someField = 'test';
}

// Note: private_field_underscore rule is registered but not yet implemented
// This class demonstrates what should be checked when the rule is complete
class ClassWithBadPrivateField {
  String _goodPrivateField = 'good';
  // When implemented, the rule might check for fields that should be private
  String badPrivateField = 'public field - naming is okay';
}

// Bad: line too long (violates line_length rule - default max is 120 characters)
class ClassWithLongLine {
  String veryLongLineOfCodeThatExceedsTheMaximumLineLengthLimitAndShouldBeDetectedByTheAnalyzerAsAStyleViolationBecauseItIsTooLong = 'way too long';
}
