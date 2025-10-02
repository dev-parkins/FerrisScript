# Phase 4 Implementation Plan: Advanced Topics & Tooling

**Created:** October 2, 2025  
**Phase:** 4 of 6 (v0.0.2 Documentation Workflow)  
**Target:** Days 9-10 of workflow  
**Branch Strategy:** Feature branches → Pull Requests → Main

---

## Executive Summary

Phase 4 shifts from user-facing documentation (Phase 2-3) to developer-facing advanced topics and tooling automation. This phase adds technical depth for contributors and automates quality checks.

**Key Objectives:**
1. Add architecture and design decision documentation
2. Expand example projects with detailed explanations
3. Implement documentation quality automation (linting, link checking)
4. Create SECURITY.md for vulnerability reporting
5. Optionally add AUTHORS/CONTRIBUTORS recognition

---

## Phase 4 Deliverables

### Primary Deliverables (Must-Have)

#### 1. SECURITY.md
- **Purpose:** Vulnerability reporting policy
- **Location:** Root directory (`/SECURITY.md`)
- **Time Estimate:** 1 hour
- **Priority:** HIGH (GitHub community standard)

**Content Requirements:**
- Supported versions table (currently v0.0.1)
- How to report vulnerabilities (email: dev-parkins@users.noreply.github.com or private Security Advisory)
- Expected response time (48 hours)
- Disclosure policy (after fix released, coordinated)
- Public disclosure timeline

**Template:**
```markdown
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.0.1   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. Please do NOT open a public issue.

**Preferred Method:** GitHub Security Advisory
1. Go to https://github.com/dev-parkins/FerrisScript/security/advisories
2. Click "New draft security advisory"
3. Provide details of the vulnerability

**Alternative:** Email dev-parkins@users.noreply.github.com with:
- Description of vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

**Response Time:**
- Initial response: Within 48 hours
- Fix timeline: Depends on severity (Critical: 1-3 days, High: 1 week, Medium: 2 weeks)
- Public disclosure: After fix released and users notified

## Disclosure Policy

We follow coordinated disclosure:
1. You report the vulnerability privately
2. We confirm and investigate
3. We develop and test a fix
4. We release a patched version
5. We publicly disclose the issue (crediting you, if desired)

Thank you for keeping FerrisScript secure! 🔒
```

---

#### 2. Documentation Linting CI
- **Purpose:** Automated quality checks for documentation
- **Location:** `.github/workflows/docs-lint.yml`
- **Time Estimate:** 1-1.5 hours
- **Priority:** HIGH (prevents broken links, enforces consistency)

**Workflow Content:**
```yaml
name: Documentation Linting

on:
  pull_request:
    paths:
      - '**.md'
      - 'docs/**'
  push:
    branches: [main]
    paths:
      - '**.md'
      - 'docs/**'

jobs:
  markdown-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run markdownlint
        uses: nosborn/github-action-markdown-cli@v3.3.0
        with:
          files: .
          config_file: .markdownlint.json
          
  link-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Check links
        uses: gaurav-nelson/github-action-markdown-link-check@v1
        with:
          use-quiet-mode: 'yes'
          config-file: '.markdown-link-check.json'
```

**Configuration Files:**

`.markdownlint.json`:
```json
{
  "default": true,
  "MD013": false,
  "MD033": false,
  "MD041": false
}
```

`.markdown-link-check.json`:
```json
{
  "ignorePatterns": [
    {
      "pattern": "^http://localhost"
    }
  ],
  "timeout": "20s",
  "retryOn429": true,
  "retryCount": 3
}
```

---

#### 3. ARCHITECTURE.md (Technical Design Documentation)
- **Purpose:** Explain system design for contributors
- **Location:** `/docs/ARCHITECTURE.md`
- **Time Estimate:** 2-3 hours
- **Priority:** MEDIUM-HIGH (critical for contributors)

**Content Outline:**

1. **System Overview**
   - High-level architecture diagram (text-based or mermaid)
   - Compiler pipeline: Lexer → Parser → Type Checker → Codegen
   - Runtime architecture
   - GDExtension bridge

2. **Crate Structure**
   - `rustyscript_compiler`: Lexer, Parser, AST, Type Checker
   - `rustyscript_runtime`: Variable storage, function execution
   - `rustyscript_godot_bind`: GDExtension FFI layer

3. **Compiler Pipeline Detail**
   - Lexer: Tokenization strategy (regex-based)
   - Parser: Recursive descent, error recovery
   - Type Checker: Type inference rules, coercion
   - (Future) Codegen: Bytecode vs native

4. **Runtime Execution**
   - Variable scoping
   - Function calls
   - Godot interop (calling Godot API from FerrisScript)

5. **Extension Points**
   - Adding new types
   - Adding new operators
   - Adding new Godot bindings

6. **Design Decisions**
   - Why tree-walk interpreter (not bytecode yet)
   - Why GDExtension (not custom GDScript VM integration)
   - Why Rust (safety, performance, Godot 4 compatibility)

---

#### 4. Expand examples/ with Detailed READMEs
- **Purpose:** Teach language features through practical examples
- **Location:** `examples/` directory
- **Time Estimate:** 1.5-2 hours
- **Priority:** MEDIUM (helps new users and contributors)

**Current Examples:**
- `hello.ferris` - Basic print statement
- `move.ferris` - Simple node movement
- `bounce.ferris` - Bounce physics

**Enhancements:**

Each example gets a companion README explaining:
- What the example demonstrates
- Line-by-line explanation
- Common gotchas
- How to run in Godot
- Variations to try

**Example: `examples/move/README.md`**
```markdown
# move.ferris - Node Movement Example

## What This Demonstrates

- Variable declaration (`let mut velocity: f32`)
- Godot node property access (`self.position.x`)
- Arithmetic operations
- `_process(delta)` callback

## Code Explanation

```ferris
fn _ready() {
    // Runs once when node enters scene
    print("Movement script ready!");
}

fn _process(delta: f32) {
    // Runs every frame
    // delta = time since last frame (typically 0.016 for 60 FPS)
    
    self.position.x += 50.0 * delta;
    // Moves node 50 pixels/second to the right
    // Without delta, speed would vary based on framerate!
}
```

## Running This Example

### Option 1: Cargo (No Godot)
```bash
cargo run --example move
# Prints compilation output and runtime trace
```

### Option 2: In Godot
1. Create a Node2D in Godot scene
2. Attach FerrisScript custom script
3. Set script content to `move.ferris`
4. Run scene - node moves right continuously

## Common Gotchas

- **Forgot `* delta`:** Node moves at framerate speed (fast on powerful PCs, slow on weak ones)
- **Forgot `mut`:** Compile error if trying to modify velocity later
- **Wrong type for `delta`:** Must be `f32` (Godot's float type)

## Variations to Try

- Move on the Y axis: `self.position.y += 50.0 * delta;`
- Move diagonally: Modify both x and y
- Add velocity variable: Make speed configurable
- Add boundary checking: Stop at screen edge
```

---

### Secondary Deliverables (Optional)

#### 5. DESIGN_DECISIONS.md
- **Purpose:** Document "why" decisions for contributors
- **Location:** `/docs/DESIGN_DECISIONS.md`
- **Time Estimate:** 2 hours
- **Priority:** LOW (nice-to-have, can defer to v0.1.0)

**Content:**
- Why tree-walk interpreter (simplicity, v0.1.0 bytecode plan)
- Why `.ferris` extension (not `.rs` or `.fscr`)
- Why no borrow checker (game scripting simplicity)
- Why mutable-by-default (GDScript familiarity)
- Why GDExtension (Godot 4 native, performance)

---

#### 6. AUTHORS.md / CONTRIBUTORS.md
- **Purpose:** Credit contributors
- **Location:** `/AUTHORS.md` (root)
- **Time Estimate:** 30 minutes
- **Priority:** LOW (can defer until more contributors)

**Content:**
- Project maintainer(s)
- Core contributors (from `git log`)
- Documentation contributors
- Link to GitHub contributors page

---

## Phase 4 Workflow

### Task 4.1: Create SECURITY.md
**Branch:** `feature/docs-security`

1. Create `/SECURITY.md` using template above
2. Adjust email/contact info
3. Test GitHub Security Advisory link
4. Commit: `docs: add security policy for vulnerability reporting`
5. Push and create PR
6. Merge after review

---

### Task 4.2: Add Documentation Linting CI
**Branch:** `feature/ci-docs-lint`

1. Create `.github/workflows/docs-lint.yml`
2. Create `.markdownlint.json` config
3. Create `.markdown-link-check.json` config
4. Test locally (if possible) or push and check CI
5. Fix any existing lint errors in documentation
6. Commit: `ci: add markdown linting and link checking`
7. Push and create PR
8. Verify CI runs on PR
9. Merge after CI passes

---

### Task 4.3: Create ARCHITECTURE.md
**Branch:** `feature/docs-architecture`

1. Draft system overview section
2. Add crate structure explanations
3. Document compiler pipeline (lexer → parser → type checker)
4. Document runtime execution model
5. Add design decisions (why tree-walk, why GDExtension)
6. Add extension points for contributors
7. (Optional) Create mermaid diagrams for architecture
8. Commit: `docs: add ARCHITECTURE.md explaining system design`
9. Push and create PR
10. Request technical review from maintainers
11. Merge after review

---

### Task 4.4: Enhance Examples with READMEs
**Branch:** `feature/docs-examples`

1. Create `examples/hello/` directory, move `hello.ferris`
2. Create `examples/hello/README.md` with explanation
3. Repeat for `move` and `bounce` examples
4. Update root README.md to link to example READMEs
5. Test examples still run: `cargo run --example hello`
6. Commit: `docs: add detailed READMEs for examples/`
7. Push and create PR
8. Merge after review

---

### Task 4.5: Optional - DESIGN_DECISIONS.md
**Branch:** `feature/docs-design-decisions`

(Only if time permits, otherwise defer to v0.1.0)

1. Create `/docs/DESIGN_DECISIONS.md`
2. Document major "why" decisions
3. Commit: `docs: add design decisions document`
4. Push and create PR
5. Merge after review

---

### Task 4.6: Optional - AUTHORS.md
**Branch:** `feature/docs-authors`

(Defer until more contributors, likely v0.1.0)

1. Create `/AUTHORS.md`
2. List maintainers
3. Generate contributor list from `git log`
4. Commit: `docs: add AUTHORS.md crediting contributors`
5. Push and create PR
6. Merge after review

---

## Phase 4 Success Criteria

### Must-Have (Required for Phase 4 Completion)
- [x] `SECURITY.md` created and passes GitHub community standards
- [x] Documentation linting CI workflow implemented and passing
- [x] `ARCHITECTURE.md` created with system design explanation
- [x] All examples have detailed README.md explanations

### Nice-to-Have (Optional, Can Defer)
- [ ] `DESIGN_DECISIONS.md` documents major "why" decisions
- [ ] `AUTHORS.md` credits all contributors
- [ ] Spell-checking CI workflow

### Validation Checks
- [ ] GitHub community standards shows 100% complete
- [ ] CI passes on all PRs (including docs linting)
- [ ] No broken links in any documentation
- [ ] Examples work and READMEs are accurate
- [ ] Technical reviewers approve ARCHITECTURE.md

---

## Time Estimates

| Task | Estimated Time | Priority |
|------|----------------|----------|
| SECURITY.md | 1 hour | HIGH |
| Documentation Linting CI | 1-1.5 hours | HIGH |
| ARCHITECTURE.md | 2-3 hours | MEDIUM-HIGH |
| Example READMEs | 1.5-2 hours | MEDIUM |
| DESIGN_DECISIONS.md | 2 hours | LOW (defer) |
| AUTHORS.md | 30 minutes | LOW (defer) |
| **Total (Must-Have)** | **5.5-7.5 hours** | **~1 workday** |
| **Total (All)** | **8-10 hours** | **~1.5 workdays** |

**Recommended:** Complete must-have tasks (SECURITY, CI, ARCHITECTURE, Examples) in Phase 4. Defer DESIGN_DECISIONS and AUTHORS to v0.1.0 when there are more contributors and clearer patterns.

---

## Integration with Existing Documentation

### Cross-References to Add

**From README.md:**
- Link to ARCHITECTURE.md in "Contributing" section
- Link to enhanced examples/ READMEs

**From CONTRIBUTING.md:**
- Reference ARCHITECTURE.md for technical overview
- Reference example READMEs for code patterns

**From FAQ.md:**
- Link to SECURITY.md for vulnerability questions
- Link to ARCHITECTURE.md for "how does it work" questions

**From ARCHITECTURE.md:**
- Link to CONTRIBUTING.md for how to contribute
- Link to examples/ for practical demonstrations
- Link to DESIGN_DECISIONS.md (if created) for "why" questions

---

## Coordination with GitHub Management (Phase 3 Deliverable)

### Label Creation (from v0.0.2-CHECKLIST.md)

When creating issues/PRs for Phase 4 tasks, use these labels:
- `documentation` (type)
- `P1-High` or `P2-Medium` (priority)
- `good-first-issue` (for example READMEs - easy for newcomers)
- `advanced` (for ARCHITECTURE.md - requires deep knowledge)
- `docs` (component)

### Milestone Assignment

Add all Phase 4 tasks to `v0.0.2` milestone (should be created from Phase 3 GitHub management checklist).

---

## Documentation Organization (From Phase 3 Feedback)

**User Feedback:** "Troubleshooting doc looks decent, while this is in the right place do we possibly need to bucket the phase documents to not sit in the root of /docs to avoid them cluttering?"

**Solution Implemented:** See `docs/DOCUMENTATION_ORGANIZATION.md` (updated)

**Phase 4 File Locations:**

**Permanent Docs (stay in root or /docs):**
- `/SECURITY.md` - Root (GitHub expects here)
- `/docs/ARCHITECTURE.md` - User/contributor-facing
- `/docs/DESIGN_DECISIONS.md` - User/contributor-facing (if created)
- `/AUTHORS.md` - Root (GitHub convention, if created)
- `examples/*/README.md` - Next to examples (logical location)

**Development Artifacts (move to /docs/meta after completion):**
- Phase 4 completion report → `docs/meta/phase-reports/`
- Any planning docs → `docs/meta/planning/`

---

## Next Steps After Phase 4 Completion

1. **Create Phase 4 Completion Report**
   - Document time tracking (actual vs estimated)
   - List all deliverables with metrics
   - Capture learnings and insights
   - Provide recommendations for Phase 5

2. **Update CHANGELOG.md**
   - Add Phase 4 deliverables to [Unreleased] section
   - Follow Keep a Changelog format

3. **Move to Phase 5: Review & Merge**
   - Cross-reference validation
   - GitHub community standards check (should be 100% now)
   - Link checking across all docs
   - Final merge of all feature branches

4. **Reorganize /docs (if not done yet)**
   - Implement `docs/meta/` structure from DOCUMENTATION_ORGANIZATION.md
   - Move phase reports and planning docs to appropriate subdirectories

---

## Questions & Clarifications

### From User Feedback

**Q:** "Do we possibly need to bucket the phase documents to not sit in the root of /docs to avoid them cluttering?"

**A:** ✅ **Addressed** - Created `docs/DOCUMENTATION_ORGANIZATION.md` with `/docs/meta` structure proposal. Will implement after Phase 4 completion.

**Q:** "On the project management doc, I'd opt for option 1 as well for future implementation."

**A:** ✅ **Recorded** - Path-based conditional CI (Option 1) added to `v0.0.2-CHECKLIST.md` under "v0.0.3 (Deferred)" section. Priority: HIGH (95% time savings).

**Q:** "Not sure how to add the github stars badge yet."

**A:** ✅ **Resolved** - Created `docs/GITHUB_BADGES_GUIDE.md` with copy-paste instructions. Badges added to v0.0.2-CHECKLIST.md under "GitHub Project Management" section.

---

## Resources & References

- **Phase Tracking:** `docs/PHASE_TRACKING.md` (Phase 4 requirements)
- **Workflow Doc:** `docs/v0.0.2-DOCUMENTATION-WORKFLOW.md` (Phase 4 section, lines 450-500)
- **GitHub Management:** `docs/GITHUB_PROJECT_MANAGEMENT.md` (CI/CD, labels, milestones)
- **Documentation Organization:** `docs/DOCUMENTATION_ORGANIZATION.md` (file structure strategy)
- **Badge Guide:** `docs/GITHUB_BADGES_GUIDE.md` (shields.io instructions)

---

**Ready to begin Phase 4:** Start with SECURITY.md (highest priority, shortest task) → CI linting (automates quality) → ARCHITECTURE.md (core technical doc) → Example READMEs (nice polish). 🚀
