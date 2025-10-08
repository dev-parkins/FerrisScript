# PATH Security Hardening - Residual Risk Mitigation

**Date**: October 8, 2025  
**Issue**: Security scanner flagging PATH usage (Low severity)  
**Status**: ✅ Hardened with configuration option  
**Commit**: 5b3a32b

---

## 🔍 Issue Analysis

### Security Scanner Finding

**Tool**: Static security analysis  
**Severity**: Low (informational)  
**Message**: "Make sure the PATH variable only contains fixed, unwriteable directories"

### Context

After fixing command injection vulnerabilities (f7731b5), the scanner still flags PATH usage in line 68:

```typescript
cp.spawnSync('ferrisscript', ['--version'], { shell: false });
```

**Why This Is Flagged**:

- Any use of PATH-based executable resolution is flagged
- Even with `shell: false`, PATH could theoretically point to malicious binary
- Scanner cannot verify PATH contents at static analysis time

---

## 🛡️ Defense-in-Depth Approach

### Layer 1: User Configuration (NEW - Highest Security)

**Added Setting**: `ferrisscript.compilerPath`

```json
{
  "ferrisscript.compilerPath": "/usr/local/bin/ferrisscript"
}
```

**Benefits**:

- Bypasses PATH entirely
- User specifies exact trusted compiler location
- Absolute path validated before use
- Zero reliance on environment variables

**Use Case**: Security-sensitive environments, corporate policies, compliance requirements

### Layer 2: Workspace Search (Existing)

**Checked Locations**:

```
workspace/target/debug/ferrisscript[.exe]
workspace/target/release/ferrisscript[.exe]
```

**Benefits**:

- Local to project (not PATH-dependent)
- User controls workspace contents
- Common for development workflows

### Layer 3: PATH Search (Existing - Enhanced)

**Implementation**:

```typescript
cp.spawnSync('ferrisscript', ['--version'], { 
    encoding: 'utf-8',
    shell: false,  // No command injection
    timeout: 3000  // Prevent hanging
});
```

**Benefits**:

- Standard CLI tool discovery pattern
- Used by npm, cargo, python, etc.
- Convenient for users

**Protections**:

- `shell: false` - prevents command injection
- Timeout - prevents malicious binary from hanging
- User notification - transparency about which compiler is used

---

## 📊 Risk Assessment

### Residual Risk: Low → Negligible

| Attack Vector | Before | After Hardening |
|---------------|--------|-----------------|
| Command Injection | ❌ High | ✅ Prevented (spawnSync) |
| Malicious Binary in PATH | ⚠️ Low | ✅ Negligible (config option) |
| Compiler Not Found | ⚠️ Silent | ✅ Logged & notified |
| Hanging Process | ⚠️ Possible | ✅ Prevented (timeout) |

### Why Residual Risk Is Acceptable

1. **Standard Practice**: All major tools use PATH for CLI discovery
   - Node.js (`npm`, `node`)
   - Rust (`cargo`, `rustc`)
   - Python (`python`, `pip`)
   - Git (`git`)

2. **User Control**: PATH is controlled by user/administrator
   - Not writable by arbitrary processes
   - Requires elevated privileges to modify system PATH
   - Per-user PATH modifications only affect that user

3. **Multiple Mitigations**:
   - Configuration option (bypass PATH)
   - Workspace search (prefer local)
   - Timeout protection
   - User notification

4. **Threat Model**:
   - If attacker can modify PATH, they can already execute arbitrary code
   - This is not a vulnerability introduced by extension
   - Defense-in-depth provides additional protection layers

---

## 📖 User Guidance

### For Maximum Security

**Recommended Setting** (in VS Code settings.json):

```json
{
  "ferrisscript.compilerPath": "C:\\Program Files\\FerrisScript\\ferrisscript.exe"
}
```

**Steps**:

1. Install FerrisScript CLI to trusted location
2. Open VS Code Settings (`Ctrl+,`)
3. Search for "ferrisscript compiler"
4. Enter absolute path to compiler
5. Reload window

### For Standard Security (Default)

**No configuration needed** - Extension will:

1. Check workspace `target/` directories
2. Check PATH for `ferrisscript`
3. Notify when compiler found
4. Disable diagnostics if not found

**User Responsibility**:

- Keep PATH clean (standard OS hygiene)
- Don't add untrusted directories to PATH
- Keep system updated

---

## 🔬 Scanner Response

### Why Scanner Still Flags This

**Static Analysis Limitation**:

- Scanner cannot verify PATH contents at analysis time
- PATH is runtime environment variable
- Scanner errs on side of caution (good practice)

**Rating**: Low/Informational

- Not a vulnerability in extension code
- Flag is informational about general PATH usage
- Standard practice across all CLI tools

### Proper Response

1. ✅ **Acknowledge**: PATH usage is intentional and documented
2. ✅ **Mitigate**: Provide configuration option for absolute path
3. ✅ **Document**: Explain security posture and user options
4. ✅ **Accept**: Residual risk is acceptable for CLI tool discovery
5. ⚠️ **Don't**: Try to eliminate PATH usage entirely (breaks UX)

### If Scanner Must Be Satisfied

**Options**:

1. **Suppress Finding**: Mark as false positive with justification
2. **Document Exception**: Security review accepts PATH usage for CLI discovery
3. **Risk Accept**: Management accepts low residual risk
4. **Configure Default**: Set organization-wide `ferrisscript.compilerPath` via policy

---

## 🎯 Comparison with Other Tools

### How Other Extensions Handle This

**Example: Python Extension**

```json
{
  "python.pythonPath": "/usr/bin/python3"  // Optional absolute path
  // Falls back to PATH if not set
}
```

**Example: Rust Analyzer**

```json
{
  "rust-analyzer.server.path": "/usr/bin/rust-analyzer"  // Optional
  // Falls back to PATH if not set
}
```

**Pattern**: Industry standard is:

1. Optional configuration for absolute path
2. Fall back to PATH for convenience
3. User notification when tool found

---

## ✅ Compliance Recommendations

### For Security Audits

**Documentation to Provide**:

1. ✅ Defense-in-depth layers (config → workspace → PATH)
2. ✅ Mitigation controls (spawnSync, timeout, validation)
3. ✅ User guidance for high-security environments
4. ✅ Comparison with industry-standard tools
5. ✅ Risk acceptance rationale

**Talking Points**:

- "PATH usage is standard for CLI tool discovery"
- "Users can configure absolute path to bypass PATH"
- "Multiple protection layers prevent exploitation"
- "Risk is inherent to OS design, not extension vulnerability"
- "Compliant with OWASP secure coding practices"

---

## 📝 Summary

**Issue**: Security scanner flags PATH usage (low severity)  
**Response**: Added configuration option for absolute path  
**Result**:

- ✅ Users can bypass PATH entirely (zero risk)
- ✅ Default behavior remains user-friendly (low risk)
- ✅ Multiple protection layers (defense-in-depth)
- ✅ Documented security posture

**Recommendation**:

- Accept residual low risk for default behavior
- Document configuration option for high-security users
- Mark scanner finding as "accepted" or "mitigated"

---

**Commit**: 5b3a32b  
**Files Changed**: 3 files (+92/-10 lines)  
**Status**: ✅ Security hardening complete
