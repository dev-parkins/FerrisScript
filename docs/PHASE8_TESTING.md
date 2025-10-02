# Phase 8 Testing Guide: Mutable State & Control Flow

**Purpose**: Validate that mutable variables persist between frames and control flow works correctly in RustyScript

**Last Updated**: October 1, 2025

---

## 📋 Quick Test (2 minutes)

### Setup
1. Open Godot 4.2+ project from `godot_test/`
2. Open the test scene (or create a new one)
3. Add a `RustyScriptNode` (extends Node2D)
4. Set `script_path` to `res://scripts/bounce_test.rscr`
5. Add a Sprite2D or ColorRect as child (for visualization)

### Expected Behavior
- Node starts at x=0, moves right
- At x=200, reverses and moves left
- At x=-200, reverses and moves right
- Continues bouncing indefinitely
- Movement is smooth without stuttering

### Quick Validation
Run the scene (F5) and observe:
- ✅ Console shows "Bounce test started" message
- ✅ Node moves horizontally
- ✅ Direction reverses at boundaries
- ✅ Bouncing continues indefinitely

---

## 📊 Comprehensive Test Suite

### Test 1: Mutable State Persistence
**Acceptance Criteria:**
- [ ] Global `dir` variable maintains value between frames
- [ ] Direction changes persist across multiple _process calls
- [ ] No reset of state when switching between boundaries

**How to Test:**
1. Run scene and watch node movement
2. Direction should only change at boundaries
3. Add a debug print of `dir` value (modify script to print direction)
4. Verify direction is -1.0 when moving left, 1.0 when moving right

**Expected Output Pattern:**
```
Bounce test started
Node will bounce between x = -200 and x = 200
[Movement occurs continuously without console spam]
```

### Test 2: Control Flow (If Statements)
**Acceptance Criteria:**
- [ ] If statement `self.position.x > boundary` works correctly
- [ ] If statement `self.position.x < -boundary` works correctly
- [ ] Both conditions can be true in sequence (no mutual exclusion bug)

**How to Test:**
1. Run scene
2. Verify reversal at right boundary (x > 200)
3. Verify reversal at left boundary (x < -200)
4. Watch for multiple complete bounces

**Visual Confirmation:**
- Node should bounce at BOTH boundaries
- Should not get stuck at one boundary
- Should not overshoot boundaries significantly

### Test 3: Self Property Modification
**Acceptance Criteria:**
- [ ] `self.position.x += ...` modifies node position
- [ ] Position updates are reflected in visual node position
- [ ] No console errors about property access

**How to Test:**
1. Run scene with visual child (Sprite2D/ColorRect)
2. Visual element should move smoothly
3. Check Inspector during play - position.x should update

**Expected Behavior:**
- Smooth horizontal movement
- No teleporting or stuttering
- Consistent movement speed

### Test 4: Performance & Stability
**Acceptance Criteria:**
- [ ] No memory leaks (long-running test)
- [ ] Consistent FPS (check Godot's FPS counter)
- [ ] No gradual slowdown over time

**How to Test:**
1. Run scene for 60+ seconds
2. Monitor FPS in Godot (View → Show FPS)
3. Check memory usage doesn't continuously grow

**Expected Results:**
- Stable FPS (60 or your target)
- Memory usage plateaus quickly
- No performance degradation

---

## 🎯 Acceptance Criteria Summary

### Phase 8.1: Mutable Variable Tracking ✅
- [x] Implemented in runtime (VarInfo struct with mutability flag)
- [x] Enforced at assignment time
- [x] Tests pass for immutable violation detection
- [x] Tests pass for mutable variable updates

### Phase 8.2: Persistent Script State ✅
- [x] Environment stored in RustyScriptNode
- [x] Same env reused across _process calls
- [x] Global variables persist between frames
- [x] Tests demonstrate counter persistence

### Phase 8.3: Control Flow Implementation ✅
- [x] If/else statements work correctly
- [x] While loops work correctly
- [x] Nested control flow works
- [x] 26 runtime tests pass including complex control flow

### Phase 8.4: Bounce Test
- [ ] **MANUAL**: Node bounces between boundaries
- [ ] **MANUAL**: Direction variable persists
- [ ] **MANUAL**: Movement is smooth
- [ ] **MANUAL**: No console errors
- [ ] **MANUAL**: Performance is acceptable

---

## 🧪 Extended Test Scenarios

### Scenario 1: Variable Boundaries
Modify `bounce_test.rscr` to test edge cases:
```rust
let boundary: f32 = 50.0;  // Smaller boundary
```
Expected: Faster bouncing, same behavior

### Scenario 2: Different Speed
```rust
let speed: f32 = 300.0;  // Faster movement
```
Expected: Faster movement, same bouncing logic

### Scenario 3: Multiple Nodes
Create 2+ RustyScriptNode instances with bounce_test.rscr
Expected: Each node has independent state (separate environments)

---

## 🐛 Troubleshooting

### Issue: Node doesn't move
**Possible Causes:**
- Script path not set correctly
- Script failed to compile (check console for errors)
- Node not visible (add child Sprite2D/ColorRect)

**Solution:**
1. Check console for "Successfully loaded RustyScript" message
2. Check console for "Bounce test started" message
3. Verify script_path property in Inspector

### Issue: Node moves but doesn't bounce
**Possible Causes:**
- Boundaries not triggering correctly
- Direction variable not persisting

**Solution:**
1. Add print statements to debug direction changes
2. Check that if conditions are evaluating correctly
3. Verify mutable state is working (dir should change values)

### Issue: Node bounces erratically
**Possible Causes:**
- Multiple boundary checks triggering in same frame
- Position overshooting boundaries

**Solution:**
- Current logic should handle this correctly (uses separate if statements)
- If issue persists, may need else if logic

### Issue: Performance degradation
**Possible Causes:**
- Memory leak in runtime
- Property getter/setter overhead

**Solution:**
1. Profile with Godot's profiler
2. Check memory usage over time
3. Report findings for optimization

---

## 📝 Test Results Template

```
Date: YYYY-MM-DD
Tester: [Your Name]
Godot Version: [e.g., 4.5]
Test Duration: [e.g., 5 minutes]

Results:
[ ] Test 1: Mutable State Persistence - PASS/FAIL
[ ] Test 2: Control Flow - PASS/FAIL
[ ] Test 3: Self Property Modification - PASS/FAIL
[ ] Test 4: Performance & Stability - PASS/FAIL

Notes:
[Any observations, issues, or additional findings]

Overall Result: ✅ PASS / ⚠️ PARTIAL / ❌ FAIL
```

---

## 🎓 Learning Objectives

By completing this test, you should understand:
1. How mutable variables persist in RustyScript across frames
2. How control flow (if statements) works in game loops
3. How self property modification integrates with Godot
4. Performance characteristics of RustyScript execution

---

## 🚀 Next Steps After Testing

Once all tests pass:
1. Update `docs/copilot-checklist.md` with results
2. Document any limitations or edge cases found
3. Note performance characteristics
4. Consider any improvements for future versions
5. Proceed to Phase 9 (Polish & Documentation)
