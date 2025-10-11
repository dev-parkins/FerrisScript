# Inspector Integration Testing Guide

**File**: `examples/inspector_test.ferris`  
**Version**: v0.0.4  
**Date**: October 10, 2025

---

## Quick Start

### 1. Compile FerrisScript
```powershell
cargo build --package ferrisscript_godot_bind
```

### 2. Open Godot Editor
- Open your Godot project (e.g., `godot_test/project.godot`)
- Create a new scene with Node2D as root

### 3. Attach Script
1. Select the Node2D
2. In Inspector, click "Attach Script"
3. Choose "FerrisScriptNode" (the custom script type)
4. Set "Script Path" to: `res://examples/inspector_test.ferris`
5. Save the scene

### 4. View Properties
The Inspector should now show **20+ exported properties** organized by type:

---

## Test File Contents

### Property Types Tested (8 types)

#### Basic Types:
- ✅ `i32` - player_health (100)
- ✅ `f32` - movement_speed (5.5)
- ✅ `bool` - is_alive (true)
- ✅ `String` - player_name ("Hero")

#### Godot Types:
- ✅ `Vector2` - spawn_position (100.0, 200.0)
- ✅ `Color` - tint_color (RGBA)
- ✅ `Rect2` - collision_rect (position + size)
- ✅ `Transform2D` - spawn_transform (position + rotation + scale)

### Property Hints Tested (4 hints)

#### Range Hints (5 examples):
```ferris
@export(range(0, 100, 1))
let mut stamina: i32 = 50;           // Slider 0-100, step 1

@export(range(0.0, 10.0, 0.5))
let mut run_speed: f32 = 7.5;        // Slider 0.0-10.0, step 0.5

@export(range(-100, 100, 10))
let mut temperature: i32 = 20;       // Slider -100 to 100, step 10

@export(range(0, 360, 1))
let mut rotation_degrees: i32 = 0;   // Slider 0-360 (rotation)

@export(range(0.0, 1.0, 0.1))
let mut opacity: f32 = 1.0;          // Slider 0.0-1.0 (percentage)
```

#### Enum Hints (3 examples):
```ferris
@export(enum("Warrior", "Mage", "Rogue", "Ranger"))
let mut character_class: String = "Warrior";  // Dropdown menu

@export(enum("Easy", "Normal", "Hard", "Nightmare"))
let mut difficulty: String = "Normal";        // Dropdown menu

@export(enum("Red", "Green", "Blue", "Yellow"))
let mut team_color: String = "Red";           // Dropdown menu
```

#### File Hints (3 examples):
```ferris
@export(file("*.png", "*.jpg", "*.jpeg"))
let mut avatar_texture: String = "";   // File picker for images

@export(file("*.ogg", "*.wav", "*.mp3"))
let mut sound_effect: String = "";     // File picker for audio

@export(file("*.tscn", "*.scn"))
let mut spawn_scene: String = "";      // File picker for scenes
```

---

## Testing Checklist

### ✅ Property Display Test
1. Open scene with FerrisScriptNode
2. Check Inspector panel
3. **Expected**: All 20+ properties visible
4. **Expected**: Properties show correct default values
5. **Expected**: Range properties show as sliders
6. **Expected**: Enum properties show as dropdowns
7. **Expected**: File properties show file picker button

### ✅ Property Reading Test
1. Look at Inspector values
2. Run scene (F5)
3. Check console output
4. **Expected**: Console prints all property values
5. **Expected**: Values match Inspector display

### ✅ Property Writing Test
1. In Inspector, change `player_health` to 75
2. Run scene (F5)
3. **Expected**: Console prints "Health: 75"
4. Stop scene (F8)
5. Change `character_class` to "Mage"
6. Run scene
7. **Expected**: Console prints "Class: Mage"

### ✅ Range Clamping Test
1. In Inspector, set `stamina` to 150 (above max)
2. **Expected**: Value clamps to 100
3. Set `stamina` to -10 (below min)
4. **Expected**: Value clamps to 0
5. Set `opacity` to 1.5 (above max)
6. **Expected**: Value clamps to 1.0
7. Set `temperature` to 200 (above max)
8. **Expected**: Value clamps to 100

### ✅ Enum Validation Test
1. Change `character_class` dropdown
2. **Expected**: Only shows "Warrior", "Mage", "Rogue", "Ranger"
3. Change `difficulty` dropdown
4. **Expected**: Only shows "Easy", "Normal", "Hard", "Nightmare"
5. **Expected**: Can't enter arbitrary text

### ✅ File Picker Test
1. Click file picker button for `avatar_texture`
2. **Expected**: File dialog opens
3. **Expected**: Filter shows only .png, .jpg, .jpeg
4. Select a file
5. **Expected**: Path appears in Inspector
6. Run scene
7. **Expected**: Console prints file path

### ✅ Struct Type Test
1. Expand `spawn_position` property
2. **Expected**: Shows X and Y fields
3. Change X to 50, Y to 100
4. Run scene
5. **Expected**: Console prints "Position X: 50 Y: 100"
6. Expand `tint_color` property
7. **Expected**: Shows R, G, B, A fields (color picker)
8. Change color
9. **Expected**: New values visible in console

### ✅ Hot-Reload Test
1. Keep Inspector open
2. Open `examples/inspector_test.ferris` in text editor
3. Change `player_health` default from 100 to 200
4. Save file
5. **Expected**: Inspector updates automatically (no scene reload needed)
6. **Expected**: Inspector now shows player_health = 200

### ✅ Runtime Update Test
1. Run scene (F5)
2. Watch console output
3. **Expected**: `rotation_degrees` increments every frame (0→360)
4. **Expected**: `stamina` regenerates to 100
5. Let run for 10 seconds
6. **Expected**: No crashes, smooth updates

### ✅ Built-in Properties Test
1. In Inspector, scroll to find Node2D built-in properties
2. Find "Transform" → "Position"
3. Change Position X to 100
4. **Expected**: Node moves in scene view
5. **Expected**: FerrisScript properties still work
6. **Expected**: No conflicts between systems

### ✅ Error Handling Test
1. Try to set invalid property name (via code)
2. **Expected**: Graceful error message, no crash
3. Try to set wrong type (String to i32)
4. **Expected**: Error logged, fallback to Godot
5. **Expected**: Editor remains stable

---

## Expected Console Output

When you run the scene (F5), you should see:

```
=== Inspector Test Started ===

--- Basic Properties ---
Health: 100
Speed: 5.5
Alive: true
Name: Hero

--- Range Properties ---
Stamina: 50
Run Speed: 7.5
Temperature: 20
Rotation: 0
Opacity: 1.0

--- Enum Properties ---
Class: Warrior
Difficulty: Normal
Team Color: Red

--- File Properties ---
Avatar Texture: (not set)
Sound Effect: (not set)

--- Godot Struct Properties ---
Spawn Position X: 100.0 Y: 200.0
Tint Color R: 1.0 G: 0.5 B: 0.0
Collision Rect Size: 64.0x64.0

=== Ready to Test Inspector Changes ===

(Then rotation_degrees increments every frame...)
```

---

## Common Issues & Solutions

### ❌ Properties Not Visible in Inspector

**Symptom**: Inspector doesn't show exported properties

**Solutions**:
1. Check `#[class(tool)]` annotation in `crates/godot_bind/src/lib.rs` line 357
   ```rust
   #[derive(GodotClass)]
   #[class(base=Node2D, tool)]  // ← Must have 'tool'
   pub struct FerrisScriptNode { ... }
   ```

2. Verify script path is correct:
   - Should be `res://examples/inspector_test.ferris`
   - NOT an absolute path like `C:\...`

3. Check console for compilation errors:
   - Open Godot console (Output panel)
   - Look for FerrisScript compilation errors
   - Fix syntax errors and reload

### ❌ Inspector Changes Don't Save

**Symptom**: Change property in Inspector, but value doesn't persist

**Solutions**:
1. Verify properties use `let mut` (mutable):
   ```ferris
   @export
   let mut health: i32 = 100;  // ✅ Correct
   
   @export
   let health: i32 = 100;      // ❌ Wrong (immutable)
   ```

2. Check `set_property()` returns true in `godot_bind/src/lib.rs`

3. Look for errors in Godot console output

### ❌ Range Clamping Not Working

**Symptom**: Can set values outside range (e.g., health = 150 when max is 100)

**Solutions**:
1. Verify `from_inspector=true` in `set_property()` call
2. Check range hint syntax:
   ```ferris
   @export(range(0, 100, 1))  // ✅ Correct format
   @export(range(0, 100))     // ❌ Missing step
   ```

3. Ensure type matches hint:
   ```ferris
   @export(range(0, 100, 1))
   let mut health: i32 = 50;   // ✅ i32 compatible with range
   
   @export(range(0, 100, 1))
   let mut name: String = "";  // ❌ String not compatible
   ```

### ❌ Hot-Reload Not Working

**Symptom**: Change script file, but Inspector doesn't update

**Solutions**:
1. Verify `notify_property_list_changed()` called in `load_script()`
   - Check `godot_bind/src/lib.rs` around line 703

2. Script path must be `res://` path (not absolute):
   - ✅ `res://examples/inspector_test.ferris`
   - ❌ `C:\Projects\FerrisScript\examples\inspector_test.ferris`

3. Try manual refresh:
   - Close and reopen Inspector panel
   - Or close and reopen scene

### ❌ Enum Dropdown Shows Wrong Values

**Symptom**: Dropdown shows incorrect options or empty

**Solutions**:
1. Check enum hint syntax (no quotes in compiled output):
   ```ferris
   @export(enum("Easy", "Medium", "Hard"))  // ✅ Correct
   ```

2. Ensure at least one value provided:
   ```ferris
   @export(enum())  // ❌ Compile error (empty enum)
   ```

3. Check for duplicate values:
   ```ferris
   @export(enum("Easy", "Easy"))  // ❌ Compile warning
   ```

### ❌ Type Conversion Errors

**Symptom**: Console shows "Failed to set FerrisScript property" errors

**Solutions**:
1. Ensure property type matches expected type
2. Check `variant_to_value()` in `godot_bind/src/lib.rs`
3. Look for NaN/Infinity edge cases with floats
4. Verify struct literal syntax for Godot types

---

## Performance Expectations

- **Property Read**: < 1 microsecond (HashMap lookup)
- **Property Write**: < 5 microseconds (includes range clamping)
- **Property List Generation**: < 100 microseconds (for 100 properties)
- **Inspector Interaction**: No noticeable lag

If you experience lag, check:
- Number of properties (100+ may slow down Inspector)
- Complex default values (nested struct literals)
- Console spam (excessive print statements)

---

## Advanced Testing

### Test Coverage Achieved

- ✅ 8 property types (i32, f32, bool, String, Vector2, Color, Rect2, Transform2D)
- ✅ 4 property hints (None, Range, Enum, File)
- ✅ 5 range variations (positive, negative, float, rotation, percentage)
- ✅ 3 enum examples (class, difficulty, color)
- ✅ 3 file picker types (image, audio, scene)
- ✅ Lifecycle callbacks (_ready, _process)
- ✅ Runtime property modification
- ✅ Hot-reload behavior
- ✅ Error handling and graceful degradation

### Manual Test Results

Record your test results:

```
[ ] Property Display: PASS / FAIL
[ ] Property Reading: PASS / FAIL
[ ] Property Writing: PASS / FAIL
[ ] Range Clamping: PASS / FAIL
[ ] Enum Validation: PASS / FAIL
[ ] File Picker: PASS / FAIL
[ ] Struct Types: PASS / FAIL
[ ] Hot-Reload: PASS / FAIL
[ ] Runtime Updates: PASS / FAIL
[ ] Built-in Properties: PASS / FAIL
[ ] Error Handling: PASS / FAIL

Overall: PASS / FAIL

Notes:
_________________________________
_________________________________
_________________________________
```

---

## Next Steps After Testing

1. **If All Tests Pass**: Inspector integration is ready for v0.0.4 release! 🎉

2. **If Issues Found**: 
   - Document specific failures
   - Check troubleshooting section
   - Review `docs/planning/v0.0.4/TESTING_STRATEGY_PHASE5.md` for advanced debugging

3. **Report Results**:
   - Update test results in this file
   - Add findings to `docs/LEARNINGS.md`
   - Create issues for any bugs found

---

## Related Documentation

- **Implementation**: `docs/archive/v0.0.4/SESSION_SUMMARY_BUNDLES_7-8.md`
- **Testing Strategy**: `docs/planning/v0.0.4/TESTING_STRATEGY_PHASE5.md`
- **API Details**: `docs/archive/v0.0.4/BUNDLE_7_IMPLEMENTATION_PLAN.md`
- **Integration Tests**: `crates/runtime/tests/inspector_sync_test.rs`

---

**Good luck with your testing!** 🦀🎮

If you encounter any issues not covered here, check the Godot console output first - it usually provides helpful error messages.
