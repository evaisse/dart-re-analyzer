#!/bin/bash

echo "====================================="
echo "Testing dart-re-analyzer on test_project"
echo "====================================="

# Build the analyzer
echo "Building dart-re-analyzer..."
cargo build --release

# Path to the analyzer
ANALYZER="./target/release/dart-re-analyzer"

# Test 1: Basic analyze command
echo ""
echo "Test 1: Basic analyze command"
echo "-------------------------------------"
$ANALYZER analyze test_project || true

# Test 2: Analyze with JSON output
echo ""
echo "Test 2: JSON output format"
echo "-------------------------------------"
OUTPUT_JSON=$($ANALYZER analyze test_project --format json || true)
echo "$OUTPUT_JSON" | jq '.' 2>/dev/null || echo "$OUTPUT_JSON"

# Test 3: Style rules only
echo ""
echo "Test 3: Style rules only"
echo "-------------------------------------"
$ANALYZER analyze test_project --style-only || true

# Test 4: Runtime rules only
echo ""
echo "Test 4: Runtime rules only"
echo "-------------------------------------"
$ANALYZER analyze test_project --runtime-only || true

# Test 5: Init config
echo ""
echo "Test 5: Init config command"
echo "-------------------------------------"
(cd test_project && ../$ANALYZER init-config)
if [ -f "test_project/analyzer_config.json" ]; then
    echo "✓ Config file created successfully"
    cat test_project/analyzer_config.json
    rm test_project/analyzer_config.json  # Clean up
else
    echo "✗ Config file was not created"
    exit 1
fi

# Test 6: Analyze with custom config
echo ""
echo "Test 6: Analyze with custom config"
echo "-------------------------------------"
cat > test_project/custom_config.json <<EOF
{
  "enabled": true,
  "exclude_patterns": [
    ".dart_tool/**",
    "build/**"
  ],
  "style_rules": {
    "enabled": true,
    "disabled_rules": []
  },
  "runtime_rules": {
    "enabled": true,
    "disabled_rules": ["avoid_print"]
  },
  "max_line_length": 100,
  "parallel": true
}
EOF
$ANALYZER analyze test_project --config test_project/custom_config.json || true
rm test_project/custom_config.json

# Validate that we found expected issues
echo ""
echo "====================================="
echo "Validation"
echo "====================================="

# Run analyzer and capture output
FULL_OUTPUT=$($ANALYZER analyze test_project 2>&1 || true)

# Check for expected rule violations
EXPECTED_RULES=(
    "camel_case_class_names"
    "snake_case_file_names"
    "line_length"
    "avoid_dynamic"
    "avoid_empty_catch"
    "avoid_print"
    "unused_import"
    "avoid_null_check_on_nullable"
)

echo "Checking for expected rule violations..."
ALL_FOUND=true

for rule in "${EXPECTED_RULES[@]}"; do
    if echo "$FULL_OUTPUT" | grep -q "$rule"; then
        echo "✓ Found $rule"
    else
        echo "✗ Missing $rule"
        ALL_FOUND=false
    fi
done

if [ "$ALL_FOUND" = true ]; then
    echo ""
    echo "====================================="
    echo "✓ All tests passed!"
    echo "====================================="
    exit 0
else
    echo ""
    echo "====================================="
    echo "✗ Some tests failed!"
    echo "====================================="
    echo ""
    echo "Full output:"
    echo "$FULL_OUTPUT"
    exit 1
fi
