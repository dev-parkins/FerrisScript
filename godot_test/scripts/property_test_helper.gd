extends Node2D
# GDScript Test Helper for FerrisScript Exported Properties
# This script tests the PropertyInfo integration from Godot's side

func _ready():
	print("=== GDScript PropertyInfo Integration Test ===")
	test_property_list()
	test_property_get_set()
	test_inspector_clamping()
	print("=== GDScript Tests Complete ===")

# Test 1: Verify get_property_list() returns correct PropertyInfo
func test_property_list():
	print("\n--- Test 1: Property List Verification ---")
	
	var properties = get_property_list()
	print("Total properties: ", properties.size())
	
	# Filter to only FerrisScript exported properties (exclude engine properties)
	var ferris_properties = []
	for prop in properties:
		var name = prop["name"]
		# Our test properties start with specific prefixes
		if name.begins_with("basic_") or name.begins_with("health") or \
		   name.begins_with("speed") or name.begins_with("size") or \
		   name == "position" or name == "color":
			ferris_properties.append(prop)
	
	print("FerrisScript exported properties found: ", ferris_properties.size())
	
	# Verify specific properties
	for prop in ferris_properties:
		verify_property_info(prop)

func verify_property_info(prop: Dictionary):
	var name = prop["name"]
	var type = prop["type"]
	var hint = prop["hint"]
	var hint_string = prop["hint_string"]
	var usage = prop["usage"]
	
	print("\nProperty: ", name)
	print("  Type: ", type, " (", type_string(type), ")")
	print("  Hint: ", hint, " (", hint_string(hint), ")")
	print("  Hint String: ", hint_string)
	print("  Usage: ", usage)
	
	# Verify expected types
	match name:
		"basic_int", "health", "temperature", "rotation_degrees":
			assert(type == TYPE_INT, "Expected INT type for " + name)
		"basic_float", "speed", "opacity":
			assert(type == TYPE_FLOAT, "Expected FLOAT type for " + name)
		"basic_bool", "test_passed":
			assert(type == TYPE_BOOL, "Expected BOOL type for " + name)
		"basic_string", "size", "color_name", "texture_path", "resource_path", "animation_state":
			assert(type == TYPE_STRING, "Expected STRING type for " + name)
		"position":
			assert(type == TYPE_VECTOR2, "Expected VECTOR2 type for position")
		"color":
			assert(type == TYPE_COLOR, "Expected COLOR type for color")
		"bounds":
			assert(type == TYPE_RECT2, "Expected RECT2 type for bounds")
		"transform":
			assert(type == TYPE_TRANSFORM2D, "Expected TRANSFORM2D type for transform")
	
	# Verify expected hints
	match name:
		"health", "speed", "temperature", "rotation_degrees", "opacity":
			assert(hint == PROPERTY_HINT_RANGE, "Expected RANGE hint for " + name)
			assert(hint_string != "", "Expected non-empty hint string for range")
		"size", "color_name", "animation_state":
			assert(hint == PROPERTY_HINT_ENUM, "Expected ENUM hint for " + name)
			assert(hint_string.contains(","), "Expected comma-separated enum values")
		"texture_path", "resource_path":
			assert(hint == PROPERTY_HINT_FILE, "Expected FILE hint for " + name)
			assert(hint_string.contains("*."), "Expected file extension pattern")
	
	print("  ✓ Verified")

func type_string(type: int) -> String:
	match type:
		TYPE_NIL: return "NIL"
		TYPE_BOOL: return "BOOL"
		TYPE_INT: return "INT"
		TYPE_FLOAT: return "FLOAT"
		TYPE_STRING: return "STRING"
		TYPE_VECTOR2: return "VECTOR2"
		TYPE_RECT2: return "RECT2"
		TYPE_VECTOR3: return "VECTOR3"
		TYPE_TRANSFORM2D: return "TRANSFORM2D"
		TYPE_PLANE: return "PLANE"
		TYPE_QUATERNION: return "QUATERNION"
		TYPE_AABB: return "AABB"
		TYPE_BASIS: return "BASIS"
		TYPE_TRANSFORM3D: return "TRANSFORM3D"
		TYPE_COLOR: return "COLOR"
		TYPE_OBJECT: return "OBJECT"
		_: return "UNKNOWN"

func hint_string(hint: int) -> String:
	match hint:
		PROPERTY_HINT_NONE: return "NONE"
		PROPERTY_HINT_RANGE: return "RANGE"
		PROPERTY_HINT_ENUM: return "ENUM"
		PROPERTY_HINT_FILE: return "FILE"
		PROPERTY_HINT_DIR: return "DIR"
		PROPERTY_HINT_GLOBAL_FILE: return "GLOBAL_FILE"
		PROPERTY_HINT_RESOURCE_TYPE: return "RESOURCE_TYPE"
		PROPERTY_HINT_MULTILINE_TEXT: return "MULTILINE_TEXT"
		PROPERTY_HINT_PLACEHOLDER_TEXT: return "PLACEHOLDER_TEXT"
		_: return "OTHER"

# Test 2: Verify get/set work correctly
func test_property_get_set():
	print("\n--- Test 2: Property Get/Set Verification ---")
	
	# Test basic types
	if has_method("set") and has_method("get"):
		# Set a property
		set("basic_int", 100)
		var value = get("basic_int")
		print("Set basic_int to 100, got: ", value)
		assert(value == 100, "Property get/set failed for basic_int")
		
		# Set a Vector2 property
		var new_pos = Vector2(10.0, 20.0)
		set("position", new_pos)
		var pos_value = get("position")
		print("Set position to (10, 20), got: ", pos_value)
		assert(pos_value == new_pos, "Property get/set failed for position")
		
		print("✓ Get/Set working correctly")
	else:
		print("⚠ Skipping get/set test (methods not available)")

# Test 3: Verify Inspector clamping behavior
func test_inspector_clamping():
	print("\n--- Test 3: Inspector Clamping Verification ---")
	print("Note: Automatic clamping tested by setting properties from Inspector")
	print("Manual test: ")
	print("  1. Open scene in Godot Editor")
	print("  2. Select FerrisScriptNode")
	print("  3. Try setting 'health' to 150 in Inspector")
	print("  4. Verify it clamps to 100")
	print("  5. Try setting 'health' to -20 in Inspector")
	print("  6. Verify it clamps to 0")
	
	# Automated test: Set from script (should warn, not clamp)
	if has_method("set"):
		print("\nTesting script set (should not clamp):")
		set("health", 150)
		var health_value = get("health")
		print("Set health to 150 from script, result: ", health_value)
		if health_value == 150:
			print("✓ Script set does not clamp (expected)")
		elif health_value == 100:
			print("✗ Script set clamped (unexpected - should only warn)")
		
		# Reset to valid value
		set("health", 75)

# Test 4: Verify all 8 exportable types
func test_all_types():
	print("\n--- Test 4: All Exportable Types ---")
	var type_tests = [
		["basic_int", TYPE_INT],
		["basic_float", TYPE_FLOAT],
		["basic_bool", TYPE_BOOL],
		["basic_string", TYPE_STRING],
		["position", TYPE_VECTOR2],
		["color", TYPE_COLOR],
		["bounds", TYPE_RECT2],
		["transform", TYPE_TRANSFORM2D]
	]
	
	for test in type_tests:
		var prop_name = test[0]
		var expected_type = test[1]
		
		if has_method("get"):
			var value = get(prop_name)
			var actual_type = typeof(value)
			print(prop_name, ": ", value, " (type ", actual_type, ")")
			assert(actual_type == expected_type, 
				"Type mismatch for " + prop_name + ": expected " + str(expected_type) + ", got " + str(actual_type))
	
	print("✓ All 8 types verified")

# Test 5: Verify all 4 hint types
func test_all_hints():
	print("\n--- Test 5: All Hint Types ---")
	var hint_tests = [
		["health", PROPERTY_HINT_RANGE],
		["size", PROPERTY_HINT_ENUM],
		["texture_path", PROPERTY_HINT_FILE],
		["basic_int", PROPERTY_HINT_NONE]
	]
	
	var properties = get_property_list()
	for test in hint_tests:
		var prop_name = test[0]
		var expected_hint = test[1]
		
		for prop in properties:
			if prop["name"] == prop_name:
				var actual_hint = prop["hint"]
				print(prop_name, " hint: ", actual_hint, " (", hint_string(actual_hint), ")")
				assert(actual_hint == expected_hint,
					"Hint mismatch for " + prop_name + ": expected " + str(expected_hint) + ", got " + str(actual_hint))
				break
	
	print("✓ All 4 hint types verified")

# Manual test function to call from console
func run_all_tests():
	_ready()

# Utility to print all properties in detail
func print_all_properties():
	print("\n=== All Properties (Detailed) ===")
	var properties = get_property_list()
	for prop in properties:
		if prop["name"].begins_with("_"):
			continue  # Skip internal properties
		print("\n", prop["name"], ":")
		for key in prop.keys():
			print("  ", key, ": ", prop[key])
