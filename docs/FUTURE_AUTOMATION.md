# Future Automation Ideas - FerrisScript

**Created:** October 2, 2025  
**Status:** Tracking for future versions  
**Purpose:** Document automation opportunities discovered during v0.0.2 work

---

## Godot CLI Automation (v0.0.3+)

### Problem Statement

**Current State (v0.0.2):**
- Godot integration testing is **deferred** (manual validation required)
- Cannot programmatically validate "Using in Godot" steps
- CI/CD cannot test Godot integration automatically
- Documentation validation requires manual Godot installation and testing

**Impact:**
- Phase 1 validation had to defer Godot steps
- Cannot guarantee Godot instructions work in CI
- Contributors must manually test Godot integration

### Proposed Solution: Godot Headless/CLI Testing

**Godot 4.x supports headless mode** which could enable automated testing:

```bash
# Godot 4.x headless mode
godot --headless --path /path/to/project --script test_script.gd
```

**Potential Approach:**

1. **Download Godot in CI:**
   ```yaml
   # .github/workflows/godot-integration.yml
   - name: Download Godot 4.2
     run: |
       wget https://github.com/godotengine/godot/releases/download/4.2-stable/Godot_v4.2-stable_linux.x86_64.zip
       unzip Godot_v4.2-stable_linux.x86_64.zip
       chmod +x Godot_v4.2-stable_linux.x86_64
   ```

2. **Automate project import:**
   ```bash
   ./Godot_v4.2-stable_linux.x86_64 --headless --path godot_test --import
   ```

3. **Run automated tests:**
   ```bash
   # Create test script that validates FerrisScript integration
   ./Godot_v4.2-stable_linux.x86_64 --headless --path godot_test --script test_ferrisscript.gd
   ```

4. **Validate GDExtension loading:**
   - Test that `FerrisScriptNode` is available
   - Test that `.ferris` scripts execute
   - Test that `_ready()` and `_process()` callbacks work
   - Capture output and assert expected behavior

### Benefits

✅ **Removes "deferred" state** - All validations can be completed in Phase 1  
✅ **CI/CD coverage** - Every PR tests Godot integration  
✅ **Documentation accuracy** - Automated validation ensures docs stay current  
✅ **Faster development** - Contributors can validate without manual Godot testing  
✅ **Regression prevention** - Breaking changes to Godot integration caught immediately

### Implementation Effort

**Estimated Complexity:** Medium (2-3 days)

**Tasks:**
1. Research Godot 4.x headless mode capabilities
2. Create automated test script (`godot_test/test_ferrisscript.gd`)
3. Add GitHub Actions workflow (`.github/workflows/godot-integration.yml`)
4. Download/cache Godot binary in CI
5. Validate all "Using in Godot" steps programmatically
6. Update documentation validation process

**Dependencies:**
- Godot 4.2+ headless binary (Linux/Windows/macOS)
- Test framework for GDExtension validation
- CI/CD runner with sufficient resources (~4GB RAM)

### Target Version

**Recommendation:** v0.0.3 or v0.1.0

**Rationale:**
- v0.0.2 is documentation-focused, this is tooling/CI work
- Not blocking for current release
- Would benefit from CONTRIBUTING.md being in place first
- Fits with v0.0.3 quality improvements or v0.1.0 tooling

### Related Issues to Track

When implemented, this will resolve:
- Deferred Godot validations from Phase 1
- Manual testing burden for contributors
- CI/CD gap in Godot integration coverage

### References

- [Godot 4.x Command Line Tutorial](https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html)
- [Godot Headless Mode](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html)
- [GDExtension Testing](https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/index.html)

---

## Other Automation Opportunities

### 1. Markdown Link Checker (v0.0.2 - Phase 4)

**Status:** Planned for Phase 4  
**Tool:** `markdown-link-check`  
**Benefit:** Catch broken links in documentation automatically

### 2. Spell Checker (v0.0.2 - Phase 4, Optional)

**Status:** Optional for Phase 4  
**Tool:** `cspell`  
**Benefit:** Catch typos in documentation

### 3. API Documentation Generation (v0.1.0+)

**Status:** Future  
**Tool:** `cargo doc` or `rustdoc`  
**Benefit:** Auto-generated API reference from Rust docs

### 4. Benchmark Automation (v0.1.0+)

**Status:** Future  
**Tool:** `criterion` or custom benchmarks  
**Benefit:** Track performance regressions automatically

### 5. Code Coverage Reporting (v0.0.3+)

**Status:** Future (mentioned in v0.0.2 checklist)  
**Tool:** `tarpaulin` or `codecov`  
**Benefit:** Visualize test coverage, ensure quality

---

## Tracking

**Document Location:** `docs/FUTURE_AUTOMATION.md`  
**Review Frequency:** Each minor version (v0.X.0)  
**Owner:** Maintainers

When planning v0.0.3 or v0.1.0, review this document for automation opportunities.

---

**End of Future Automation Ideas**
