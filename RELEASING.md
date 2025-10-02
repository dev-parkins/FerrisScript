# FerrisScript v0.0.1 Release Guide 🦀

**Repository**: https://github.com/dev-parkins/FerrisScript  
**Date**: January 2025  
**Status**: Ready for initial release

---

## 📋 Pre-Release Checklist

- [x] All 96 tests passing
- [x] Documentation complete
- [x] Examples validated
- [x] License added (MIT)
- [x] Project fully rebranded to FerrisScript
- [x] GitHub repository created
- [x] CI/CD workflow configured
- [x] Archive organized by version
- [x] All URLs updated to correct repository
- [ ] Code pushed to GitHub
- [ ] Release tagged
- [ ] Release published

---

## 🚀 Step-by-Step Release Process

### Step 1: Add Remote and Push Code

```bash
# Add the GitHub remote (if not already done)
git remote add origin https://github.com/dev-parkins/FerrisScript.git

# Verify remote
git remote -v

# Push main branch to GitHub
git push -u origin main
```

**Expected Output**:

```
Enumerating objects: 500+, done.
Counting objects: 100%, done.
...
To https://github.com/dev-parkins/FerrisScript.git
 * [new branch]      main -> main
branch 'main' set up to track 'origin/main'.
```

---

### Step 2: Verify GitHub Actions CI/CD

After pushing, GitHub Actions will automatically:

1. **Run Tests** on Linux, Windows, macOS
   - Check: https://github.com/dev-parkins/FerrisScript/actions
   - Should see "CI/CD" workflow running
   - Wait for all tests to pass (green checkmarks)

2. **Build Release Artifacts**
   - Creates binaries for all platforms
   - Uploads as GitHub Actions artifacts

**If tests fail**: Check the Actions tab for detailed logs

---

### Step 3: Create Release Tag

Once CI passes on main branch:

```bash
# Create annotated tag for v0.0.1
git tag -a v0.0.1 -m "FerrisScript v0.0.1 - Initial Release

🎉 First stable release of FerrisScript!

Named after Ferris 🦀 (the Rust mascot), this release brings a 
Rust-inspired scripting language to Godot 4.x.

Features:
- Static typing with type inference
- Immutability by default (explicit mut)
- Full Godot 4.x GDExtension integration
- 96 passing tests
- 11 example scripts
- Comprehensive documentation

See RELEASE_NOTES.md for complete details."

# Verify tag
git tag -n1 v0.0.1

# Push tag to GitHub
git push origin v0.0.1
```

**Expected Output**:

```
To https://github.com/dev-parkins/FerrisScript.git
 * [new tag]         v0.0.1 -> v0.0.1
```

---

### Step 4: GitHub Actions Auto-Release

When the tag is pushed, GitHub Actions will automatically:

1. ✅ Run full test suite on all platforms
2. ✅ Build release binaries (optimized)
3. ✅ Create GitHub Release draft
4. ✅ Attach release artifacts:
   - `ferrisscript-linux-x86_64.so`
   - `ferrisscript-windows-x86_64.dll`
   - `ferrisscript-macos-x86_64.dylib`
   - `ferrisscript.gdextension`
5. ✅ Use RELEASE_NOTES.md as release description

**Check Progress**:

- Actions: https://github.com/dev-parkins/FerrisScript/actions
- Releases: https://github.com/dev-parkins/FerrisScript/releases

---

### Step 5: Publish GitHub Release

Once the release workflow completes:

1. **Go to Releases Page**:
   - https://github.com/dev-parkins/FerrisScript/releases

2. **Review Draft Release**:
   - Title: "FerrisScript v0.0.1"
   - Tag: `v0.0.1`
   - Description: From RELEASE_NOTES.md
   - Artifacts: All platform binaries attached

3. **Edit if Needed**:
   - Add additional release notes
   - Highlight breaking changes (none for v0.0.1)
   - Add screenshots or GIFs (optional)

4. **Publish Release**:
   - Click "Publish release" button
   - Release becomes public immediately

---

## 📦 Post-Release Tasks

### 1. Verify Release Assets

Check that all files are downloadable:

- [ ] `ferrisscript-linux-x86_64.so`
- [ ] `ferrisscript-windows-x86_64.dll`
- [ ] `ferrisscript-macos-x86_64.dylib`
- [ ] `ferrisscript.gdextension`
- [ ] Source code (zip)
- [ ] Source code (tar.gz)

### 2. Test Download and Installation

```bash
# Download release artifact
wget https://github.com/dev-parkins/FerrisScript/releases/download/v0.0.1/ferrisscript-linux-x86_64.so

# Verify file
file ferrisscript-linux-x86_64.so

# Test in Godot
# (Copy to Godot project and verify loading)
```

### 3. Update Project Status

- [ ] Add "Releases" badge to README
- [ ] Update RELEASE_NOTES.md checklist
- [ ] Create v0.1.0 milestone for next release
- [ ] Close v0.0.1 milestone (if created)

### 4. Announce Release

Consider announcing on:

- [ ] Godot Discord/Forum
- [ ] Reddit (r/godot, r/rust)
- [ ] Twitter/X with #godot #rustlang #gamedev
- [ ] Dev.to or personal blog

**Example Tweet**:

```
🎉 FerrisScript v0.0.1 is here!

A Rust-inspired scripting language for @godotengine 4.x 🦀

✅ Static typing
✅ Immutability by default  
✅ GDExtension integration
✅ 96 passing tests

https://github.com/dev-parkins/FerrisScript

#rustlang #gamedev #indiedev
```

---

## 🐛 Troubleshooting

### Issue: Git Push Fails with "Permission Denied"

**Solution**: Configure Git credentials

```bash
# Using GitHub CLI
gh auth login

# Or set up SSH key
ssh-keygen -t ed25519 -C "your-email@example.com"
# Add key to GitHub: Settings → SSH Keys
```

### Issue: GitHub Actions Fails

**Check**:

1. Workflow file syntax: `.github/workflows/ci.yml`
2. Rust version compatibility
3. Dependency availability
4. Platform-specific build issues

**Debug**:

```bash
# Run tests locally first
cargo test --workspace

# Check for platform-specific issues
cargo build --target x86_64-unknown-linux-gnu
cargo build --target x86_64-pc-windows-msvc
cargo build --target x86_64-apple-darwin
```

### Issue: Release Artifacts Missing

**Check**:

1. Workflow completed successfully
2. Artifact upload steps passed
3. Release job triggered by tag push
4. File paths correct in workflow

**Manual Upload**:
If automated upload fails, manually attach files:

```bash
# Build locally
cargo build --release

# Upload via GitHub web interface:
# Releases → Edit Release → Attach Files
```

---

## 📊 Success Metrics

After release, monitor:

- ⭐ **GitHub Stars**: Community interest
- 🍴 **Forks**: Developer engagement  
- 📥 **Downloads**: Release artifact downloads
- 🐛 **Issues**: Bug reports and feature requests
- 💬 **Discussions**: Community questions

---

## 🎯 Next Steps (v0.1.0)

After v0.0.1 release, plan for v0.1.0:

**Priority Features**:

1. Array/collection types
2. For loops
3. Match expressions  
4. More Godot types (Color, Rect2, etc.)
5. Signal support

**Tooling**:

1. Language Server Protocol (LSP)
2. Syntax highlighting plugin
3. VS Code extension

**Documentation**:

1. Tutorial series
2. API reference site
3. Video tutorials

---

## 📝 Release Notes Template (Future Releases)

```markdown
## v0.x.0 - Release Name (Month Year)

**Status**: Released  
**Tag**: `v0.x.0`

### 🎉 Highlights
- Major feature 1
- Major feature 2

### ✨ New Features
- Feature description

### 🐛 Bug Fixes
- Fix description

### 💥 Breaking Changes
- Breaking change description
- Migration guide

### 📚 Documentation
- Documentation improvements

### 🙏 Contributors
Thank you to all contributors!
```

---

**Ready to Release?** Follow the steps above to publish FerrisScript v0.0.1! 🚀

For questions or issues, open a discussion at:
https://github.com/dev-parkins/FerrisScript/discussions
