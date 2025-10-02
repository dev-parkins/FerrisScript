# Phase 7: _process and self binding - Testing Guide

## Quick Test (2 minutes)

### Test 1: Basic _process callback

1. Open Godot project (`godot_test/project.godot`)
2. Open `test_scene.tscn`
3. Select the `FerrisScriptTest` node
4. In Inspector, change `Script Path` to `res://scripts/process_test.ferris`
5. Press F5 to run
6. **Expected Output (every 60 frames):**
   ```
   Process test started! Counting frames...
   60 frames passed! Delta was: 0.016667
   ```

### Test 2: Object Movement with self.position

1. In the scene, select `FerrisScriptTest` node
2. Change `Script Path` to `res://scripts/move_test.ferris`
3. Make sure the node is visible:
   - Add a Sprite2D or ColorRect as a child to see it move
   - OR note the initial position in the Inspector
4. Press F5 to run
5. **Expected Behavior:**
   - Console shows: "Movement test started! Node will move right."
   - Node moves 50 pixels/second to the right
   - Position.x increases continuously
   - After 10 seconds, should be ~500 pixels to the right

## Acceptance Criteria

### ✅ Phase 7 Complete When:

- [ ] **_process callback works**
  - Script's `_process(delta)` function is called every frame
  - Delta parameter is passed correctly (typically 0.016-0.017 for 60 FPS)

- [ ] **self binding works**
  - Can reference `self` in script functions
  - No errors when accessing `self`

- [ ] **Property getter works**
  - Can read `self.position` from script
  - Returns correct Vector2 value
  - Can access fields: `self.position.x`, `self.position.y`

- [ ] **Property setter works**
  - Can modify `self.position.x` in script
  - Changes are reflected in Godot node
  - Node actually moves on screen
  - `self.position.x += value` syntax works

- [ ] **Performance acceptable**
  - No significant FPS drops
  - Script executes smoothly every frame
  - 60 FPS maintained (check with Godot's FPS counter)

## Detailed Testing Steps

### Setup

1. Close Godot if it's open
2. Rebuild the extension:
   ```powershell
   cargo build --package FerrisScript_godot_bind
   ```
3. Open Godot and import `godot_test/project.godot`

### Test Scenarios

#### Scenario 1: Frame Counting
- Script: `process_test.ferris`
- Purpose: Verify _process is called and delta is valid
- Pass Criteria: See periodic messages (every 60 frames)

#### Scenario 2: Horizontal Movement
- Script: `move_test.ferris`
- Purpose: Verify self.position.x can be modified
- Pass Criteria: Node moves smoothly to the right

#### Scenario 3: Check Position Values
1. Run with `move_test.ferris`
2. Stop after 5 seconds
3. Check Position in Inspector
4. X should be approximately: initial_x + (50 * 5) = initial_x + 250

#### Scenario 4: Verify Property Isolation
1. Add a second FerrisScriptNode to the scene
2. Set both to `move_test.ferris`
3. Run the scene
4. Both nodes should move independently
5. Changing one's position doesn't affect the other

## Troubleshooting

### Node doesn't move
- Check that node type is FerrisScriptNode (not regular Node)
- Verify script path is correct
- Check console for errors
- Ensure extension loaded successfully

### "Property 'position' not supported" error
- FerrisScriptNode must inherit from Node2D
- Rebuild extension if you changed base class
- Restart Godot to reload extension

### FPS drops
- Check Godot's profiler (Debug > Profiler)
- Script execution should be <0.1ms per frame
- If slow, may need optimization in runtime

### Position resets every frame
- Check if script is being reloaded
- Verify environment persists between frames
- Global variables should maintain state

## Success Output Example

```
Movement test started! Node will move right.
```

And in the scene view, you should see the node smoothly moving to the right at 50 pixels per second.

## Next Steps After Validation

Once Phase 7 tests pass:
- Phase 8: Mutable state tracking
- Phase 9: Polish and documentation
- 0.0.1 release!

