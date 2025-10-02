# v0.0.2 Phase Tracking - Action Items from Validation

**Created:** October 2, 2025  
**Purpose:** Track specific action items identified in Phase 1 validation for future phases  
**Source:** `docs/VALIDATION_REPORT.md`

---

## Phase 2: Core Community Documentation (Days 3-5)

### CONTRIBUTING.md Content Requirements

From validation report, include:

- [ ] **Prerequisites section** - Link to README.md (don't duplicate)
- [ ] **Development environment setup** - IDE recommendations, extensions
- [ ] **Running examples** - `cargo run --example <name>`
- [ ] **Code formatting** - `cargo fmt` requirement
- [ ] **Linting** - `cargo clippy` requirement  
- [ ] **Testing requirements** - All tests must pass before PR

**Time Estimate:** 4-5 hours

---

## Phase 3: User Support Documentation (Days 6-8)

### FAQ.md Content (from Validation Report)

**Installation & Setup:**
- [ ] Q: Minimum requirements? → A: Rust 1.70+, Godot 4.2+, Git (link to README)
- [ ] Q: Build time? → A: 3-5 minutes first build (validated)
- [ ] Q: Compile errors? → A: Link to TROUBLESHOOTING.md
- [ ] Q: Need Godot to build? → A: No, only for testing integration

**Using FerrisScript:**
- [ ] Q: Create first `.ferris` file? → A: Link to Language Overview in README
- [ ] Q: Use without Godot? → A: Not yet, v0.2.0 planned for standalone
- [ ] Q: Difference from GDScript? → A: Compiled to native code, better performance

**Godot Integration:**
- [ ] Q: Load in Godot? → A: Link to README "Using in Godot" (4 steps)
- [ ] Q: Godot doesn't recognize? → A: Check .gdextension file, link to TROUBLESHOOTING.md
- [ ] Q: Mix with GDScript? → A: Yes, side-by-side

**Performance:**
- [ ] Q: Faster than GDScript? → A: Early benchmarks 2-5x, not optimized yet
- [ ] Q: Optimization plans? → A: Link to v0.1.0-ROADMAP.md

**Time Estimate:** 3-4 hours

---

### TROUBLESHOOTING.md Content (from Validation Report)

**Platform-Specific Prerequisites:**

Windows:
- [ ] Issue: MSVC linker not found
- [ ] Solution: Install Visual Studio 2019+ with C++ tools
- [ ] Reference: Validation identified this gap

Linux:
- [ ] Issue: Missing libclang
- [ ] Solution: `sudo apt install libclang-dev build-essential`
- [ ] Reference: Validation identified this gap

macOS:
- [ ] Issue: "xcrun: error: invalid active developer path"
- [ ] Solution: `xcode-select --install`
- [ ] Reference: Validation identified this gap

**Common Build Errors:**

- [ ] Error: "rustc version mismatch"
  - Cause: Rust too old
  - Solution: `rustup update`

- [ ] Error: "failed to resolve: use of undeclared crate"
  - Cause: Missing dependencies
  - Solution: `cargo clean && cargo build`

- [ ] Error: Build freezes/too slow
  - Cause: Low memory or disk space
  - Solution: Close apps, need ~2GB free disk space

**Godot Integration Issues (DEFERRED):**

⚠️ **Note:** These require manual Godot testing or future automation (see `docs/FUTURE_AUTOMATION.md`)

- [ ] Godot doesn't recognize .gdextension file
  - Verify location: `res://addons/ferrisscript/ferrisscript.gdextension`
  - Check syntax matches README template
  - Restart Godot editor

- [ ] "Entry point not found" error
  - Rebuild for correct platform: `cargo build --target <platform>`

**Time Estimate:** 4-5 hours

---

### README.md Updates (from Validation Report)

**Optional Improvements:**

- [ ] Add build time estimate: "(first build takes 3-5 minutes)" after `cargo build --workspace`
- [ ] Add disk space requirement: "Approximately 2GB disk space" in Prerequisites
- [ ] Add "What success looks like" note after test command

**Priority:** Low (nice-to-have)  
**Time Estimate:** 30 minutes

---

## Phase 4: Tooling & Automation (Days 9-10)

### SECURITY.md Content

- [ ] Supported versions table (currently v0.0.1)
- [ ] Vulnerability reporting process (email, private disclosure)
- [ ] Expected response time (suggest: 48 hours)
- [ ] Disclosure policy (after fix released)

**Time Estimate:** 1 hour

---

## Phase 5: Review & Merge (Day 10-11)

### Cross-Reference Validation

From validation report, verify:

- [ ] Installation only in README.md (no duplication in FAQ/TROUBLESHOOTING)
- [ ] Contributing only in CONTRIBUTING.md (README links to it)
- [ ] Godot quick start in README (TROUBLESHOOTING has errors only)

**Reference:** `docs/SINGLE_SOURCE_OF_TRUTH.md`

**Time Estimate:** 1 hour

---

## Future Work (v0.0.3+)

### Godot Automation (from FUTURE_AUTOMATION.md)

- [ ] Research Godot 4.x headless mode
- [ ] Create automated Godot test script
- [ ] Add GitHub Actions workflow for Godot testing
- [ ] Remove "deferred" status from Godot validations

**Target:** v0.0.3 or v0.1.0  
**Effort:** 2-3 days

---

## PR Workflow Best Practices

### Merge Strategy

**Question from User:** Should I use merge or squash merge? Delete branch on close?

**Recommendation:**

1. **For feature branches (like `feature/docs-validation`):**
   - ✅ **Squash and Merge** (recommended)
   - **Why:** Keeps main branch history clean, one commit per feature
   - **Result:** Single commit like "docs: Phase 1 validation - fix installation and create duplication matrix"

2. **For hotfix branches (critical bugs):**
   - ✅ **Merge commit** (alternative)
   - **Why:** Preserves exact commit history for audit trail
   - **Use case:** Production bugs, security fixes

3. **Branch deletion:**
   - ✅ **Always delete after merge**
   - ✅ **Enable "Automatically delete head branches" in GitHub settings**
   - **Why:** Prevents branch clutter, merged work is in main

**GitHub Settings to Enable:**

```
Repository Settings → General → Pull Requests:
☑ Automatically delete head branches
☑ Allow squash merging (default)
☐ Allow merge commits (optional, for hotfixes)
☐ Allow rebase merging (not recommended for this project)
```

### Workflow Summary

```bash
# 1. Create feature branch
git checkout -b feature/docs-contributing

# 2. Make changes, commit
git add .
git commit -m "docs: create CONTRIBUTING.md with GitHub best practices"

# 3. Push and create PR
git push -u origin feature/docs-contributing
# Create PR on GitHub

# 4. After approval, squash and merge via GitHub UI
# ✅ Branch automatically deleted

# 5. Update local main
git checkout main
git pull origin main

# 6. Start next feature
git checkout -b feature/docs-conduct
```

---

## Summary of Tracking

### Immediate (v0.0.2):
- ✅ Phase 1 complete (validation, duplication matrix)
- ⏳ Phase 2-6 (follow workflow document)

### Deferred (v0.0.3+):
- Godot CLI automation
- Code coverage reporting
- Additional platform testing

### Tracked In:
- `docs/v0.0.2-DOCUMENTATION-WORKFLOW.md` - Main workflow
- `docs/VALIDATION_REPORT.md` - Detailed findings
- `docs/SINGLE_SOURCE_OF_TRUTH.md` - Duplication prevention
- `docs/FUTURE_AUTOMATION.md` - Automation opportunities
- `docs/PHASE_TRACKING.md` - This file

---

**End of Phase Tracking Document**
