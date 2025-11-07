// This file tests runtime rules: avoid_dynamic, avoid_empty_catch, avoid_print, avoid_null_check_on_nullable
import 'dart:io';

// Test avoid_dynamic rule
class DynamicTypeTest {
  // Bad: using dynamic type (violates avoid_dynamic)
  dynamic dynamicVariable = 'test';
  
  // Good: using specific type
  String typedVariable = 'test';
  
  // Bad: function returning dynamic (violates avoid_dynamic)
  dynamic getDynamicValue() {
    return 'something';
  }
  
  // Good: function returning specific type
  String getTypedValue() {
    return 'something';
  }
}

// Test avoid_empty_catch rule
class EmptyCatchTest {
  void testEmptyCatch() {
    try {
      throw Exception('Test exception');
    } catch (e) {} // Bad: empty catch block (violates avoid_empty_catch)
  }
  
  void testNonEmptyCatch() {
    try {
      throw Exception('Test exception');
    } catch (e) {
      // Good: catch block with handling
      print('Exception caught: $e');
    }
  }
}

// Test avoid_print rule
class PrintTest {
  void testPrint() {
    // Bad: using print in production code (violates avoid_print)
    print('This should be detected');
    print('Another print statement');
  }
  
  void betterLogging() {
    // Good: use proper logging instead
    // logger.info('This is better');
  }
}

// Test avoid_null_check_on_nullable rule
class NullCheckTest {
  void testNullAssertion() {
    String? nullableString;
    
    // Bad: using null assertion operator on nullable (violates avoid_null_check_on_nullable)
    // This is dangerous because it can cause runtime errors
    int length = nullableString!.length;
    
    // Good: proper null handling
    String? anotherNullable;
    int safeLength = anotherNullable?.length ?? 0;
  }
}

void main() {
  // Bad: print in main (violates avoid_print)
  print('Running runtime rules test');
  
  DynamicTypeTest();
  EmptyCatchTest();
  PrintTest();
  NullCheckTest();
}
