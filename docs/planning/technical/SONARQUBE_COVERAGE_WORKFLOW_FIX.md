# SonarQube Coverage Integration - Workflow Fix

**Date**: October 8, 2025  
**Issue**: SonarQube showing 0.0% coverage despite LCOV generation  
**Root Cause**: Jobs running in parallel - SonarQube scans before coverage is generated  
**Status**: Fixed with workflow dependency chain

---

## 🐛 Problem Analysis

### Issue Reported

SonarQube dashboard showing **0.0% coverage** despite:

- ✅ Tarpaulin generating LCOV format (`--out Lcov`)
- ✅ `sonar-project.properties` configured with `sonar.rust.lcov.reportPaths=coverage/lcov.info`
- ✅ Coverage job completing successfully

### Root Cause Identified

**Timing Issue**: Jobs were running **in parallel**, not sequentially.

**Evidence from workflow run `18355233555`**:

```
Job: Code Coverage (Codecov)
  Started:   2025-10-08T19:04:38Z
  Completed: 2025-10-08T19:08:48Z  (4 min 10 sec)

Job: SonarQube Quality Scan  
  Started:   2025-10-08T19:04:38Z
  Completed: 2025-10-08T19:05:56Z  (1 min 18 sec)
```

**The Problem**:

1. Both jobs start at the **same time** (19:04:38Z)
2. SonarQube finishes **first** (19:05:56Z) - scans code with **NO coverage data**
3. Coverage generates **after** (19:08:48Z) - LCOV file created **too late**
4. SonarQube never sees the coverage file

**Why This Happened**:

Original workflow structure (before fix):

```yaml
jobs:
  sonarqube:    # No dependencies - starts immediately
    ...
  
  codecov:      # No dependencies - starts immediately
    ...
```

Both jobs are **independent**, so GitHub Actions runs them in **parallel** for speed.

---

## ✅ Solution Implemented

### New Workflow Structure

**Key Changes**:

1. **Renamed job**: `codecov` → `coverage` (more accurate name)
2. **Added dependency**: `sonarqube` now `needs: coverage`
3. **Artifact sharing**: Coverage reports uploaded and downloaded between jobs
4. **Separate PR job**: Lightweight SonarQube scan for PRs (no coverage needed)

### Workflow Diagram

**Before (Parallel Execution)** ❌:

```
[Push to develop]
     ├─→ [SonarQube Scan] (1.5 min) ✓ NO COVERAGE DATA
     └─→ [Generate Coverage] (4 min) ✓ Creates LCOV too late
```

**After (Sequential Execution)** ✅:

```
[Push to develop]
     └─→ [Generate Coverage] (4 min)
            └─→ Upload artifacts (cobertura.xml, lcov.info)
                ├─→ [Upload to Codecov]
                └─→ [SonarQube Scan] (1.5 min) ✓ HAS COVERAGE DATA
```

---

## 🔧 Technical Implementation

### 1. Coverage Job (Renamed from `codecov`)

```yaml
coverage:
  name: Generate Coverage Report
  if: |
    github.event_name == 'push' && 
    (github.ref == 'refs/heads/main' || github.ref == 'refs/heads/develop')
  runs-on: ubuntu-latest
  steps:
    # ... Generate coverage with tarpaulin ...
    
    # NEW: Upload as artifacts for other jobs
    - name: Upload coverage reports as artifacts
      uses: actions/upload-artifact@v4
      with:
        name: coverage-reports
        path: |
          coverage/cobertura.xml
          coverage/lcov.info
        retention-days: 7
    
    # Still upload to Codecov as before
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@...
```

**Why This Works**:

- Generates **both** Cobertura and LCOV in one run
- Uploads to Codecov immediately
- **Saves as artifacts** for SonarQube to download

---

### 2. SonarQube Job (Now Depends on Coverage)

```yaml
sonarqube:
  name: SonarQube Quality Scan
  needs: coverage  # ← CRITICAL: Wait for coverage to finish
  if: |
    always() &&
    github.event_name == 'push' && 
    (github.ref == 'refs/heads/main' || github.ref == 'refs/heads/develop')
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@...
      with:
        fetch-depth: 0
    
    # NEW: Download coverage reports from previous job
    - name: Download coverage reports
      uses: actions/download-artifact@v4
      with:
        name: coverage-reports
        path: coverage
    
    # Now scan with coverage data available
    - name: SonarQube Scan
      uses: SonarSource/sonarqube-scan-action@...
```

**Why This Works**:

- `needs: coverage` ensures this job **waits** until coverage completes
- Downloads artifacts to `coverage/` directory (same path as generation)
- `sonar-project.properties` points to `coverage/lcov.info` (now exists!)
- SonarQube scanner picks up coverage data correctly

---

### 3. PR-Only SonarQube Job (Lightweight)

```yaml
sonarqube-pr:
  name: SonarQube Quality Scan (PR)
  if: github.event_name == 'pull_request'
  runs-on: ubuntu-latest
  steps:
    # Simple scan without coverage (coverage job doesn't run on PRs)
    - uses: actions/checkout@...
    - name: SonarQube Scan
      uses: SonarSource/sonarqube-scan-action@...
```

**Why This Matters**:

- PRs don't generate coverage (by design - saves CI time)
- Still get quality scan (code smells, security)
- Separate job avoids dependency failures

---

## 📊 Expected Results

### After Next Push to Develop

**Execution Order**:

1. ✅ **Coverage job** runs (~4 minutes)
   - Generates `coverage/cobertura.xml`
   - Generates `coverage/lcov.info`
   - Uploads to Codecov
   - Saves as artifacts

2. ✅ **SonarQube job** runs (~1.5 minutes)
   - Waits for coverage to finish
   - Downloads coverage artifacts
   - Scans with coverage data present
   - **Coverage should show ~64%** (matching Codecov)

**SonarQube Dashboard Changes**:

- **Before**: Coverage: 0.0% (or N/A)
- **After**: Coverage: ~64.5% (matching tarpaulin report)

---

## 🔍 Verification Steps

### 1. Check Workflow Execution

After next push to develop:

```bash
gh run list --workflow="Code Scanning & Coverage" --limit 1
gh run view <run-id> --json jobs --jq '.jobs[] | {name, conclusion, startedAt, completedAt}'
```

**Expected**:

- `coverage` job completes first
- `sonarqube` job starts **after** coverage completes
- Both show `conclusion: success`

### 2. Verify Artifacts

```bash
gh run view <run-id> --log | grep -i "upload\|download"
```

**Expected**:

- "Upload coverage reports as artifacts" succeeds
- "Download coverage reports" succeeds
- Both files present: `cobertura.xml`, `lcov.info`

### 3. Check SonarQube Dashboard

Navigate to: <https://sonarcloud.io/project/overview?id=dev-parkins_FerrisScript>

**Expected**:

- **Coverage**: ~64.5% (previously 0.0%)
- **Quality Gate**: Should pass (if threshold ≤65%)
- **Coverage files**: 1 file processed (lcov.info)

### 4. Compare with Codecov

Navigate to: <https://codecov.io/gh/dev-parkins/FerrisScript>

**Expected**:

- Coverage percentages should match (±2%)
- Both use same source data (tarpaulin)

---

## 🚨 Potential Issues & Solutions

### Issue 1: Artifact Not Found

**Symptom**: `sonarqube` job fails with "Artifact not found"

**Cause**: `coverage` job failed or didn't upload artifacts

**Solution**:

- Check `coverage` job logs for tarpaulin errors
- Verify `coverage/` directory exists after generation
- Check artifact upload step succeeded

### Issue 2: Coverage Still 0.0%

**Symptom**: SonarQube shows 0% despite artifacts downloaded

**Causes**:

1. LCOV file path mismatch
2. LCOV file format issue
3. SonarQube property incorrect

**Solutions**:

```bash
# Check artifact contents
gh run view <run-id> --log | grep "coverage/"

# Verify LCOV file format (should start with "SF:")
cat coverage/lcov.info | head -5

# Double-check sonar-project.properties
grep lcov sonar-project.properties
```

### Issue 3: Job Dependency Loop

**Symptom**: Workflow never starts or jobs stuck pending

**Cause**: Circular dependency or incorrect `needs:` configuration

**Solution**:

- Verify `needs: coverage` only in `sonarqube` job
- No other jobs depend on `sonarqube`

---

## 📝 Files Changed

### Modified

**`.github/workflows/code-scanning.yml`**:

- Renamed `codecov` job → `coverage`
- Added artifact upload step (cobertura.xml + lcov.info)
- Added `sonarqube` job with `needs: coverage` dependency
- Added artifact download step in `sonarqube` job
- Added separate `sonarqube-pr` job for pull requests

### Not Changed

**`sonar-project.properties`**: No changes needed, already configured correctly

---

## 🎯 Success Criteria

This fix is successful when:

- [x] Workflow modified to use job dependencies
- [x] Coverage job uploads artifacts
- [x] SonarQube job downloads artifacts
- [ ] Next push to develop shows coverage execution order (coverage → sonarqube)
- [ ] SonarQube dashboard shows ~64% coverage (verify after next run)
- [ ] Coverage matches Codecov within ±2%
- [ ] No workflow errors or artifact issues

---

## 💡 Why This Fix Works

**Before**:

- Jobs ran in parallel (faster but wrong order)
- SonarQube scanned before coverage existed
- LCOV file never seen by SonarQube scanner

**After**:

- Jobs run sequentially (coverage → sonarqube)
- Coverage generates and saves LCOV file
- SonarQube downloads and scans with coverage
- Proper data flow ensures visibility

**Trade-off**:

- ⚠️ Slightly longer CI time (+30 seconds for artifact upload/download)
- ✅ Correct coverage reporting (worth the delay)

---

## 🔄 Related Documentation

**Updated**:

- This document (workflow fix explanation)

**Should Update**:

- `SONARCLOUD_COVERAGE_INTEGRATION.md` - Add workflow dependency details
- `CI_WORKFLOW_DUPLICATION_ANALYSIS.md` - Note coverage job renamed

---

**Last Updated**: October 8, 2025  
**Status**: Fix implemented, awaiting verification on next develop push  
**Next Action**: Monitor workflow run to verify coverage appears in SonarQube
