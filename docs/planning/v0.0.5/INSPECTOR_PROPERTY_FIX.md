# 🐛 Inspector Property Refresh Fix

**Status**: 📋 Ready for Implementation  
**Priority**: 🟢 Quick Win (1-2 hours)  
**Phase**: 0.1.5 (can run in parallel with Phase 0.1)  
**Delegation**: ✅ Background Agent Task  

---

## 📝 Problem Statement

### User-Facing Issue

When a `.ferris` script fails to compile due to type mismatches, the Godot Inspector continues to show stale exported properties from the previous successful compilation. This creates confusion because:

1. **User sees outdated UI**: Inspector shows properties that no longer exist in the broken script
2. **User assumes properties are valid**: Tries to modify properties that won't be used
3. **User must manually reload scene**: Forces full scene reload to clear stale state
4. **Poor developer experience**: Feels like a bug, not expected behavior

### Current Behavior

```
1. Script compiles successfully → Inspector shows 3 properties
2. User edits script → introduces type error
3. Script fails to compile → Inspector STILL shows 3 properties
4. User confused: "Why are these properties still here?"
5. User forced to reload scene to clear Inspector
```

### Expected Behavior

```
1. Script compiles successfully → Inspector shows 3 properties
2. User edits script → introduces type error
3. Script fails to compile → Inspector CLEARS properties automatically
4. User sees empty Inspector → understands script is broken
5. User fixes error → properties reappear automatically
```

### Root Cause

**Location**: `crates/godot_bind/src/lib.rs` (FerrisScriptRunner)  
**Issue**: Compilation error path doesn't notify Godot Inspector to refresh

```rust
// Current implementation (simplified)
impl IScriptExtension for FerrisScriptRunner {
    fn reload(&mut self, keep_state: bool) -> bool {
        match self.compile_and_update() {
            Ok(_) => {
                // ✅ Success path notifies Inspector
                self.base_mut().notify_property_list_changed();
                true
            }
            Err(e) => {
                // ❌ Error path does NOT notify Inspector
                godot_error!("Compilation failed: {}", e);
                // Stale properties remain in Inspector!
                false
            }
        }
    }
}
```

**Why this happens**:

- `reload()` returns `false` on error, but doesn't clear state
- Godot continues to use `cached_property_list` from previous successful compilation
- Inspector UI never receives signal to refresh

---

## 🎯 Acceptance Criteria

### Must Have

- [ ] **AC-1**: Compilation error clears exported properties list
- [ ] **AC-2**: Inspector UI updates immediately when compilation fails
- [ ] **AC-3**: Inspector UI repopulates when compilation succeeds again
- [ ] **AC-4**: Fix works with Godot hot-reload (no full scene reload required)
- [ ] **AC-5**: Fix doesn't break existing successful compilation behavior

### Should Have

- [ ] **AC-6**: Documented in code comments why `notify_property_list_changed()` is called on error
- [ ] **AC-7**: TROUBLESHOOTING.md updated to mark issue as "Fixed in v0.0.5"

### Won't Have (Out of Scope)

- ❌ Partial property updates (e.g., only hide broken properties)
- ❌ Error tooltips in Inspector (requires more invasive Godot changes)
- ❌ Property diffing (tracking which properties changed)

---

## 🔧 Implementation Tasks

### Task 1: Add `clear_on_error()` Method

**File**: `crates/godot_bind/src/lib.rs`  
**Estimated Time**: 15 minutes

**Implementation**:

```rust
impl FerrisScriptRunner {
    /// Clears all script state and notifies Godot Inspector to refresh.
    /// Called when compilation fails to prevent stale properties from displaying.
    fn clear_on_error(&mut self) {
        // Clear internal state
        self.compiled_ast = None;
        self.script_source = None;
        self.exported_properties.clear();
        
        // Notify Godot Inspector to refresh UI
        // This ensures stale properties don't linger in the Inspector
        self.base_mut().notify_property_list_changed();
        
        godot_print!("Cleared script state due to compilation error");
    }
}
```

**Why this works**:

- `notify_property_list_changed()` triggers Godot to call `get_property_list()` again
- Empty `exported_properties` vec results in empty Inspector
- Godot UI updates immediately without manual scene reload

---

### Task 2: Call `clear_on_error()` in Error Path

**File**: `crates/godot_bind/src/lib.rs`  
**Estimated Time**: 10 minutes

**Current Code** (approximate location):

```rust
fn reload(&mut self, keep_state: bool) -> bool {
    match self.compile_and_update() {
        Ok(_) => {
            self.base_mut().notify_property_list_changed();
            true
        }
        Err(e) => {
            godot_error!("Compilation failed: {}", e);
            false  // ❌ Doesn't clear state
        }
    }
}
```

**Updated Code**:

```rust
fn reload(&mut self, keep_state: bool) -> bool {
    match self.compile_and_update() {
        Ok(_) => {
            self.base_mut().notify_property_list_changed();
            true
        }
        Err(e) => {
            godot_error!("Compilation failed: {}", e);
            self.clear_on_error();  // ✅ Clear stale state
            false
        }
    }
}
```

**Edge Cases to Handle**:

- First compilation failure (no previous state to clear) ✅ Works
- Rapid edits with alternating success/failure ✅ Works (each reload clears or populates)
- Multiple scripts in same scene ✅ Works (per-instance state)

---

### Task 3: Test Manually in Godot

**Estimated Time**: 20 minutes

**Test Procedure**:

1. **Setup**:
   - Open Godot project with `.ferris` script attached to node
   - Verify script compiles successfully
   - Verify Inspector shows exported properties

2. **Introduce Error**:
   - Edit `.ferris` script to add type mismatch (e.g., `let x: int = "string"`)
   - Save file
   - **Expected**: Inspector clears properties immediately

3. **Fix Error**:
   - Restore correct type (e.g., `let x: int = 42`)
   - Save file
   - **Expected**: Inspector repopulates with properties

4. **Rapid Edits**:
   - Toggle between valid and invalid script states quickly
   - **Expected**: Inspector always reflects current compilation state

5. **Multiple Scripts**:
   - Add second `.ferris` script to different node
   - Break one script, keep other valid
   - **Expected**: Only broken script clears its Inspector properties

**Success Criteria**:

- No stale properties visible in Inspector after compilation error
- No full scene reload required to see property changes
- No errors in Godot console during test

---

### Task 4: Add Unit Test (Optional)

**File**: `crates/godot_bind/src/lib.rs` (in-module test)  
**Estimated Time**: 15 minutes

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_on_error_empties_properties() {
        // This is a pseudo-test since we can't easily mock Godot environment
        // Real testing happens manually in Godot
        
        // What we'd verify if we had mocking:
        // 1. clear_on_error() empties exported_properties vec
        // 2. clear_on_error() calls notify_property_list_changed()
        // 3. reload() calls clear_on_error() on Err path
    }
}
```

**Note**: Full test coverage requires Godot runtime (integration test). This is acceptable for a 2-hour fix.

---

### Task 5: Update Documentation

**File**: `docs/TROUBLESHOOTING.md`  
**Estimated Time**: 10 minutes

**Current Entry**:

```markdown
### Inspector properties not updating after script changes

**Status**: Known issue in v0.0.4  
**Workaround**: Reload scene (Scene → Reload Saved Scene)  
**Root Cause**: Godot caches property list, not refreshed on compilation error
```

**Updated Entry**:

```markdown
### Inspector properties not updating after script changes

**Status**: ✅ Fixed in v0.0.5  
**Solution**: Compilation errors now automatically clear Inspector properties  
**Previous Workaround** (v0.0.4 only): Reload scene (Scene → Reload Saved Scene)  
**Implementation**: See [INSPECTOR_PROPERTY_FIX.md](planning/v0.0.5/INSPECTOR_PROPERTY_FIX.md)
```

---

## 📊 Testing Strategy

### Manual Testing Checklist

- [ ] **Test 1**: Properties clear on first compilation error
- [ ] **Test 2**: Properties repopulate on successful recompilation
- [ ] **Test 3**: Rapid edits don't cause Inspector flickering
- [ ] **Test 4**: Multiple scripts in same scene behave independently
- [ ] **Test 5**: No Godot console errors during property refresh
- [ ] **Test 6**: Hot-reload works (no full editor restart required)

### Regression Testing

**Verify existing behavior still works**:

- [ ] Successful compilation still shows properties
- [ ] Property values persist across successful reloads
- [ ] `@export` syntax still parses correctly
- [ ] Property types (int, string, bool) still display correctly

### Integration Testing (Future)

**Out of scope for this fix, but noted for Phase 4**:

- Automated Godot headless test with property inspection
- Scripted error injection and Inspector state verification

---

## 🚀 Deployment Plan

### PR Structure

**Branch**: `fix/inspector-property-refresh`  
**Target**: `main`  
**Estimated Review Time**: 30 minutes

**PR Checklist**:

- [ ] Code changes in `lib.rs`
- [ ] TROUBLESHOOTING.md updated
- [ ] Manual test results in PR description
- [ ] Screenshot/GIF of Inspector behavior (before/after)
- [ ] No new compiler warnings

### Merge Criteria

1. ✅ All acceptance criteria met
2. ✅ Manual test checklist complete
3. ✅ No regressions in existing property behavior
4. ✅ Code review approved
5. ✅ CI passes (if applicable)

---

## 🔗 Related Documents

- **Planning**: [v0.0.5 README.md](README.md) (this fix is Phase 0.1.5)
- **Issue**: [TROUBLESHOOTING.md](../../TROUBLESHOOTING.md#inspector-properties-not-updating)
- **Implementation**: `crates/godot_bind/src/lib.rs`

---

## 📝 Notes for Background Agent

### Context for Agent

This is a **quick win** that can run in parallel with Phase 0.1 (Source Spans). It's a self-contained bug fix with:

- Clear root cause
- Simple 2-line fix
- Well-defined test procedure
- No dependencies on other v0.0.5 work

### Agent Execution Steps

1. Read `crates/godot_bind/src/lib.rs` to understand current `reload()` implementation
2. Add `clear_on_error()` method as specified in Task 1
3. Update `reload()` error path as specified in Task 2
4. Update `docs/TROUBLESHOOTING.md` as specified in Task 5
5. Create PR with manual test checklist (no automated test needed)
6. Notify user for manual Godot testing

### Success Signal

PR ready for user review with:

- Code changes complete
- Documentation updated
- Manual test checklist provided for user validation

---

**Estimated Total Time**: 1-2 hours (70 minutes implementation + 30 minutes review)  
**Complexity**: 🟢 Low (simple state clearing + Godot API call)  
**Impact**: 🟢 High (improves developer experience significantly)
