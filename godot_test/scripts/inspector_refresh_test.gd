extends Node
## Headless verification of PR #60: Inspector property refresh on compilation errors.
##
## Scripts through the checklist from docs/planning/v0.0.5/INSPECTOR_PROPERTY_FIX.md
## by driving a FerrisScriptNode's script_path property directly and inspecting
## get_property_list() results, since interactive Inspector clicking isn't
## possible headlessly.
##
## Output markers: [TEST_START] / [PASS] / [FAIL] / [TEST_END], consumed by
## the same convention as godot_bind_tests.gd.

var node: Node
var total_tests: int = 0
var passed_tests: int = 0

func _ready():
	print("[TEST_START] inspector_refresh_test")

	node = ClassDB.instantiate("FerrisScriptNode")
	add_child(node)

	# 1. Valid script with @export properties populates the Inspector.
	node.script_path = "res://scripts/inspector_refresh_valid.ferris"
	var props_after_valid = node.get_property_list()
	var ferris_count_valid = count_ferris_properties(props_after_valid)
	check("valid_script_populates_properties", ferris_count_valid > 0,
		"expected >0 exported properties, got %d" % ferris_count_valid)

	# 2. Broken script clears properties (the PR #60 fix under test).
	node.script_path = "res://scripts/type_error.ferris"
	var props_after_error = node.get_property_list()
	var ferris_count_error = count_ferris_properties(props_after_error)
	check("compile_error_clears_properties", ferris_count_error == 0,
		"expected 0 exported properties after compile error, got %d" % ferris_count_error)

	# 3. Fixing the script (switching back to a valid one) repopulates.
	node.script_path = "res://scripts/inspector_refresh_valid.ferris"
	var props_after_fix = node.get_property_list()
	var ferris_count_fix = count_ferris_properties(props_after_fix)
	check("fix_repopulates_properties", ferris_count_fix > 0,
		"expected >0 exported properties after fix, got %d" % ferris_count_fix)

	# 4. Rapid toggling between broken and valid leaves a consistent final state.
	for i in range(3):
		node.script_path = "res://scripts/type_error.ferris"
		node.script_path = "res://scripts/inspector_refresh_valid.ferris"
	var props_after_toggle = node.get_property_list()
	var ferris_count_toggle = count_ferris_properties(props_after_toggle)
	check("rapid_toggle_ends_consistent", ferris_count_toggle == ferris_count_valid,
		"expected %d properties after toggling, got %d" % [ferris_count_valid, ferris_count_toggle])

	# 5. Nonexistent file path clears properties.
	node.script_path = "res://scripts/does_not_exist.ferris"
	var props_after_missing = node.get_property_list()
	var ferris_count_missing = count_ferris_properties(props_after_missing)
	check("missing_file_clears_properties", ferris_count_missing == 0,
		"expected 0 exported properties for missing file, got %d" % ferris_count_missing)

	print("")
	print("[SUMMARY] Total: %d, Passed: %d, Failed: %d" % [total_tests, passed_tests, total_tests - passed_tests])
	print("[TEST_END]")

	get_tree().quit(0 if passed_tests == total_tests else 1)

func count_ferris_properties(props: Array) -> int:
	# inspector_refresh_valid.ferris exports these specific names; matching
	# against the known set is more reliable than guessing from usage flags,
	# since Node2D's own built-in properties also carry PROPERTY_USAGE_EDITOR.
	var ferris_names = ["basic_int", "basic_float"]
	var count = 0
	for prop in props:
		if ferris_names.has(prop.get("name", "")):
			count += 1
	return count

func check(name: String, condition: bool, detail: String):
	total_tests += 1
	print("[TEST] %s" % name)
	if condition:
		passed_tests += 1
		print("[PASS] %s" % name)
	else:
		print("[FAIL] %s: %s" % [name, detail])
