# Icon Theme Fix Verification Guide

**Issue**: Icon theme still showing all files without icons after fix applied
**Root Cause**: VS Code caches icon themes and requires manual reset

---

## 🔍 Current State

The icon theme JSON is correctly configured:

- ✅ No `"file"` property (was causing all files to use FerrisScript icon)
- ✅ Only maps `"ferris"` extension to custom icon
- ❌ VS Code may be using cached version

---

## 🛠️ Fix Steps

### Option 1: Clear Icon Theme Cache (Recommended)

1. **Disable Icon Theme**:
   - Press `Ctrl+Shift+P`
   - Type: "Preferences: File Icon Theme"
   - Select: "None (Disable File Icons)"
   - Wait 2 seconds

2. **Re-enable Icon Theme**:
   - Press `Ctrl+Shift+P`
   - Type: "Preferences: File Icon Theme"
   - Select: "FerrisScript"
   - Check file explorer

3. **Verify**:
   - `.ferris` files → Should have crab icon
   - `.md` files → Should have Markdown icon
   - `.ts` files → Should have TypeScript icon
   - All other files → Should have default icons

---

### Option 2: Reinstall Extension

If Option 1 doesn't work:

1. **Uninstall Extension**:
   - Use the Extensions panel in VS Code to uninstall "FerrisScript".
   - **Important:** After uninstalling, manually check for and delete any remaining extension folders:

     ```bash
     cd %USERPROFILE%\.vscode\extensions
     dir ferrisscript*
     rmdir /s ferrisscript-0.0.3
     ```

   - This ensures no cached files remain.

2. **Rebuild and Reinstall**:

   ```bash
   cd Y:\cpark\Projects\FerrisScript\extensions\vscode
   npm run compile

   # Copy to extensions folder
   xcopy /E /I . %USERPROFILE%\.vscode\extensions\ferrisscript-0.0.3
   ```

3. **Restart VS Code**:
   - Close all VS Code windows
   - Reopen workspace
   - Set icon theme: "FerrisScript"

#### ⚠️ Troubleshooting: Extension Not Showing as "Installing"

- If the extension does not appear as "installing" or does not show up in the Extensions panel:
  - Double-check the folder name in `%USERPROFILE%\.vscode\extensions` matches the expected format (`ferrisscript-0.0.3`).
  - Ensure `package.json` in the extension folder is valid and includes the correct `publisher`, `name`, and `version`.
  - Try running `Developer: Reload Window` from the Command Palette.
  - If still not detected, try packaging the extension (`vsce package`) and installing via VSIX.

---

### Option 3: Extension Development Host (Clean State)

If testing in development mode:

1. **Close Extension Development Host**

2. **Clear Extension Cache**:

   ```bash
   # Clear VS Code extension host cache
   rmdir /s %APPDATA%\Code\User\workspaceStorage
   ```

3. **Relaunch Development Host**:
   - Press `F5` in main VS Code window
   - Extension Development Host opens with clean state
   - Set icon theme: "FerrisScript"

---

## ✅ Expected Results After Fix

### File Explorer Icons

| File Type | Expected Icon | Notes |
|-----------|---------------|-------|
| `test.ferris` | 🦀 Crab icon | Custom FerrisScript icon |
| `README.md` | 📝 Markdown icon | VS Code default |
| `script.ts` | 📘 TypeScript icon | VS Code default |
| `data.json` | 📋 JSON icon | VS Code default |
| `config.toml` | ⚙️ TOML icon | VS Code default |
| Generic file | 📄 File icon | VS Code default |

### What Should NOT Happen

- ❌ All files showing crab icon (old bug)
- ❌ All files showing no icon (cache issue)
- ❌ Non-.ferris files showing crab icon

---

## 🐛 Troubleshooting

### Problem: All Files Show No Icons

**Cause**: Icon theme disabled or cache corrupted

**Fix**:

1. Check icon theme is enabled: `Ctrl+Shift+P` → "Preferences: File Icon Theme"
2. Try selecting a different theme (e.g., "Seti"), then back to "FerrisScript"
3. Restart VS Code

---

### Problem: All Files Still Show Crab Icon

**Cause**: Old version of extension still active

**Fix**:

1. Check extension version in Extensions view
2. Uninstall and reinstall extension (see Option 2)
3. Verify icon theme JSON file has no `"file"` property

---

### Problem: No Icon for .ferris Files

**Cause**: Icon theme not selected or SVG file missing

**Fix**:

1. Verify `extensions/vscode/resources/icons/ferrisscript.svg` exists
2. Select icon theme: `Ctrl+Shift+P` → "Preferences: File Icon Theme" → "FerrisScript"
3. Check `package.json` has correct `iconThemes` contribution

---

## 📋 Verification Checklist

After applying fix, verify:

- [ ] `.ferris` files show crab icon
- [ ] `.md` files show Markdown icon (or appropriate default)
- [ ] `.ts` files show TypeScript icon (or appropriate default)
- [ ] `.json` files show JSON icon (or appropriate default)
- [ ] Generic text files show file icon (or appropriate default)
- [ ] No other file types show crab icon
- [ ] Extension loads without errors

---

## 📝 Update Test Results

Once verified, update `PHASE_5_MANUAL_TESTING.md`:

**Test 13: File Icon Display**

- Change: `Result: [ ] Pass [X] Fail` → `Result: [X] Pass [ ] Fail`
- Update Notes: "Icon displays correctly after clearing VS Code icon theme cache"

---

## 🔍 Root Cause Analysis

**Why This Happened**:

1. Initial implementation had `"file": "ferrisscript-file"` in JSON
2. VS Code cached this configuration
3. Fix removed the line, but cache persisted
4. VS Code continued using cached version

**Prevention**:

- Always test icon themes in clean Extension Development Host
- Document cache clearing steps in testing guide
- Consider versioning icon theme in `package.json` to force reload

---

## 📚 Related Files

- Icon Theme JSON: `extensions/vscode/resources/icons/ferrisscript-icon-theme.json`
- Package Manifest: `extensions/vscode/package.json` (iconThemes contribution)
- SVG Icon: `extensions/vscode/resources/icons/ferrisscript.svg`
- Testing Guide: `docs/planning/v0.0.3/PHASE_5_MANUAL_TESTING.md` (Test 13)

---

**Status**: Awaiting user verification after cache clear
