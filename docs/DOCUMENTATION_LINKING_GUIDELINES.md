# Documentation Linking Guidelines

**Purpose**: Ensure general-purpose documentation remains evergreen and doesn't link to version-specific archived content.

---

## 🎯 Core Principle

**General documentation should only link to other general documentation.**

Version-specific documentation can link anywhere, but general docs should remain version-agnostic to stay relevant across all versions.

---

## 📋 Linking Rules

### ✅ Appropriate Links

**From General Docs → General Docs**:

- `DEVELOPMENT.md` → `ARCHITECTURE.md` ✅
- `CONTRIBUTING.md` → `DOCUMENTATION_ORGANIZATION.md` ✅
- `FAQ.md` → `TROUBLESHOOTING.md` ✅

**From Version Docs → General Docs**:

- `docs/v0.0.2/v0.0.2-CHECKLIST.md` → `../CONTRIBUTING.md` ✅

**From Version Docs → Version Docs**:

- `docs/v0.0.2/v0.0.2-CHECKLIST.md` → `LEARNINGS.md` ✅

**Meta-Documentation Exceptions**:

- `DOCUMENTATION_INVENTORY.md` can reference version-specific docs (it's cataloging them) ✅
- `VERSION_PLANNING.md` can reference version-specific docs with clear "archived/historical" labels ✅

### ❌ Inappropriate Links

**From General Docs → Version Docs**:

- `DEVELOPMENT.md` → `v0.0.2/TEST_COVERAGE_ANALYSIS.md` ❌
- `CONTRIBUTING.md` → `v0.0.2/PHASE_TRACKING.md` ❌
- `README.md` → `v0.0.2/BENCHMARK_BASELINE.md` ❌

**Why?**: These links become stale and misleading once the version is complete.

---

## 🔧 How to Fix Inappropriate Links

### Strategy 1: Generalize the Content

If general docs need to reference version-specific information, extract the evergreen content:

**Before** (DEVELOPMENT.md):

```markdown
- Baseline coverage: See [v0.0.2/TEST_COVERAGE_ANALYSIS.md](v0.0.2/TEST_COVERAGE_ANALYSIS.md)
```

**After** (DEVELOPMENT.md):

```markdown
- **Test Count**: 116+ tests (and growing)
- **Coverage**: Actively tracked via cargo-llvm-cov locally and cargo-tarpaulin in CI
- **Historical Baseline**: See version-specific archives for baseline snapshots
```

### Strategy 2: Remove the Link

If the content isn't needed in general docs, simply remove it:

**Before** (CONTRIBUTING.md):

```markdown
3. Follow the structure outlined in [Phase Tracking](docs/v0.0.2/PHASE_TRACKING.md)
```

**After** (CONTRIBUTING.md):

```markdown
3. Check [DOCUMENTATION_ORGANIZATION.md](docs/DOCUMENTATION_ORGANIZATION.md) for where new docs should live
```

### Strategy 3: Mark as Historical (for planning docs only)

Only for `VERSION_PLANNING.md` and similar meta-docs:

**Before**:

```markdown
📋 **Full Checklist**: See [v0.0.2/v0.0.2-CHECKLIST.md](v0.0.2/v0.0.2-CHECKLIST.md)
```

**After**:

```markdown
📋 **Historical Checklist**: [v0.0.2/v0.0.2-CHECKLIST.md](v0.0.2/v0.0.2-CHECKLIST.md) *(archived for reference)*
```

---

## 📊 Documentation Categories

### General Documentation (docs/ root)

**Characteristics**:

- Applies to all versions
- Continuously updated
- No version numbers in content
- Evergreen reference material

**Examples**:

- `ARCHITECTURE.md` - System architecture
- `DEVELOPMENT.md` - Developer guide
- `CONTRIBUTING.md` - Contribution guide
- `FAQ.md` - Frequently asked questions
- `TROUBLESHOOTING.md` - Platform troubleshooting

**Linking Policy**: Only link to other general docs or external resources

### Version-Specific Documentation (docs/vX.Y.Z/)

**Characteristics**:

- Tied to specific version
- Point-in-time snapshot
- Contains version numbers
- Archived after release

**Examples**:

- `docs/v0.0.2/v0.0.2-CHECKLIST.md` - Release checklist
- `docs/v0.0.2/BENCHMARK_BASELINE.md` - Performance baseline
- `docs/v0.0.2/LEARNINGS.md` - Development learnings

**Linking Policy**: Can link to general docs or other version docs

### Meta-Documentation (special case)

**Characteristics**:

- Documents the documentation itself
- Catalogs or inventories all docs
- Plans future documentation

**Examples**:

- `DOCUMENTATION_INVENTORY.md` - Catalog of all docs
- `VERSION_PLANNING.md` - Version strategy
- `DOCUMENTATION_ORGANIZATION.md` - Organization principles

**Linking Policy**: Can reference version-specific docs with clear labeling (e.g., "archived", "historical")

---

## ✅ Validation Checklist

Before merging documentation changes, verify:

- [ ] General docs don't link to version-specific docs (except meta-docs with clear labels)
- [ ] Version-specific content is extracted to general docs if needed long-term
- [ ] Links from version docs to general docs use relative paths that won't break
- [ ] Meta-documentation clearly labels version-specific links as "archived" or "historical"

---

## 🔍 How to Audit Links

Use grep to find potentially problematic links:

```bash
# Find links to version-specific docs from root
grep -r "docs/v[0-9]" *.md

# Find links to version-specific docs from docs/ root
cd docs/
grep -r "v[0-9]\.[0-9]\.[0-9]/" *.md | grep -v "v0\.[0-9]\.[0-9]/"
```

Review each match and verify it follows the guidelines above.

---

## 📝 Examples of Good Documentation Practices

### Example 1: Coverage Information

**❌ Bad** (general doc linking to version-specific):

```markdown
See [v0.0.2/TEST_COVERAGE_ANALYSIS.md](v0.0.2/TEST_COVERAGE_ANALYSIS.md) for baseline coverage.
```

**✅ Good** (general doc with extracted evergreen info):

```markdown
**Current Coverage**: 116+ tests, actively tracked via cargo-llvm-cov and cargo-tarpaulin.
For historical baselines, see version-specific documentation archives.
```

### Example 2: Planning References

**❌ Bad** (general doc treating archived content as current):

```markdown
Follow the checklist in [v0.0.2-CHECKLIST.md](v0.0.2/v0.0.2-CHECKLIST.md)
```

**✅ Good** (meta-doc with clear archived label):

```markdown
**Historical Checklist**: [v0.0.2/v0.0.2-CHECKLIST.md](v0.0.2/v0.0.2-CHECKLIST.md) *(archived for reference)*
```

### Example 3: Contribution Guidelines

**❌ Bad** (general doc referencing version-specific workflow):

```markdown
Follow the structure in [Phase Tracking](docs/v0.0.2/PHASE_TRACKING.md)
```

**✅ Good** (general doc referencing general organizational doc):

```markdown
Check [DOCUMENTATION_ORGANIZATION.md](docs/DOCUMENTATION_ORGANIZATION.md) for documentation structure.
```

---

## 🎯 Summary

**Golden Rule**: General documentation should be timeless. If it references version-specific content, either:

1. Extract the evergreen information into the general doc
2. Remove the reference if it's not needed
3. (Meta-docs only) Label clearly as "archived" or "historical"

This keeps general documentation relevant and useful across all versions of the project.
