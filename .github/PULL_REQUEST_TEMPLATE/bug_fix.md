## 🐛 Bug Fix

<!-- 
⚠️ IMPORTANT: This PR should target the 'develop' branch, not 'main'.
Bug fixes go through develop for integration testing before reaching main.
-->

<!-- Provide a clear and concise description of the bug you're fixing -->

## Related Issues

<!-- Link to the issue this PR fixes -->

- Fixes #

## Bug Description

**What was the bug?**
<!-- Describe the incorrect behavior -->

**What was the root cause?**
<!-- Explain what was causing the issue -->

**How did you fix it?**
<!-- Describe your solution -->

## Changes Made

<!-- List the specific changes you made -->

-
-
-

## Testing

<!-- Describe how you verified the fix -->

- [ ] All existing tests pass (`cargo test`)
- [ ] Added regression test to prevent this bug from reoccurring
- [ ] Manual testing completed

**Test Coverage:**
<!-- Show the test that proves this bug is fixed -->

```rust
#[test]
fn test_bug_fix_for_issue_X() {
    // Your test here
}
```

**Manual Testing:**

```bash
# Commands you ran to verify the fix
cargo test
cargo run examples/reproducer.ferris
```

## Before/After

**Before (buggy behavior):**

```
<!-- Paste error message or incorrect output -->
```

**After (fixed behavior):**

```
<!-- Paste correct output -->
```

## Affected Areas

<!-- What parts of the codebase are affected? -->

- [ ] Compiler
- [ ] Runtime
- [ ] Godot Bindings
- [ ] Examples
- [ ] Documentation

## Breaking Changes

- [ ] This fix introduces breaking changes (rare for bug fixes)

**If yes, explain why breaking changes were necessary:**
<!-- Describe the breaking changes and provide migration guidance -->

## Code Quality

- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated (if needed)
- [ ] CHANGELOG.md updated (add to "Fixed" section)
- [ ] Added comments explaining the fix for future maintainers

## Regression Risk

<!-- Assess the risk that this fix might break something else -->

**Risk Level:** <!-- Low / Medium / High -->

**Areas that might be affected:**
<!-- List any code that depends on the changed behavior -->

-
-

## Checklist

- [ ] I have read the [CONTRIBUTING.md](../../CONTRIBUTING.md) guide
- [ ] All tests pass locally
- [ ] I've added a test to prevent regression
- [ ] The bug is completely fixed (not just a workaround)
- [ ] My commits follow [Conventional Commits](https://www.conventionalcommits.org/) (e.g., `fix: resolve null pointer in parser`)

## Notes to Reviewers

<!-- Any specific concerns or areas you'd like reviewers to focus on? -->

---

Thank you for fixing bugs in FerrisScript! 🦀🔧
