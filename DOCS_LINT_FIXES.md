# Documentation Linting Fixes - Summary

## How to Apply to PR #3

The fixes have been made and are available in this branch. To apply them to PR #3 (`feature/docs-phase4`), run:

```bash
# Checkout the PR branch
git checkout feature/docs-phase4

# Cherry-pick the fix commits
git cherry-pick 5f094f9 ee151d5

# Push to update PR #3
git push origin feature/docs-phase4
```

Alternatively, you can merge this branch into PR #3's branch.

---

## What Was Fixed (Automatically)

Successfully reduced markdown linting errors from **~1000** to **44** by auto-fixing simple formatting issues:

### Fixed Error Types

- **MD031**: Fenced code blocks now surrounded by blank lines
- **MD032**: Lists now surrounded by blank lines  
- **MD022**: Headings now surrounded by blank lines
- **MD012**: Removed multiple consecutive blank lines

### Files Modified: 41 files

All changes are non-breaking formatting improvements that add blank lines for better readability and standards compliance.

## What Remains (Requires User Decision)

### 1. MD029 - Ordered List Prefix (7 instances)

**File:** `CONTRIBUTING.md` (lines 137, 143, 149, 185, 194, 201, 207)

These are intentionally styled numbered lists. Review to decide if you want:

- Option A: Keep current style (2, 3, 4 format)
- Option B: Change to sequential (1, 2, 3 format)

### 2. MD036 - Emphasis Used as Heading (37 instances)

**Common examples:**

- "Made with 🦀 and ❤️ for the Godot community!" (footer in multiple files)
- "Decision: Tree-walking" (docs/ARCHITECTURE.md)
- "Test 1: Branch Logic" (docs/archive/v0.0.1/PHASE6_TESTING.md)
- Error messages in quotes (docs/archive/v0.0.1/PHASE6_TESTING.md)

**Decision needed:** These are stylistic choices. Options:

- Option A: Convert to proper headings (## or ###)
- Option B: Disable MD036 rule in `.markdownlint.json`
- Option C: Leave as-is (they provide visual emphasis)

### 3. MD056 - Table Column Count (3 instances)

**File:** `docs/VERSION_PLANNING.md` (lines 176, 188, 197)

Malformed table rows. **Requires manual review** - automated fix could corrupt data.

### 4. Link Check Failures (CANNOT AUTO-FIX)

#### Broken Internal Links (Missing Files)

- `docs/LANGUAGE_REFERENCE.md` (referenced in 3 example READMEs)
- `docs/ROADMAP.md` (referenced in 2 example READMEs)
- `CONTRIBUTING.md`, `LICENSE`, etc. in archived release notes

**Action Required:**

- Create missing files OR
- Update references to point to existing files OR
- Add to link-check ignore patterns

#### GitHub Settings URLs (404)

- `https://github.com/dev-parkins/FerrisScript/settings/branches`
- `https://github.com/dev-parkins/FerrisScript/settings`

**Reason:** These are private URLs only accessible to repo owners.

**Action Required:**

- Add these patterns to `.markdown-link-check.json` ignore list OR
- Replace with public URLs OR
- Document as "admin-only" links

#### External URL Failures

- `https://status.shields.io/` (Status: 0 - likely timeout/network issue)

**Action Required:**

- Verify URL is correct
- May need to increase timeout in `.markdown-link-check.json`
- Or add to ignore patterns if it's a temporary issue

## Recommendations

1. **Commit this fix** ✅ (Already done - simple formatting improvements)

2. **Address MD036 violations** (Optional):

   ```json
   // Add to .markdownlint.json if you want to keep emphasis style:
   {
     "MD036": false
   }
   ```

3. **Fix broken links** (High Priority):
   - Create `docs/LANGUAGE_REFERENCE.md` or update references
   - Create `docs/ROADMAP.md` or point to `docs/v0.1.0-ROADMAP.md`

4. **Update link-check config** (Medium Priority):

   ```json
   // Add to .markdown-link-check.json:
   {
     "ignorePatterns": [
       {
         "pattern": "^https://github.com/.*/settings"
       },
       {
         "pattern": "^https://status.shields.io"
       }
     ]
   }
   ```

5. **Fix table formatting** (Low Priority):
   - Manually review `docs/VERSION_PLANNING.md` tables at lines 176, 188, 197

## Testing

To verify fixes locally:

```bash
# Install tools
npm install -g markdownlint-cli

# Run markdown linter
markdownlint --config .markdownlint.json .

# Run link checker
npx markdown-link-check --config .markdown-link-check.json README.md
```

## Summary

✅ **Simple formatting fixes applied** - 96% error reduction
⚠️ **Remaining issues require user decisions** - semantic/content changes
📋 **Link check failures need content updates** - missing files or broken URLs
