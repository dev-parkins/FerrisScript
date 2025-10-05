# Branch Protection Configuration for FerrisScript

**Created**: October 5, 2025  
**For**: v0.0.2 Release - Phase 5A  

---

## Overview

This document describes the branch protection rules configured for the `main` branch to ensure code quality and prevent accidental changes.

---

## Protected Branch: `main`

### Configuration Steps

1. **Navigate to GitHub Settings**:
   - Go to: https://github.com/dev-parkins/FerrisScript/settings/branches
   - Click "Add branch protection rule"

2. **Branch Name Pattern**:

   ```
   main
   ```

3. **Protection Settings**:

   **Pull Request Requirements**:
   - ✅ **Require a pull request before merging**
     - Required number of approvals before merging: **1**
     - ✅ Dismiss stale pull request approvals when new commits are pushed
     - ❌ Require review from Code Owners *(not yet configured)*

   **Status Checks**:
   - ✅ **Require status checks to pass before merging**
     - ✅ Require branches to be up to date before merging
     - Required status checks:
       - `cargo test` *(when CI configured)*
       - `cargo clippy` *(when CI configured)*
       - `cargo fmt --check` *(when CI configured)*

   **Additional Settings**:
   - ✅ **Require conversation resolution before merging**
   - ✅ **Require signed commits** *(recommended for security)*
   - ✅ **Require linear history** *(prevents merge commits)*
   - ✅ **Require deployments to succeed before merging** *(if applicable)*

   **Force Push & Deletion**:
   - ❌ **Allow force pushes**: DISABLED
     - Nobody can force push to this branch
   - ❌ **Allow deletions**: DISABLED
     - Branch cannot be accidentally deleted

   **Automation**:
   - ✅ **Automatically delete head branches** after PR merge
     - Keeps repository clean

4. **Who Can Override** (if needed):
   - ✅ Repository administrators can bypass protections
   - ✅ Include administrators in protection rules *(recommended)*

---

## Workflow Impact

### Before Branch Protection

```bash
# This would work (but shouldn't!)
git checkout main
git commit -m "Direct commit to main"
git push origin main
```

### After Branch Protection

```bash
# This will be REJECTED by GitHub
git checkout main
git commit -m "Direct commit to main"
git push origin main
# Error: protected branch hook declined
```

**Required Workflow**:

```bash
# 1. Create feature branch
git checkout -b feature/my-work

# 2. Make changes and commit
git add .
git commit -m "Implement feature"

# 3. Push to feature branch
git push origin feature/my-work

# 4. Create Pull Request via GitHub
gh pr create --title "Implement feature" --body "Description"

# 5. Wait for review and approval

# 6. Merge PR (automatic or via GitHub UI)
# Feature branch automatically deleted after merge
```

---

## Benefits

### Code Quality

1. **Mandatory Code Review**
   - All changes reviewed by at least one person
   - Knowledge sharing across team
   - Catches bugs before merging

2. **Automated Testing**
   - CI must pass before merge
   - Prevents broken code in main
   - Maintains test coverage

3. **Clean History**
   - Linear history easier to understand
   - Revert changes cleanly if needed
   - Better blame/bisect experience

### Safety

1. **No Accidental Changes**
   - Can't directly push to main
   - Can't force-push to rewrite history
   - Can't delete main branch

2. **Conversation Resolution**
   - All review comments must be addressed
   - Ensures issues are discussed
   - Documents decision-making

3. **Signed Commits** *(if enabled)*
   - Verify author identity
   - Prevents impersonation
   - Audit trail for compliance

---

## Testing Branch Protection

### Test 1: Direct Push (Should Fail)

```bash
# Try to push directly to main
git checkout main
echo "test" > test.txt
git add test.txt
git commit -m "Test direct push"
git push origin main

# Expected: 
# ! [remote rejected] main -> main (protected branch hook declined)
# error: failed to push some refs to 'github.com:dev-parkins/FerrisScript'
```

### Test 2: PR Without Approval (Should Block Merge)

```bash
# Create feature branch and PR
git checkout -b feature/test-protection
echo "test" > test.txt
git add test.txt
git commit -m "Test PR protection"
git push origin feature/test-protection
gh pr create --title "Test PR" --body "Testing branch protection"

# Try to merge immediately via web UI
# Expected: "Merging is blocked" - Requires 1 approval
```

### Test 3: PR With Approval (Should Succeed)

```bash
# Same PR as above
# Have another maintainer approve
# Expected: "Merge pull request" button becomes available
```

---

## Exceptions & Override

### When to Override

Branch protection can be temporarily bypassed by repository administrators in rare cases:

1. **Emergency Hotfix**
   - Critical production bug
   - No time for review process
   - Must document reason in commit message

2. **Initial Setup**
   - Setting up CI/CD pipelines
   - Bootstrapping automation
   - Adding branch protection itself

3. **Repository Maintenance**
   - Renaming branches
   - Restructuring repository
   - Migrating to new structure

### How to Override

1. **Temporarily Disable Protection**:
   - Go to Settings → Branches
   - Edit rule → Uncheck "Include administrators"
   - Make necessary change
   - Re-enable protection immediately

2. **Best Practice**:
   - Document reason in commit message
   - Notify team of override
   - Re-enable protections ASAP
   - Create follow-up issue to fix properly

---

## Maintenance

### Regular Reviews

**Quarterly** (every 3 months):

- Review protection settings
- Update required status checks
- Adjust approval requirements
- Check for new security options

**After CI Changes**:

- Update required status checks
- Test new workflows
- Verify protections still work

**As Team Grows**:

- Add CODEOWNERS file
- Require specific reviewers
- Add more required checks
- Increase approval count

---

## Related Documentation

- **CONTRIBUTING.md**: How to create PRs following protection rules
- **GITHUB_PROJECT_MANAGEMENT.md**: Overall GitHub workflow
- **CI/CD Documentation**: Status checks configuration *(when available)*

---

## References

- [GitHub Branch Protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches)
- [Requiring Status Checks](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches#require-status-checks-before-merging)
- [Signed Commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification)

---

## Status

**Current State**: ⏸️ Pending Manual Configuration

Branch protection rules must be configured via GitHub web interface by a repository administrator. This cannot be automated via GitHub CLI due to permission requirements.

**To Configure**: Visit https://github.com/dev-parkins/FerrisScript/settings/branches

**Next Steps After Configuration**:

1. Test protection with dummy PR
2. Document in CONTRIBUTING.md
3. Update team documentation
4. Screenshot configuration for reference
