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

// Bad: private field without underscore (violates private_field_underscore)
class ClassWithBadPrivateField {
  String _goodPrivateField = 'good';
  // This should be _badPrivateField according to the rule if it's meant to be private
  String badPrivateField = 'should have underscore if private';
}

// Bad: line too long (violates line_length rule - default max is 120 characters)
class ClassWithLongLine {
  String veryLongLineOfCodeThatExceedsTheMaximumLineLengthLimitAndShouldBeDetectedByTheAnalyzerAsAStyleViolationBecauseItIsTooLong = 'way too long';
}
