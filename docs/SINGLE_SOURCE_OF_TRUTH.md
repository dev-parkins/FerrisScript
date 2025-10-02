# Single Source of Truth Matrix

**Created:** October 2, 2025  
**Purpose:** Prevent documentation duplication by defining primary locations for each topic  
**Status:** v0.0.2 Planning

---

## Overview

This matrix defines the **single source of truth** for each documentation topic in FerrisScript. When adding or updating documentation, always check this matrix first to ensure you're editing the correct file and not creating duplicate content.

**Golden Rule:** High-traffic documentation should **never** be duplicated. Link to it, don't copy it.

---

## Primary Documentation Matrix

| Topic | Primary Location | Can Reference From | Never Duplicate In | Notes |
|-------|-----------------|-------------------|-------------------|-------|
| **Installation** | `README.md` | FAQ.md, CONTRIBUTING.md (link only) | DEVELOPMENT.md, TROUBLESHOOTING.md | High-traffic, must stay in README |
| **Quick Start** | `README.md` | CONTRIBUTING.md (link only) | Any other file | First thing users see |
| **Prerequisites** | `README.md` | CONTRIBUTING.md (link only), FAQ.md (link only) | TROUBLESHOOTING.md | Keep in one place |
| **Language Overview** | `README.md` | CONTRIBUTING.md (link only) | DEVELOPMENT.md | Basic syntax examples |
| **Contributing Process** | `CONTRIBUTING.md` (Phase 2) | README.md (link only), PR template | Issue templates | Detailed workflow |
| **Code of Conduct** | `CODE_OF_CONDUCT.md` (Phase 2) | README.md (link), CONTRIBUTING.md (link) | Never duplicate | Standard location |
| **Troubleshooting** | `TROUBLESHOOTING.md` (Phase 3) | FAQ.md (link to specific sections) | README.md, CONTRIBUTING.md | Detailed error solutions |
| **FAQ** | `FAQ.md` (Phase 3) | README.md (link only) | Never duplicate | Quick Q&A with links |
| **Security Policy** | `SECURITY.md` (Phase 4) | CONTRIBUTING.md (link only) | Never duplicate | Standard location |
| **Architecture** | `ARCHITECTURE.md` | CONTRIBUTING.md (link only) | DEVELOPMENT.md | Technical deep dive |
| **Development Setup** | `CONTRIBUTING.md` (Phase 2) | DEVELOPMENT.md (can expand) | README.md | Contributor-specific |
| **Release Process** | `RELEASING.md` | Never reference externally | DEVELOPMENT.md | Maintainer-only |
| **License** | `LICENSE` | README.md (link only) | Never duplicate | Legal requirement |
| **Godot Integration** | `README.md` (Quick Start) | TROUBLESHOOTING.md (errors only) | CONTRIBUTING.md | Keep basics in README |
| **Examples** | `examples/*.ferris` | README.md (link), tutorials | Never inline in docs | Link to examples/ |
| **API Reference** | (Future: generate from code) | README.md (link when exists) | Never manually document | Auto-generated only |

---

## High-Traffic Documents (Never Duplicate)

These documents are frequently accessed and must remain authoritative. **Always link to them, never copy content from them.**

1. **README.md**
   - Installation steps
   - Prerequisites
   - Quick start (4-step Godot guide)
   - Basic language overview
   - License badge

2. **CONTRIBUTING.md** (Phase 2)
   - How to contribute
   - Development workflow
   - Code style guide
   - PR process

3. **CODE_OF_CONDUCT.md** (Phase 2)
   - Community guidelines
   - Enforcement process

4. **LICENSE**
   - MIT License text

---

## Cross-Reference Rules

### ✅ DO: Link to Primary Source

**Good Example (FAQ.md):**

```markdown
**Q: How do I install FerrisScript?**
A: See the [Installation section in README.md](../README.md#installation) for complete instructions.
```

**Good Example (CONTRIBUTING.md):**

```markdown
## Getting Started

Before contributing, ensure you have FerrisScript installed. Follow the 
[installation instructions in README.md](../README.md#installation).
```

### ❌ DON'T: Duplicate Content

**Bad Example (FAQ.md):**

```markdown
**Q: How do I install FerrisScript?**
A: Run these commands:
```bash
git clone https://github.com/dev-parkins/FerrisScript.git
cd FerrisScript
cargo build --workspace
```

```

**Why it's bad:** If installation steps change, this becomes outdated and misleading.

---

## Topic Ownership

### README.md Owns:
- Installation (3 commands)
- Prerequisites (Rust 1.70+, Godot 4.2+, Git)
- Quick Start (Godot 4-step guide)
- Basic syntax examples
- Feature list
- Project structure

### CONTRIBUTING.md Owns (Phase 2):
- Fork and clone workflow
- Branch naming conventions
- Commit message format
- PR submission process
- Code review expectations
- Development environment setup
- Running tests
- Code style guidelines

### FAQ.md Owns (Phase 3):
- Common questions with **links** to detailed docs
- Quick troubleshooting hints with **links** to TROUBLESHOOTING.md
- "Where do I find X?" answers with **links**

### TROUBLESHOOTING.md Owns (Phase 3):
- Detailed error messages and solutions
- Platform-specific issues (Windows/Linux/macOS)
- Build errors
- Godot integration errors
- Runtime errors

### SECURITY.md Owns (Phase 4):
- Supported versions
- Vulnerability reporting process
- Disclosure policy

---

## Duplication Detection Checklist

Before writing documentation, ask:

1. **Does this content already exist?**
   - Search existing docs for the topic
   - Check this matrix for ownership

2. **Is this a high-traffic topic?**
   - Installation → README.md only
   - Contributing → CONTRIBUTING.md only
   - Code of Conduct → CODE_OF_CONDUCT.md only

3. **Should I link instead of duplicate?**
   - If content is >3 lines, probably yes
   - If it's a process/command, definitely yes

4. **Will this get out of sync?**
   - If commands might change, link to primary source
   - If it's dynamic (versions, steps), don't duplicate

5. **Is there a better place for this?**
   - Installation details → README.md
   - Error solutions → TROUBLESHOOTING.md
   - Process workflows → CONTRIBUTING.md

---

## Current State (v0.0.1)

### ✅ No Duplication Found

Based on validation, FerrisScript v0.0.1 has **no documentation duplication**:
- Installation only in README.md
- Godot integration only in README.md
- Architecture details only in ARCHITECTURE.md
- Development only in DEVELOPMENT.md (root)

### ⚠️ Potential Risk: DEVELOPMENT.md

**Issue:** Both `DEVELOPMENT.md` (root) and `docs/DEVELOPMENT.md` exist.

**Investigation:**
- Root `DEVELOPMENT.md`: Exists, used by contributors
- `docs/DEVELOPMENT.md`: Does NOT exist (false positive in inventory)

**Resolution:** ✅ No action needed. Only one DEVELOPMENT.md exists.

---

## v0.0.2 Prevention Strategy

### When Creating New Docs (Phase 2-4):

1. **CONTRIBUTING.md**
   - ✅ DO: Link to README.md for installation
   - ✅ DO: Add development-specific setup (IDE, extensions)
   - ❌ DON'T: Duplicate installation commands
   - ❌ DON'T: Duplicate language overview

2. **FAQ.md**
   - ✅ DO: Provide quick answers with links
   - ✅ DO: Link to TROUBLESHOOTING.md for details
   - ❌ DON'T: Duplicate installation instructions
   - ❌ DON'T: Duplicate error solutions from TROUBLESHOOTING.md

3. **TROUBLESHOOTING.md**
   - ✅ DO: Provide detailed error solutions
   - ✅ DO: Reference README.md for correct commands
   - ❌ DON'T: Repeat basic installation steps
   - ❌ DON'T: Duplicate FAQ answers

4. **Issue/PR Templates**
   - ✅ DO: Link to CONTRIBUTING.md
   - ✅ DO: Enforce process with checklists
   - ❌ DON'T: Duplicate contribution guidelines

---

## Maintenance Strategy

### Monthly Review
- Check for new duplicates (search for repeated commands)
- Validate all cross-reference links
- Update this matrix if ownership changes

### Version Release
- Archive version-specific docs
- Update primary docs for new version
- Ensure links still point to correct sections

### Documentation PRs
- Reviewer must check this matrix
- Verify no duplication introduced
- Confirm links used instead of copying

---

## Validation Commands

### Check for Duplicate Installation Commands
```powershell
# Search for "git clone" in all markdown files
Get-ChildItem -Path . -Recurse -Include "*.md" | Select-String "git clone" | Select-Object Path, LineNumber
```

### Check for Duplicate Prerequisites

```powershell
# Search for "Rust 1.70" in all markdown files
Get-ChildItem -Path . -Recurse -Include "*.md" | Select-String "Rust 1.70" | Select-Object Path, LineNumber
```

### Check for Duplicate Godot Steps

```powershell
# Search for "_ready()" function in docs (should only be in examples)
Get-ChildItem -Path . -Recurse -Include "*.md" | Select-String "_ready\(\)" | Select-Object Path, LineNumber
```

---

## Future Considerations

### Auto-Generated Documentation

- API reference (from Rust docs)
- Test coverage reports
- Benchmark results

**Rule:** Never manually duplicate auto-generated content. Always link to generated docs.

### Localization/Translations

If FerrisScript adds translated docs:

- Translate entire files, not snippets
- Maintain same structure as English docs
- Update all translations when primary source changes

---

## Sign-Off

**Created By:** GitHub Copilot  
**Date:** October 2, 2025  
**Status:** ✅ Active - Use this matrix for all v0.0.2 documentation work  
**Next Review:** After Phase 5 (before v0.0.2 release)

---

End of Single Source of Truth Matrix
