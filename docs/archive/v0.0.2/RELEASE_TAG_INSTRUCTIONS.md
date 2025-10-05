# v0.0.2 Release Instructions for User

**After PR Merge**: Follow these steps to create the official v0.0.2 release

---

## Prerequisites

✅ PR #20 (feature/v0.0.2-phase6-release-preparation) merged to main  
✅ All CI checks passed on main  
✅ Local repository up-to-date

---

## Step 1: Update Local Repository

```powershell
# Switch to main branch
git checkout main

# Pull latest changes (including merged PR)
git pull origin main

# Verify you're on the correct commit
git log --oneline -5
```

**Expected**: You should see the Phase 6 merge commit at the top

---

## Step 2: Create Annotated Git Tag

```powershell
git tag -a v0.0.2 -m "Release v0.0.2: Community Foundation

Major improvements:
- Community infrastructure (CONTRIBUTING, CODE_OF_CONDUCT, templates)
- Enhanced error messages (38 errors with context and hints)
- Test coverage expansion (116 tests, 70-75% coverage)
- API documentation (395+ lines rustdoc)
- VS Code syntax highlighting extension
- Comprehensive testing guide (655 lines TESTING.md)

See CHANGELOG.md for complete details."
```

**Verification**:
```powershell
# Verify tag created
git tag -l v0.0.2

# View tag details
git show v0.0.2
```

---

## Step 3: Push Tag to Remote

```powershell
# Push the tag to GitHub
git push origin v0.0.2
```

**Result**: Tag will appear in GitHub under "Tags" section and trigger any release workflows

---

## Step 4: Create GitHub Release

### Option A: Using GitHub CLI (Recommended)

```powershell
# Create release from tag
gh release create v0.0.2 `
  --title "v0.0.2: Community Foundation 🦀✨" `
  --notes-file RELEASE_NOTES.md `
  --latest
```

### Option B: Using GitHub Web Interface

1. **Navigate to Releases**:
   - Go to: https://github.com/dev-parkins/FerrisScript/releases/new

2. **Select Tag**:
   - Choose existing tag: `v0.0.2`

3. **Release Title**:
   ```
   v0.0.2: Community Foundation 🦀✨
   ```

4. **Release Description**:
   - Copy the v0.0.2 section from `RELEASE_NOTES.md` (lines 11-138)
   - Paste into description field
   - Ensure formatting renders correctly (preview before publishing)

5. **Release Options**:
   - ✅ Set as latest release
   - ⬜ Pre-release (leave unchecked)
   - ⬜ Create discussion (optional)

6. **Publish**:
   - Click **"Publish release"**

---

## Step 5: Verify Release

### Check Release Page

Visit: https://github.com/dev-parkins/FerrisScript/releases/tag/v0.0.2

**Verify**:
- ✅ Title displays correctly
- ✅ Description formatted properly (headings, code blocks, lists)
- ✅ Tag shows `v0.0.2`
- ✅ Marked as "Latest" release
- ✅ CHANGELOG.md link works
- ✅ All sections visible (Highlights, What's New, Dependencies, Upgrade Guide, Metrics)

### Check Repository

- ✅ GitHub homepage shows "v0.0.2" badge (if badges configured)
- ✅ "Releases" section shows v0.0.2 as latest
- ✅ Tag appears in "Tags" list

---

## Step 6: Post-Release Actions (Optional)

### Update Repository Settings

If not already done:

1. **Branch Protection** (recommended):
   - Protect `main` branch
   - Require PR reviews
   - Require status checks

2. **Topics/Tags** (GitHub repository topics):
   - Add: `rust`, `godot`, `scripting-language`, `gamedev`

### Social/Communication (Optional)

- Tweet/post about release on social media
- Update any external documentation links
- Announce in relevant communities (Rust Discord, Godot forums)

---

## Troubleshooting

### Tag Already Exists

```powershell
# Delete local tag
git tag -d v0.0.2

# Delete remote tag (if pushed)
git push origin :refs/tags/v0.0.2

# Recreate tag (repeat Step 2)
```

### Wrong Commit Tagged

```powershell
# Move tag to correct commit
git tag -f -a v0.0.2 <commit-hash> -m "Release v0.0.2..."

# Force push (use with caution)
git push origin v0.0.2 --force
```

### Release Description Formatting Issues

- Use GitHub's Markdown preview
- Check heading levels (##, ###, ####)
- Ensure code blocks have blank lines before/after
- Verify emoji render correctly (🦀, ✨, 📚, etc.)

---

## Expected Timeline

- **Step 1-3**: 2-3 minutes (tag creation and push)
- **Step 4**: 5-10 minutes (GitHub release creation)
- **Step 5**: 2-3 minutes (verification)
- **Total**: ~10-15 minutes

---

## Notes

**Artifacts**: This release does not include binary artifacts (FerrisScript is an interpreted language integrated via GDExtension). If artifacts are desired in the future:

1. Build release binaries for each platform
2. Compress into archives (`.zip` for Windows, `.tar.gz` for Linux/macOS)
3. Attach to GitHub release via `gh release upload` or web interface

**Version Numbering**: v0.0.2 follows semantic versioning:
- Major: 0 (pre-1.0 development)
- Minor: 0 (no breaking changes yet)
- Patch: 2 (second patch release)

**Next Version**: v0.0.3 will focus on Editor Experience Alpha (enhanced diagnostics, VS Code polish, dev scripts)

---

## Checklist

Before considering release complete:

- [ ] Local repository updated (`git pull origin main`)
- [ ] Tag created (`git tag -a v0.0.2`)
- [ ] Tag pushed (`git push origin v0.0.2`)
- [ ] GitHub release created (via CLI or web)
- [ ] Release marked as "Latest"
- [ ] Release page verified (formatting, links, content)
- [ ] Tag appears in repository
- [ ] (Optional) Branch protection configured
- [ ] (Optional) Social/community announcement

---

**Created**: January 5, 2025  
**For Version**: v0.0.2  
**Last Updated**: January 5, 2025
