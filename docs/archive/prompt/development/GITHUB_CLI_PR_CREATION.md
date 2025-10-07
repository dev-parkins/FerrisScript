# GitHub CLI Pull Request Creation - Backtick Escaping Issue

**Issue Discovered**: October 7, 2025 (v0.0.3 Phase 4)  
**Context**: Automated PR creation using `gh pr create` with markdown body containing backticks

---

## Problem Summary

When using GitHub CLI (`gh pr create`) to create pull requests with markdown-formatted bodies, backticks (`` ` ``) used for inline code formatting are not properly escaped in PowerShell/Bash command strings, resulting in corrupted PR descriptions.

### Example Failure

**Intended Markdown**:
```markdown
- Type completion: `i32`, `f32`, `bool`, `String`
- Function: `print` with parameter hints
```

**Actual PR Output**:
```markdown
- Type completion: \i32\, \32\, \ool\, \String\
- Function: \print\ with parameter hints
```

**Root Cause**: Backticks are escape characters in PowerShell and special characters in Bash. When embedded in double-quoted strings passed to `gh pr create --body "..."`, they are interpreted/corrupted by the shell before reaching the GitHub CLI.

---

## Solutions

### Solution 1: Use a Temporary File (Recommended)

**Approach**: Write PR description to a file, then reference the file.

**Implementation**:

```powershell
# PowerShell
$prBody = @"
## Phase 4 Complete: Code Completion Provider

### Summary
Implements context-aware code completion with `keyword`, `type`, and `function` completion.

### Features
- Keyword completion: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`
- Type completion: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, `void`
- Function completion: `print(message: String)` with parameter hints

See `docs/PHASE_4_COMPLETION.md` for details.
"@

# Write to temporary file
$prBody | Out-File -FilePath ".\pr-body.txt" -Encoding UTF8

# Create PR using file
gh pr create --base develop --title "feat(vscode): Phase 4 Completion" --body-file ".\pr-body.txt"

# Clean up
Remove-Item ".\pr-body.txt"
```

**Bash Equivalent**:
```bash
# Bash
cat > pr-body.txt << 'EOF'
## Phase 4 Complete: Code Completion Provider

### Summary
Implements context-aware code completion with `keyword`, `type`, and `function` completion.

### Features
- Keyword completion: `fn`, `let`, `mut`, `if`, `else`, `while`, `return`
- Type completion: `i32`, `f32`, `bool`, `String`, `Vector2`, `Node`, `void`
- Function completion: `print(message: String)` with parameter hints

See `docs/PHASE_4_COMPLETION.md` for details.
EOF

gh pr create --base develop --title "feat(vscode): Phase 4 Completion" --body-file pr-body.txt

rm pr-body.txt
```

**Advantages**:
- ✅ No escaping needed - backticks work naturally
- ✅ Supports multi-line content easily
- ✅ Works identically in PowerShell and Bash
- ✅ Can be version controlled (optional)

**Disadvantages**:
- ⚠️ Requires file I/O
- ⚠️ Need to handle cleanup

---

### Solution 2: Escape Backticks for Shell

**Approach**: Properly escape backticks for the target shell.

**PowerShell Implementation**:
```powershell
# PowerShell: Double-escape backticks
$prBody = "## Summary`n`nFeatures:`n- Type: ``i32``, ``f32``"
gh pr create --base develop --title "Title" --body $prBody
```

**Bash Implementation**:
```bash
# Bash: Use single quotes to prevent interpretation
gh pr create --base develop --title "Title" --body 'Features:
- Type: `i32`, `f32`
- Function: `print`'
```

**Advantages**:
- ✅ No temporary files needed

**Disadvantages**:
- ❌ Shell-specific escaping rules (not portable)
- ❌ Complex multi-line strings
- ❌ Error-prone for large PR bodies

---

### Solution 3: Use GitHub API Directly

**Approach**: Bypass `gh` CLI and use GitHub REST API with proper JSON encoding.

**Implementation**:
```powershell
# PowerShell with Invoke-RestMethod
$prBody = @"
## Summary
Features:
- Type completion: `i32`, `f32`, `bool`
- Function: `print` with hints
"@

$headers = @{
    "Accept" = "application/vnd.github+json"
    "Authorization" = "Bearer $env:GITHUB_TOKEN"
    "X-GitHub-Api-Version" = "2022-11-28"
}

$body = @{
    title = "feat(vscode): Phase 4 Completion"
    body = $prBody
    head = "feature/v0.0.3-phase-4-completion"
    base = "develop"
} | ConvertTo-Json

Invoke-RestMethod -Uri "https://api.github.com/repos/dev-parkins/FerrisScript/pulls" `
    -Method Post `
    -Headers $headers `
    -Body $body `
    -ContentType "application/json"
```

**Advantages**:
- ✅ JSON encoding handles all special characters
- ✅ Full API control
- ✅ Programmatic access to PR metadata

**Disadvantages**:
- ❌ More complex
- ❌ Requires token management
- ❌ Less readable than `gh` CLI

---

## Recommended Workflow for Copilot

### For Automated PR Creation

Use **Solution 1 (Temporary File)** as the standard approach:

```powershell
# Create PR body in heredoc/here-string
$prBody = @"
## Summary

Full markdown content here with `backticks`, **bold**, and [links](url).

No escaping needed!
"@

# Write to file
$prBody | Out-File -FilePath ".pr-body-temp.txt" -Encoding UTF8

# Create PR
gh pr create --base develop `
    --title "feat: Your Feature Title" `
    --body-file ".pr-body-temp.txt"

# Cleanup
Remove-Item ".pr-body-temp.txt"
```

### Template for Phase PRs

Create reusable template in `docs/templates/PR_TEMPLATE_PHASE.md`:

```markdown
## Phase {N} Complete: {Feature Name}

### Summary
{Brief description}

### What's Included

#### Core Features
- Feature 1: `code examples`
- Feature 2: `more code`

#### Documentation
- `FILE1.md`: Description
- `FILE2.md`: Description

### Testing

**Manual Testing Guide**: `docs/planning/vX.X.X/PHASE_{N}_MANUAL_TESTING.md`

### Commits

1. `{hash}` - {commit message}
   - {details}

### Checklist

- [x] All tests passing
- [x] Documentation updated

---

**Related**: vX.X.X Phase {N}
**Reviewer Notes**: {notes}
```

**Usage**:
```powershell
# Copy template, replace placeholders
$template = Get-Content "docs/templates/PR_TEMPLATE_PHASE.md" -Raw
$prBody = $template -replace "{N}", "4" -replace "{Feature Name}", "Code Completion"

# Create PR from processed template
$prBody | Out-File -FilePath ".pr-body-temp.txt" -Encoding UTF8
gh pr create --base develop --title "Title" --body-file ".pr-body-temp.txt"
Remove-Item ".pr-body-temp.txt"
```

---

## Lessons Learned

1. **Always use `--body-file` for markdown PR descriptions** - avoids all shell escaping issues
2. **Test PR creation command in dry-run mode first** - use `gh pr create --web` to preview
3. **Use here-strings/heredocs for multi-line content** - more readable than escaped strings
4. **Consider PR templates** - standardize format, reduce errors
5. **For automation, prefer temporary files over inline strings** - simpler and more reliable

---

## Future Improvements

### Potential Enhancements for Copilot Automation

1. **Create `scripts/create-pr.ps1` helper script**:
   ```powershell
   # Usage: .\scripts\create-pr.ps1 -Title "Title" -BodyFile "pr-body.md"
   param(
       [Parameter(Mandatory=$true)]
       [string]$Title,
       
       [Parameter(Mandatory=$true)]
       [string]$BodyFile,
       
       [string]$Base = "develop"
   )
   
   gh pr create --base $Base --title $Title --body-file $BodyFile
   ```

2. **Add PR body validation**:
   - Check for required sections (Summary, Testing, Checklist)
   - Validate markdown syntax
   - Verify file references exist

3. **Generate PR body from commits**:
   - Extract commit messages from feature branch
   - Auto-generate commit list section
   - Parse conventional commit prefixes

4. **Integrate with todo list**:
   - Export completed todos to PR checklist
   - Auto-link documentation files mentioned in todos

---

**References**:
- GitHub CLI PR creation: https://cli.github.com/manual/gh_pr_create
- PowerShell here-strings: https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_quoting_rules
- GitHub API: https://docs.github.com/en/rest/pulls/pulls#create-a-pull-request

**Last Updated**: October 7, 2025
