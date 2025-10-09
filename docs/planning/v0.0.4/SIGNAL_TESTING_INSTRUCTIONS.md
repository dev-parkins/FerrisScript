# Signal Testing Instructions

**Date**: October 8, 2025  
**Purpose**: Verify FerrisScript signal functionality without editor UI  
**Status**: Ready for testing

---

## 🎯 Overview

FerrisScript signals are **registered and functional**, but don't appear in Godot's Node→Signals panel due to dynamic registration. This test verifies they work via programmatic connection.

---

## 📝 Test Setup

### Files Required

1. ✅ `godot_test/scripts/signal_test.ferris` (FerrisScript with signals)
2. ✅ `godot_test/scripts/receiver.gd` (GDScript receiver)
3. ✅ Updated `ferrisscript_godot_bind.dll` (just rebuilt)

### Scene Setup

1. **Open Godot** (`godot_test/project.godot`)

2. **Create Test Scene**:
   - Create new scene or use existing test scene
   - Add a **Node2D** (name it "FerrisScriptNode")
   - Attach script: `signal_test.ferris`

3. **Add Receiver Node**:
   - Add a **Node** (name it "SignalReceiver")
   - Attach script: `receiver.gd`
   - **Important**: Adjust the node path in `receiver.gd` line 6 to match your scene structure

4. **Adjust Path if Needed**:
   ```gdscript
   # In receiver.gd, line 6
   var ferris_node = get_node_or_null("../FerrisScriptNode")
   ```
   Change `"../FerrisScriptNode"` to match your node hierarchy.

---

## 🧪 Test Procedure

### Step 1: Run the Scene

- Press **F5** (or click "Run Scene")
- Watch the **Output** panel (bottom of Godot editor)

### Step 2: Verify Initial Output

You should see:
```
Successfully loaded FerrisScript: res://scripts/signal_test.ferris
Registered signal: health_changed
Registered signal: player_died
Registered signal: score_updated
Signal Test Ready!
Available signals: health_changed, player_died, score_updated

=== Signal Receiver Ready ===
✅ Connected to health_changed signal
✅ Connected to player_died signal
✅ Connected to score_updated signal
=== Ready to receive signals ===
```

### Step 3: Verify Signal Emissions

After 1 second, you should see:
```
=== Testing signal emissions ===
Called take_damage(25)
📡 SIGNAL RECEIVED: health_changed(100, 75)

Called add_score(100)
📡 SIGNAL RECEIVED: score_updated(100)

Called take_damage(150) - should trigger player_died
📡 SIGNAL RECEIVED: health_changed(100, -50)
📡 SIGNAL RECEIVED: player_died()
```

---

## ✅ Expected Results

### Success Criteria

| Test | Expected Result | Pass/Fail |
|------|----------------|-----------|
| Script loads | "Successfully loaded FerrisScript" | ⬜ |
| Signals registered | 3 "Registered signal" messages | ⬜ |
| Connections succeed | 3 "✅ Connected" messages | ⬜ |
| `take_damage(25)` emits | `health_changed(100, 75)` received | ⬜ |
| `add_score(100)` emits | `score_updated(100)` received | ⬜ |
| `take_damage(150)` emits | Both `health_changed` and `player_died` received | ⬜ |

---

## 🐛 Troubleshooting

### Error: "FerrisScriptNode not found"

**Cause**: Node path in `receiver.gd` doesn't match scene structure

**Fix**:
1. Check your scene tree
2. Update line 6 in `receiver.gd`:
   ```gdscript
   var ferris_node = get_node_or_null("/root/NodeName/FerrisScriptNode")
   ```
   Use absolute path or adjust relative path

---

### Error: "Failed to connect to [signal]"

**Possible Causes**:
1. Signal not registered (check "Registered signal" messages)
2. FerrisScript didn't load (check "Successfully loaded" message)
3. Godot version incompatibility

**Debug Steps**:
1. Verify signal registration in console
2. Check for script compilation errors
3. Ensure GDExtension loaded (no errors about `classdb_register_extension_class5`)

---

### No Signal Received

**Possible Causes**:
1. Connection failed (check for ❌ messages)
2. Function not called (check "Called" messages)
3. Logic error in FerrisScript

**Debug Steps**:
1. Add print statements in FerrisScript functions
2. Verify signal emission reaches `emit_signal()` call
3. Check parameter types match signal declaration

---

## 📊 Results Template

```
=== TEST RESULTS ===
Date: [Date]
Godot Version: [Version]
FerrisScript Version: v0.0.4-dev

✅ Script Loading: PASS/FAIL
✅ Signal Registration: PASS/FAIL (3/3 signals)
✅ Signal Connections: PASS/FAIL (3/3 connections)
✅ Signal Emissions: PASS/FAIL
   - health_changed: PASS/FAIL
   - player_died: PASS/FAIL
   - score_updated: PASS/FAIL

Notes:
[Any additional observations]
```

---

## 🎯 Next Steps After Testing

### If All Tests Pass ✅

1. Document results in Phase 1 completion report
2. Update PR with test verification
3. Note editor UI limitation in documentation
4. Proceed to Phase 2

### If Tests Fail ❌

1. Document specific failures
2. Check Godot console for errors
3. Verify GDExtension loaded correctly
4. Report findings for debugging

---

## 📝 Additional Test Cases

### Manual Test: Direct Function Call

In Godot's script editor debugger, you can also test directly:

1. Set a breakpoint in `receiver.gd`
2. Run scene
3. In debugger console:
   ```gdscript
   var node = get_node("../FerrisScriptNode")
   node.call_ferris_function("take_damage", [10])
   ```

---

## 🔍 Understanding the Flow

```
FerrisScript (signal_test.ferris)
    ↓
  signal health_changed(old: i32, new: i32);
    ↓
  take_damage(25) → emit_signal("health_changed", 100, 75)
    ↓
  Runtime (lib.rs) → set_signal_emitter callback
    ↓
  Godot Node2D.emit_signal("health_changed", [100, 75])
    ↓
  GDScript (receiver.gd) → _on_health_changed(100, 75)
    ↓
  Console: "📡 SIGNAL RECEIVED: health_changed(100, 75)"
```

---

**Ready to test!** Run the scene and report results. ✅
