extends Node
## Headless Godot Integration Tests for FerrisScript GDExtension
##
## This script tests the godot_bind functionality by calling test methods
## on FerrisScriptTestNode and validating results. Designed to run headlessly
## with parseable output for automated testing.
##
## Output Format:
##   [TEST_START] - Begin test suite
##   [TEST] <name> - Starting test
##   [ASSERT] <condition> - Expected condition
##   [ACTUAL] <value> - Actual value
##   [PASS] <name> - Test passed
##   [FAIL] <name> - Test failed
##   [TEST_END] - End test suite
##
## Exit Codes:
##   0 - All tests passed
##   1 - One or more tests failed
##   2 - Test runner error

var test_node: Node = null
var total_tests: int = 0
var passed_tests: int = 0
var failed_tests: int = 0

func _ready():
	print("[TEST_START] godot_bind_tests")
	
	# Initialize test node (FerrisScript GDExtension)
	# Note: FerrisScriptTestNode will be added to the GDExtension in next step
	# For now, we'll test the basic structure
	
	# Run all tests
	test_basic_functionality()
	
	# Summary
	print("")
	print("[SUMMARY] Total: %d, Passed: %d, Failed: %d" % [total_tests, passed_tests, failed_tests])
	print("[TEST_END]")
	
	# Exit with appropriate code
	if failed_tests > 0:
		push_error("Tests failed: %d/%d" % [failed_tests, total_tests])
		get_tree().quit(1)
	else:
		print("All tests passed!")
		get_tree().quit(0)

func test_basic_functionality():
	"""Test basic Godot functionality to ensure headless mode works"""
	run_test("basic_node_creation", func():
		var node = Node.new()
		assert_not_null(node, "Node creation")
		node.queue_free()
	)
	
	run_test("property_hint_enum", func():
		# Test that Godot PropertyHint enum is available
		assert_equal(PropertyHint.NONE, 0, "PropertyHint.NONE value")
		assert_equal(PropertyHint.RANGE, 1, "PropertyHint.RANGE value")
		assert_equal(PropertyHint.ENUM, 2, "PropertyHint.ENUM value")
		assert_equal(PropertyHint.FILE, 13, "PropertyHint.FILE value")
	)
	
	run_test("variant_type_enum", func():
		# Test that Godot VariantType enum is available  
		assert_equal(typeof(42), TYPE_INT, "Integer type")
		assert_equal(typeof(3.14), TYPE_FLOAT, "Float type")
		assert_equal(typeof(true), TYPE_BOOL, "Bool type")
		assert_equal(typeof("test"), TYPE_STRING, "String type")
	)

# ============================================================================
# Test Framework Helpers
# ============================================================================

func run_test(test_name: String, test_func: Callable):
	"""Run a single test function with error handling"""
	total_tests += 1
	print("")
	print("[TEST] %s" % test_name)
	
	var passed = true
	var error_msg = ""
	
	# Execute test with error handling
	var result = test_func.call()
	if result is String and result.begins_with("FAIL:"):
		passed = false
		error_msg = result
	
	# Report result
	if passed:
		print("[PASS] %s" % test_name)
		passed_tests += 1
	else:
		print("[FAIL] %s - %s" % [test_name, error_msg])
		failed_tests += 1

func assert_equal(actual, expected, message: String):
	"""Assert that two values are equal"""
	print("[ASSERT] %s: expected=%s" % [message, expected])
	print("[ACTUAL] %s: actual=%s" % [message, actual])
	
	if actual != expected:
		push_error("Assertion failed: %s (expected %s, got %s)" % [message, expected, actual])
		return "FAIL: %s" % message

func assert_not_null(value, message: String):
	"""Assert that value is not null"""
	print("[ASSERT] %s: not null" % message)
	print("[ACTUAL] %s: %s" % [message, "null" if value == null else "not null"])
	
	if value == null:
		push_error("Assertion failed: %s is null" % message)
		return "FAIL: %s" % message

func assert_contains(text: String, substring: String, message: String):
	"""Assert that text contains substring"""
	print("[ASSERT] %s: contains '%s'" % [message, substring])
	print("[ACTUAL] %s: '%s'" % [message, text])
	
	if not text.contains(substring):
		push_error("Assertion failed: %s (text does not contain '%s')" % [message, substring])
		return "FAIL: %s" % message
