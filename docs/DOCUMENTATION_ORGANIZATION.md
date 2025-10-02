# Documentation Organization Guide 📚

**Last Updated**: October 2, 2025  
**Purpose**: Prevent `/docs` clutter by separating user-facing docs from development artifacts

---

## 🚨 Problem: Documentation Clutter (October 2025)

As of October 2, 2025, `/docs` contains 18+ files mixing permanent docs with development artifacts, making it hard to find user-facing documentation.

**Solution:** Implement `/docs/meta` directory structure (see below).

---

## 📁 Root-Level Documentation

These files should **always** be in the project root:

### Core Project Files
| File | Purpose | Update Frequency |
|------|---------|------------------|
| `README.md` | Project overview, quick start | Per major release |
| `LICENSE` | Legal license (MIT) | Rarely |
| `CHANGELOG.md` | Complete version history | Every release |
| `CONTRIBUTING.md` | ✅ How to contribute (v0.0.2) | As needed |
| `CODE_OF_CONDUCT.md` | ✅ Community guidelines (v0.0.2) | Rarely |

**Why root?** These are the first files developers look for. GitHub displays them prominently.

---

## 📂 docs/ Root (User-Facing Permanent Documentation)

**New Strategy:** Only 5-6 permanent, user-facing docs in `/docs` root.

### Permanent User-Facing Documentation
| File | Purpose | Audience |
|------|---------|----------|
| `FAQ.md` | ✅ 31 Q&As about installation, usage, Godot | Users |
| `TROUBLESHOOTING.md` | ✅ Platform-specific error resolution | Users |
| `DEVELOPMENT.md` | Developer setup and workflow | Contributors |
| `v0.1.0-ROADMAP.md` | Public roadmap and features | Users & Contributors |
| `GITHUB_BADGES_GUIDE.md` | ✅ Badge setup instructions | Maintainers |
| `ARCHITECTURE.md` | 🔜 Phase 4: Technical design | Contributors |

---

## 📂 docs/meta/ (Development Artifacts - NEW)

**Purpose:** Internal process docs, phase reports, and planning artifacts.

### Directory Structure

```
docs/
├── meta/
│   ├── phase-reports/              # Phase completion reports
│   │   ├── PHASE_2_COMPLETION_REPORT.md
│   │   ├── PHASE_3_COMPLETION_REPORT.md
│   │   └── PHASE_4_COMPLETION_REPORT.md (future)
│   ├── v0.0.2/                     # v0.0.2-specific artifacts
│   │   ├── v0.0.2-CHECKLIST.md
│   │   ├── v0.0.2-DOCUMENTATION-WORKFLOW.md
│   │   ├── v0.0.2-QUICK-START.md
│   │   ├── PHASE_TRACKING.md
│   │   ├── SINGLE_SOURCE_OF_TRUTH.md
│   │   └── VALIDATION_REPORT.md
│   ├── planning/                   # Cross-version planning
│   │   ├── DOCUMENTATION_INVENTORY.md
│   │   ├── DOCUMENTATION_ORGANIZATION.md (this file)
│   │   ├── FUTURE_AUTOMATION.md
│   │   ├── GITHUB_PROJECT_MANAGEMENT.md
│   │   ├── GITHUB_INSIGHTS_DESCRIPTION.md
│   │   ├── LOGO_SETUP.md
│   │   └── VERSION_PLANNING.md
│   └── README.md                   # Explains meta/ purpose
├── archive/                        # Deprecated docs
├── FAQ.md                          # User-facing
├── TROUBLESHOOTING.md              # User-facing
├── DEVELOPMENT.md                  # Contributor-facing
├── v0.1.0-ROADMAP.md               # Public roadmap
└── GITHUB_BADGES_GUIDE.md          # Setup instructions
```

### When to Add to meta/

**Add to meta/ if:**
- ✅ Internal process documentation (phase tracking, completion reports)
- ✅ Version-specific planning artifacts (checklists, workflows)
- ✅ Development decision records (why we chose X over Y)
- ✅ Historical reference (validation reports, inventories)

**Keep in /docs root if:**
- ✅ User-facing documentation (FAQ, Troubleshooting)
- ✅ Contributor-facing documentation (DEVELOPMENT.md)
- ✅ Setup instructions (GITHUB_BADGES_GUIDE.md)
- ✅ Public roadmaps (v0.1.0-ROADMAP.md)
- `DEBUGGING.md` - Debug techniques
- `PERFORMANCE.md` - Optimization guide
- `API_REFERENCE.md` - Generated API docs

**Why docs/?** Detailed information that contributors need but not first-time visitors.

---

## 🗄️ docs/archive/ Directory

For historical, version-specific documentation:

### Structure
```
docs/archive/
├── v0.0.1/
│   ├── RELEASE_NOTES_v0.0.1.md  ← Release-specific notes
│   ├── v0.0.1-checklist.md       ← Phase checklist
│   ├── PHASE6_SUMMARY.md         ← Development phases
│   ├── PHASE6_TESTING.md
│   ├── PHASE7_TESTING.md
│   └── PHASE8_TESTING.md
├── v0.0.2/                       ← Created after v0.0.2 release
└── v0.1.0/                       ← Created after v0.1.0 release
```

### What Goes in Archive
- ✅ Version-specific release notes
- ✅ Development phase documentation
- ✅ Testing reports for that version
- ✅ One-time checklists that are completed
- ❌ NOT planning docs for future versions (those stay in docs/)

**Why archive?** Keeps history accessible without cluttering active workspace.

---

## 🎨 assets/ Directory

For branding and media files:

```
assets/
├── ferrisscript-logo.png       ← Primary logo
├── README.md                   ← Usage guidelines
└── (future: icons, banners, etc.)
```

**Why separate?** Non-text files, used across multiple contexts (README, website, etc.)

---

## 📝 Documentation Workflow

### When Creating a New Release

1. **Update CHANGELOG.md**
   ```markdown
   ## [0.0.2] - 2025-11-XX
   ### Added
   - Feature 1
   ### Fixed
   - Bug 1
   ```

2. **Update RELEASE_NOTES.md**
   - Replace v0.0.1 content with v0.0.2
   - Or keep both if you want multi-version in one file

3. **Archive old release notes**
   ```bash
   cp RELEASE_NOTES.md docs/archive/v0.0.1/RELEASE_NOTES_v0.0.1.md
   ```

4. **Move completed checklists to archive**
   ```bash
   git mv docs/v0.0.2-CHECKLIST.md docs/archive/v0.0.2/
   ```

5. **Create next version planning docs**
   - `docs/v0.0.3-CHECKLIST.md` (if doing another patch)
   - Or start working on `docs/v0.1.0-ROADMAP.md`

---

## 🔄 Documentation Lifecycle

### Active Documents (Root)
**Stay in root, update frequently:**
- README.md
- CHANGELOG.md
- RELEASE_NOTES.md (latest version)
- ARCHITECTURE.md
- CONTRIBUTING.md

### Planning Documents (docs/)
**Active until feature/version complete:**
- VERSION_PLANNING.md (permanent)
- v0.X.Y-CHECKLIST.md (until that version releases)
- v0.X.0-ROADMAP.md (until that version releases)

**Lifecycle:**
```
Create → Work on → Complete → Archive → Create next
```

### Archived Documents (docs/archive/)
**Historical record, rarely updated:**
- Version-specific release notes
- Completed checklists
- Phase summaries
- Test reports

---

## ✅ Current State (Post-Cleanup)

### Root Structure ✅
```
/
├── README.md              ✅ Main docs
├── LICENSE                ✅ MIT license
├── CHANGELOG.md           ✅ New! All version history
├── ARCHITECTURE.md        ✅ Technical design
├── RELEASE_NOTES.md       ✅ Updated (v0.0.1 complete)
├── RELEASING.md           ✅ Release process
└── [other code files]
```

### docs/ Structure ✅
```
docs/
├── VERSION_PLANNING.md           ✅ Strategy overview
├── v0.0.2-CHECKLIST.md          ✅ Next patch plan
├── v0.1.0-ROADMAP.md            ✅ Feature roadmap
├── DOCUMENTATION_INVENTORY.md   ✅ Doc audit
├── LOGO_SETUP.md                ✅ Moved from root
├── DEVELOPMENT.md               ⚠️ Might be duplicate?
└── archive/
    └── v0.0.1/
        ├── RELEASE_NOTES_v0.0.1.md  ✅ Archived
        ├── v0.0.1-checklist.md      ✅ Original checklist
        ├── PHASE6_SUMMARY.md        ✅ Phase docs
        ├── PHASE6_TESTING.md
        ├── PHASE7_TESTING.md
        └── PHASE8_TESTING.md
```

---

## 🎯 Benefits of This Organization

1. **Clear Separation**
   - Root = Important, frequently accessed
   - docs/ = Detailed, for deep dives
   - archive/ = Historical record

2. **Easy to Find**
   - GitHub users find README, CONTRIBUTING in root
   - Contributors find planning docs in docs/
   - Historians find old versions in archive/

3. **Scales Well**
   - Add new versions to archive/
   - Planning docs don't clutter root
   - Easy to see what's active vs historical

4. **Standard Practice**
   - Follows common open source patterns
   - Similar to React, Rust, Vue, etc.
   - Contributors know where to look

---

## 🚦 Next Steps for v0.0.2

### Documentation to Create
1. **CONTRIBUTING.md** (root) - High priority!
2. **CODE_OF_CONDUCT.md** (root)
3. **FAQ.md** (root or docs/)
4. **TROUBLESHOOTING.md** (root or docs/)

### Documentation to Review
1. Check for duplicate DEVELOPMENT.md (root vs docs/)
2. Update VERSION_PLANNING.md as decisions are made
3. Keep CHANGELOG.md updated with each PR

### Documentation to Archive (After v0.0.2)
1. Move `docs/v0.0.2-CHECKLIST.md` → `docs/archive/v0.0.2/`
2. Copy RELEASE_NOTES.md → `docs/archive/v0.0.2/RELEASE_NOTES_v0.0.2.md`
3. Create `docs/v0.0.3-CHECKLIST.md` or start v0.1.0 work

---

## 📚 Documentation Standards

### File Naming
- **Root docs**: UPPERCASE (README.md, CONTRIBUTING.md)
- **Subdocs**: lowercase with hyphens (api-reference.md)
- **Archived**: Include version in name (RELEASE_NOTES_v0.0.1.md)

### Content Standards
- Start with purpose statement
- Include "Last Updated" date on living docs
- Use emojis for visual scanning
- Link between related documents
- Keep line length ≤100 characters

### Maintenance
- Review docs quarterly for accuracy
- Archive old version docs when releasing new versions
- Update CHANGELOG.md with every release
- Sync RELEASE_NOTES.md with GitHub releases

---

**Questions or suggestions?** Open a discussion on GitHub!
