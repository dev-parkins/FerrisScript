# GitHub Project Management Strategy

**Created:** October 2, 2025  
**Purpose:** Document GitHub features, CI/CD optimization, and project organization decisions  
**Status:** Active planning document

---

## 1. CI/CD Workflow Optimization

### Current State Analysis

**File:** `.github/workflows/ci.yml`

**Current Triggers:**

```yaml
on:
  push:
    branches: [main, develop]
    tags: ['v*']
  pull_request:
    branches: [main]
```

**Current Jobs:**

- `test` - Runs on all pushes/PRs (3 platforms: Ubuntu, Windows, macOS)
  - Cargo test
  - Clippy (continue-on-error)
  - Fmt check (continue-on-error)
- `build` - Runs after test passes (3 platform releases)

### Issue: Documentation PRs Running Full CI

**Problem:** Documentation-only changes trigger:

- 3x platform matrix tests (~5 minutes each = 15 minutes)
- 3x platform release builds (~10 minutes each = 30 minutes)
- **Total: ~45 minutes for a typo fix**

### Solution Options

#### Option 1: Path-Based Conditional Execution (RECOMMENDED)

**Approach:** Skip code-related jobs for docs-only changes

```yaml
name: CI/CD

on:
  push:
    branches: [main, develop]
    tags: ['v*']
  pull_request:
    branches: [main]

jobs:
  # Detect change type
  changes:
    runs-on: ubuntu-latest
    outputs:
      code: ${{ steps.filter.outputs.code }}
      docs: ${{ steps.filter.outputs.docs }}
    steps:
      - uses: actions/checkout@v4
      - uses: dorny/paths-filter@v2
        id: filter
        with:
          filters: |
            code:
              - 'crates/**'
              - 'Cargo.toml'
              - 'Cargo.lock'
            docs:
              - '**.md'
              - 'docs/**'
              - '.github/ISSUE_TEMPLATE/**'
              - '.github/PULL_REQUEST_TEMPLATE.md'

  # Only run tests if code changed
  test:
    needs: changes
    if: needs.changes.outputs.code == 'true'
    # ... existing test job

  # Only run builds if code changed and tests passed
  build:
    needs: [changes, test]
    if: needs.changes.outputs.code == 'true'
    # ... existing build job

  # Quick validation for docs-only PRs
  docs-check:
    needs: changes
    if: needs.changes.outputs.docs == 'true' && needs.changes.outputs.code == 'false'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check Markdown links
        uses: gaurav-nelson/github-action-markdown-link-check@v1
      - name: Check spelling (optional)
        uses: rojopolis/spellcheck-github-actions@v0
        with:
          config_path: .github/spellcheck-config.yml
```

**Pros:**

- ✅ Fast docs PRs (1-2 minutes vs 45 minutes)
- ✅ Still validates markdown links
- ✅ Full CI runs for code changes
- ✅ No separate branch needed
- ✅ Industry standard (used by Rust, TypeScript, React)

**Cons:**

- ⚠️ Requires `dorny/paths-filter` action
- ⚠️ More complex workflow file

**Time Savings:**

- Docs PR: 45 min → 2 min (95% reduction)
- Code PR: No change (45 min)

#### Option 2: Separate Docs Branch with Minimal CI

**Approach:** Use `docs/*` branches with different CI rules

```yaml
on:
  push:
    branches: 
      - main
      - develop
      - 'docs/**'  # Add docs branches
  pull_request:
    branches: [main]

jobs:
  test:
    # Skip on docs branches
    if: "!startsWith(github.ref, 'refs/heads/docs/')"
    # ... existing
```

**Pros:**

- ✅ Simple workflow modification
- ✅ Clear branch naming convention

**Cons:**

- ⚠️ Requires discipline (use `docs/` prefix)
- ⚠️ Doesn't help with docs in feature branches
- ⚠️ Still runs CI if one code file + docs files change

#### Option 3: Manual Workflow Dispatch

**Approach:** Use `workflow_dispatch` for docs

**Pros:**

- ✅ Maximum control

**Cons:**

- ❌ Manual trigger required (slows workflow)
- ❌ Easy to forget

### Recommendation

#### Implement Option 1 (Path-Based Conditional Execution)

**Reasoning:**

1. Industry standard (Rust, Node.js, React all use this)
2. Automatic - no manual intervention
3. Handles mixed PRs (some code + some docs)
4. Provides docs-specific checks (link validation)
5. Fastest for docs-only changes

Implementation Phase: v0.0.3 (after v0.0.2 docs complete)

Estimated Setup Time: 1-2 hours

---

## 2. GitHub Labels Strategy

### Current State

**Labels:** None configured (GitHub defaults only)

### Recommended Label System

#### Priority Labels (for triage)

```
🔴 priority: critical    - Red (#d73a4a)      - Security, data loss, blocking issues
🟠 priority: high        - Orange (#ff9800)   - Major bugs, important features
🟡 priority: medium      - Yellow (#fbca04)   - Standard priority
🟢 priority: low         - Green (#0e8a16)    - Nice-to-have, future enhancements
```

#### Type Labels (what kind of work)

```
🐛 type: bug             - Red (#d73a4a)      - Something isn't working
✨ type: feature         - Purple (#a55eea)   - New feature request
📝 type: documentation   - Blue (#0075ca)     - Documentation improvements
🧪 type: test            - Cyan (#00d4ff)     - Test coverage
♻️ type: refactor        - Gray (#6c757d)     - Code restructuring
⚡ type: performance     - Yellow (#fbca04)   - Speed improvements
🔒 type: security        - Red (#d73a4a)      - Security issue
```

#### Status Labels (workflow state)

```
🚦 status: needs-triage  - Light gray (#ededed) - Needs initial review
👀 status: needs-review  - Yellow (#fbca04)     - Waiting for reviewer
🔄 status: in-progress   - Blue (#0075ca)       - Actively being worked on
⏸️ status: blocked       - Red (#d73a4a)        - Waiting on external factor
✅ status: ready-to-merge - Green (#0e8a16)     - Approved, passing CI
```

#### Difficulty Labels (for contributors)

```
🌱 good first issue      - Green (#7057ff)    - Good for newcomers
🎓 help wanted           - Green (#008672)    - Community help needed
💪 difficulty: easy      - Light green (#c5f015) - 1-2 hours
🏃 difficulty: medium    - Yellow (#fbca04)   - Half day
🏔️ difficulty: hard      - Orange (#ff9800)   - Multiple days
```

#### Component Labels (where the issue is)

```
📦 component: compiler   - Blue (#0075ca)
🏃 component: runtime    - Purple (#a55eea)
🎮 component: godot-bind - Orange (#ff9800)
📚 component: docs       - Light blue (#c7e9f1)
🔧 component: tooling    - Gray (#6c757d)
```

### Label Usage in Templates

**Update `.github/ISSUE_TEMPLATE/bug_report.md`:**

```yaml
labels: ["type: bug", "status: needs-triage"]
```

**Update `.github/ISSUE_TEMPLATE/feature_request.md`:**

```yaml
labels: ["type: feature", "status: needs-triage"]
```

**Update `.github/ISSUE_TEMPLATE/documentation.md`:**

```yaml
labels: ["type: documentation", "status: needs-triage"]
```

### Label Automation (Future - v0.0.3+)

Use GitHub Actions to auto-label:

- PRs that touch docs: `type: documentation`
- PRs with >500 lines: `difficulty: hard`
- PRs from first-time contributors: `good first issue`

**Tool:** [actions/labeler](https://github.com/actions/labeler)

---

## 3. Milestones Strategy

### Purpose

Milestones group related issues/PRs for release planning.

### Recommended Milestones

#### Active Milestones

```
📋 v0.0.2 - Documentation & Polish
  Due: October 15, 2025
  Description: Comprehensive documentation, community standards, bug fixes
  Issues: 
    - Phase 1: Validation ✅
    - Phase 2: Community docs ✅
    - Phase 3: FAQ & Troubleshooting (in progress)
    - Phase 4: Advanced topics
    - Phase 5: Integration examples
    - Phase 6: Final polish

📋 v0.1.0 - Language Features
  Due: December 15, 2025
  Description: Match expressions, enums, structs, improved type system
  Issues: TBD (link to v0.1.0-ROADMAP.md)

📋 v0.2.0 - Tooling & Developer Experience
  Due: March 15, 2026
  Description: LSP, debugger, package manager, better error messages
  Issues: TBD
```

#### Ongoing Milestones

```
🔄 Community
  No due date
  Description: Ongoing community improvements (templates, guides, discussions)

🐛 Bug Triage
  No due date
  Description: Bugs waiting for priority assignment
```

### Milestone Usage

1. **Create milestone** before starting version work
2. **Assign issues/PRs** to milestone as they're created
3. **Track progress** via milestone page (shows %)
4. **Close milestone** when version ships
5. **Create release notes** from milestone issues

---

## 4. GitHub Projects (Beta)

### Current State

**Projects:** None configured

### Recommendation

**Wait until v0.0.3** - Current workflow (branches + PRs + milestones) sufficient for now.

**When to Adopt:**

- More than 3 active contributors
- Managing >20 concurrent issues
- Need Kanban board visualization

### Future Project Setup (v0.0.3+)

**Board Name:** "FerrisScript Development"

**Columns:**

- 📥 Backlog
- 🎯 Planned (this version)
- 🚧 In Progress
- 👀 In Review
- ✅ Done

**Automation:**

- Issues → Backlog
- PRs → In Review
- Merged PRs → Done

---

## 5. GitHub Wiki Strategy

### Current State

**Wiki:** Not enabled

### Recommendation: **Use Wiki Selectively**

### Wiki vs. Docs Folder Decision Matrix

| Content Type | Location | Rationale |
|--------------|----------|-----------|
| **Official docs** | `docs/` | Version controlled, reviewed via PR, part of releases |
| **User guides** | `docs/` | Same as above |
| **API reference** | `docs/` | Generated from code comments |
| **Contributing** | `CONTRIBUTING.md` | Root file, GitHub standard |
| **Code of Conduct** | `CODE_OF_CONDUCT.md` | Root file, GitHub standard |
| **FAQ** | `docs/FAQ.md` | Stable, version-controlled |
| **Troubleshooting** | `docs/TROUBLESHOOTING.md` | Stable, version-controlled |
| | | |
| **Community tutorials** | Wiki | Community-contributed, rapid updates |
| **Third-party integrations** | Wiki | External tools, not core docs |
| **Meeting notes** | Wiki | Internal, not user-facing |
| **Design discussions** | Wiki or Discussions | Ongoing, not finalized |
| **Known issues (dynamic)** | Wiki | Frequently updated |

### Wiki Sections (if enabled in v0.0.3+)

```
📚 Community Tutorials
  - "Building Your First Game with FerrisScript"
  - "Porting from GDScript to FerrisScript"
  - "Performance Optimization Tips"

🔌 Third-Party Tools
  - VS Code extensions (community-built)
  - Build tool integrations
  - Alternative editors

💡 Design Documents
  - Future feature proposals
  - Architecture decisions
  - RFC (Request for Comments) discussions

🗒️ Meeting Notes
  - Developer sync notes
  - Community calls
```

### Why Not Wiki for Core Docs?

**Problems with Wiki for official docs:**

1. ❌ Not version controlled (no PR review)
2. ❌ Not part of releases (can diverge)
3. ❌ Hard to maintain consistency
4. ❌ No automated testing (broken links, typos)
5. ❌ Separate from code contributions

**Exceptions (when Wiki is good):**

1. ✅ Rapidly changing content (known issues)
2. ✅ Community contributions (tutorials)
3. ✅ Internal process docs (meeting notes)
4. ✅ Supplementary content (not critical)

### Decision: **Keep Core Docs in `docs/`, Enable Wiki for Community**

**Phase:** v0.0.3+ (after docs stable)

---

## 6. Other GitHub Features to Consider

### 6.1 GitHub Discussions (ENABLED ✅)

**Status:** User has enabled this

**Recommended Categories:**

```
💬 General - General discussion about FerrisScript
💡 Ideas - Feature suggestions, brainstorming
❓ Q&A - Questions from users (enable "answered" feature)
📢 Announcements - Release notes, blog posts (read-only for most)
🎨 Show and Tell - Projects built with FerrisScript
```

**Usage:**

- Redirect "How do I..." questions from Issues → Discussions Q&A
- Use for feature design discussions before creating issues
- Community showcase

**Update:** Modify `.github/ISSUE_TEMPLATE/config.yml` discussions link to actual URL

### 6.2 GitHub Sponsors (Future)

**When:** After v0.1.0 launch + 100+ stars

**Purpose:** Sustainable funding for development

**Tiers Example:**

- $5/mo - Sponsor badge
- $25/mo - Name in README
- $100/mo - Priority support
- $500/mo - Consulting/custom features

### 6.3 Security Policy (v0.0.3)

**File:** `SECURITY.md`

**Content:**

- Supported versions
- How to report vulnerabilities (GitHub Security Advisories)
- Response timeline
- Disclosure policy

**Triggered by:** GitHub security scanning

### 6.4 Code Scanning (Future)

**When:** v0.0.3+

**Tools:**

- CodeQL (GitHub native, free for public repos)
- Clippy in CI (already have this)
- Dependabot (auto-update dependencies)

**Benefits:**

- Catch security vulnerabilities
- Automated dependency updates
- Code quality insights

### 6.5 Branch Protection Rules (IMMEDIATE)

**Recommendation:** Enable for `main` branch NOW

**Settings → Branches → Add rule for `main`:**

```
✅ Require pull request before merging
  ✅ Require approvals (1)
  ✅ Dismiss stale reviews
✅ Require status checks to pass
  ✅ Require branches to be up to date
  ✅ Status checks: test, docs-check (after CI update)
✅ Require conversation resolution before merging
✅ Include administrators (enforce for everyone)
❌ Allow force pushes (keep disabled)
❌ Allow deletions (keep disabled)
```

**Benefits:**

- Prevents accidental pushes to main
- Ensures CI passes
- Requires code review

---

## 7. Phasing Plan

### Immediate (Phase 2 - Now)

- ✅ Discussions enabled
- ✅ Templates created
- ⏳ **Branch protection** (enable now - 5 minutes)
- ⏳ **Create labels** (30 minutes)
- ⏳ **Create v0.0.2 milestone** (5 minutes)

### v0.0.3 (Next Patch - ~2 weeks)

- 🔄 CI/CD optimization (path-based execution)
- 🔄 Label automation
- 🔄 SECURITY.md
- 🔄 Wiki for community tutorials (optional)

### v0.1.0 (Next Minor - ~2 months)

- 🔄 GitHub Projects (if team grows)
- 🔄 Code scanning (CodeQL)
- 🔄 Dependabot

### Future (v0.2.0+)

- 🔄 GitHub Sponsors
- 🔄 Custom actions for FerrisScript tooling
- 🔄 Automated release notes generation

---

## 8. Immediate Action Items for User

### 1. Enable Branch Protection (5 min)

**Steps:**

1. Go to: https://github.com/dev-parkins/FerrisScript/settings/branches
2. Click "Add rule"
3. Branch name pattern: `main`
4. Check these boxes:
   - ✅ Require a pull request before merging
     - Require approvals: 1 (you can approve your own for now)
   - ✅ Require status checks to pass before merging
     - Search for "test" and check it
   - ✅ Require conversation resolution before merging
5. Save changes

### 2. Create Labels (30 min)

**Fast method using GitHub CLI:**

```bash
# If gh is installed and authenticated
gh label create "priority: critical" --color "d73a4a" --description "Security, data loss, blocking issues"
gh label create "priority: high" --color "ff9800" --description "Major bugs, important features"
# ... (repeat for all labels above)
```

**Or manual:** Settings → Labels → New label (repeat 20 times)

### 3. Create v0.0.2 Milestone (5 min)

**Steps:**

1. Go to: https://github.com/dev-parkins/FerrisScript/milestones
2. Click "New milestone"
3. Title: `v0.0.2 - Documentation & Polish`
4. Due date: October 15, 2025
5. Description: "Comprehensive documentation, community standards, bug fixes"
6. Create milestone
7. Assign Phase 2 PR to this milestone

### 4. Update config.yml with Discussions URL (2 min)

**File:** `.github/ISSUE_TEMPLATE/config.yml`

Replace placeholder URL with actual Discussions URL.

---

## Summary & Recommendations

### Questions Answered

| Question | Answer | Implementation |
|----------|--------|----------------|
| **CI for docs PRs?** | Use path-based conditional (Option 1) | v0.0.3 (1-2 hours) |
| **Labels?** | Yes - 20 labels across 5 categories | Now (30 min) |
| **Milestones?** | Yes - per version + ongoing | Now (5 min) |
| **GitHub Projects?** | Wait until v0.0.3 | v0.0.3+ |
| **Wiki?** | Yes, but only for community tutorials | v0.0.3+ |
| **Other features?** | Branch protection (NOW), Sponsors (later) | See phasing above |

### Priority Actions - COMPLETED ✅

1. ✅ **Branch protection** (5 min) - Documentation created (see `docs/BRANCH_PROTECTION.md`)
2. ✅ **Labels** (30 min) - 20 labels created across 5 categories (see `docs/GITHUB_LABELS.md`)
3. ✅ **v0.0.2 milestone** (5 min) - Milestone #1 created (due Oct 18, 2025)
4. ⏸️ **Update config.yml** (2 min) - Pending (Discussions link)
5. ⏸️ **Insights description** (3 min) - Pending (see next section)

**Completed:** October 5, 2025 (Phase 5A)  
**Documentation:**

- `docs/GITHUB_LABELS.md` - Label system documentation
- `docs/BRANCH_PROTECTION.md` - Branch protection configuration
- `scripts/create-labels.sh` - Label creation automation (Bash)
- `scripts/create-labels.ps1` - Label creation automation (PowerShell)

---

Made with 🦀 and ❤️ for the Godot community
