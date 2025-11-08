// This file tests unused_import rule
import 'dart:async';  // Good: used below
import 'dart:io';     // Bad: unused import (violates unused_import rule)
import 'dart:convert'; // Bad: unused import (violates unused_import rule)

class ImportTest {
  // Using Future from dart:async
  Future<String> fetchData() async {
    await Future.delayed(Duration(seconds: 1));
    return 'data';
  }
}

void main() {
  ImportTest();
}
