# .gitignore Setup Checklist

**Purpose**: Ensure proper .gitignore configuration before first commit  
**Created**: October 7, 2025  
**Applies To**: All new projects, folders, and extensions

---

## 🎯 Why This Matters

**Problem**: Committing build outputs, dependencies, or temporary files clutters git history and requires cleanup.

**Solution**: Set up `.gitignore` **before** first commit, then verify it's working.

**Time Investment**: 1-2 minutes  
**Time Saved**: 5-10 minutes cleanup + cleaner git history

---

## ✅ Checklist for New Projects/Folders

### 1. Create .gitignore First

**Before** creating any files in a new folder:

```bash
# Create .gitignore in new folder
touch .gitignore   # Linux/macOS
New-Item .gitignore   # PowerShell

# Or copy from template
cp templates/.gitignore-template ./.gitignore
```

### 2. Add Standard Exclusions

Based on project type:

#### TypeScript/Node.js Projects

```gitignore
# Dependencies
node_modules/
package-lock.json   # (optional, depends on team policy)

# Build outputs
dist/
out/
build/
*.js.map
*.d.ts

# IDE
.vscode/settings.json
.idea/

# OS
.DS_Store
Thumbs.db

# Logs
*.log
npm-debug.log*

# Environment
.env
.env.local
```

#### Rust Projects

```gitignore
# Build outputs
target/
Cargo.lock   # (for libraries; keep for applications)

# IDE
.vscode/settings.json
.idea/

# OS
.DS_Store
Thumbs.db

# Coverage
*.profraw
tarpaulin-report.html
cobertura.xml
```

#### VS Code Extensions

```gitignore
# Dependencies
node_modules/

# Build outputs
out/
dist/
*.vsix

# TypeScript
*.js.map
*.d.ts   # (if generated)

# Testing
.vscode-test/

# IDE
.vscode/settings.json

# Logs
*.log
```

### 3. Add Project-Specific Exclusions

```gitignore
# Temporary files (PR bodies, scripts)
/temp/
*.tmp
.pr-body-*

# Test outputs
coverage/
.nyc_output/

# Local configuration
local.config.json
```

### 4. Verify Before First Commit

**Critical Step**: Check that gitignore is working!

```bash
# Check what git will track
git status

# Should NOT see:
# - node_modules/
# - out/ or dist/
# - *.log files
# - temp/ folder

# If you see unwanted files:
# 1. Add them to .gitignore
# 2. Run git status again
# 3. Repeat until clean
```

### 5. Add and Commit .gitignore

```bash
# Add ONLY .gitignore first
git add .gitignore

# Commit it
git commit -m "chore: Add .gitignore for [project type]

- Exclude build outputs (out/, dist/, target/)
- Exclude dependencies (node_modules/, vendor/)
- Exclude IDE files (.vscode/settings.json)
- Exclude temporary files (temp/, *.tmp)"

# THEN add rest of files
git add .
git commit -m "feat: Initial [project] implementation"
```

---

## 🔧 If You Already Committed Unwanted Files

### Remove from Git Tracking (Keep Locally)

```bash
# Remove folder from git but keep locally
git rm -r --cached folder_name/

# Remove specific file pattern
git rm --cached "**/*.log"

# Commit the removal
git commit -m "chore: Remove [files] from git tracking

Files are now properly gitignored but were committed earlier.
Removed from git tracking but still exist locally."

# Push changes
git push
```

### Example: Remove out/ folder

```bash
# 1. Ensure out/ is in .gitignore
echo "out/" >> .gitignore

# 2. Remove from git tracking
git rm -r --cached out/

# 3. Verify
git status
# Should show: deleted: out/file1.js, deleted: out/file2.js

# 4. Commit
git commit -m "chore: Remove out/ folder from git tracking"

# 5. Push
git push
```

---

## 📋 Common Patterns by Technology

### Node.js / TypeScript

```gitignore
node_modules/
out/
dist/
build/
*.js.map
*.d.ts
.env
.vscode/settings.json
```

### Rust

```gitignore
target/
Cargo.lock
*.profraw
.vscode/settings.json
```

### Python

```gitignore
__pycache__/
*.py[cod]
.venv/
venv/
*.egg-info/
dist/
build/
.pytest_cache/
.coverage
```

### Go

```gitignore
# Binaries
*.exe
*.exe~
*.dll
*.so
*.dylib

# Test coverage
*.out
coverage.txt

# IDE
.vscode/settings.json
.idea/
```

---

## 🧪 Testing Your .gitignore

### Test 1: Create Ignored File

```bash
# Create a file that should be ignored
echo "test" > node_modules/test.txt   # (if node_modules/ exists)
# OR
mkdir temp && echo "test" > temp/test.txt

# Check git status
git status

# Should NOT see the file
```

### Test 2: Verify Patterns

```bash
# Check if pattern matches
git check-ignore -v filename

# Example:
git check-ignore -v out/extension.js
# Output: .gitignore:5:out/    out/extension.js
#         ^            ^        ^
#         |            |        matched file
#         |            pattern
#         file containing pattern
```

### Test 3: List Ignored Files

```bash
# Show all ignored files in directory
git status --ignored

# Should see:
# Ignored files:
#   node_modules/
#   out/
#   temp/
```

---

## ⚠️ Common Mistakes

### ❌ Mistake 1: Adding .gitignore After Committing

```bash
# Bad workflow:
git add .
git commit -m "Initial commit"   # Oops, committed node_modules!
echo "node_modules/" >> .gitignore   # Too late!
```

**Fix**: Always create .gitignore FIRST

### ❌ Mistake 2: Not Verifying .gitignore Works

```bash
# Bad workflow:
echo "out/" >> .gitignore
git add .   # Didn't check git status first!
git commit -m "..."   # Still committed out/ files!
```

**Fix**: Run `git status` before `git add`

### ❌ Mistake 3: Forgetting .gitignore in Subfolders

```bash
# Bad: .gitignore only in root
project/
  .gitignore   # Excludes root-level node_modules
  extensions/
    vscode/
      node_modules/   # Oops! This gets committed!
```

**Fix**: Add .gitignore in each folder with dependencies/builds

---

## 📊 Checklist Template

Copy this for new projects:

```markdown
## .gitignore Setup

- [ ] Create .gitignore BEFORE creating other files
- [ ] Add build outputs (out/, dist/, target/)
- [ ] Add dependencies (node_modules/, .venv/, vendor/)
- [ ] Add IDE files (.vscode/settings.json, .idea/)
- [ ] Add temporary files (temp/, *.tmp, *.log)
- [ ] Add OS files (.DS_Store, Thumbs.db)
- [ ] Run `git status` to verify
- [ ] Commit .gitignore first
- [ ] Then commit rest of files
```

---

## 🔗 Related Documents

- `PHASE_4_LESSONS_LEARNED.md` - Git hygiene lessons
- `.gitignore` templates: https://github.com/github/gitignore

---

## 📝 Quick Reference

**Do**:

1. ✅ Create .gitignore FIRST
2. ✅ Add exclusions for your tech stack
3. ✅ Run `git status` before committing
4. ✅ Commit .gitignore separately

**Don't**:

1. ❌ Commit everything then add .gitignore
2. ❌ Forget to verify .gitignore works
3. ❌ Commit without checking `git status`

**If you mess up**:

- Use `git rm -r --cached folder/` to remove from tracking
- Files stay locally, just removed from git

---

**Usage**: Follow this checklist at the start of every new project, folder, or extension. Takes 1-2 minutes, saves cleanup later.
