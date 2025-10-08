# Security Policy

Thank you for helping to keep FerrisScript and its users secure! This document outlines our security policy and responsible disclosure guidelines.

## Supported Versions

We currently provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.0.1   | :white_check_mark: |

As the project evolves, we will update this table to reflect which versions receive active security support.

## Reporting a Vulnerability

If you discover a security vulnerability in FerrisScript, please report it responsibly. We appreciate your efforts to disclose your findings in a coordinated manner.

### How to Report

**Please DO NOT report security vulnerabilities through public GitHub issues.**

Instead, please report vulnerabilities using one of the following methods:

1. **Preferred: GitHub Security Advisories**
   - Navigate to the [Security Advisories page](https://github.com/dev-parkins/FerrisScript/security/advisories)
   - Click "Report a vulnerability"
   - Provide detailed information about the vulnerability

2. **Alternative: Email**
   - Send an email to: ferrisscript@gmail.com
   - Include "SECURITY" in the subject line
   - Provide as much information as possible (see below)

### What to Include

When reporting a vulnerability, please include as much of the following information as possible:

- **Description**: A clear and detailed description of the vulnerability
- **Impact**: Your assessment of the potential impact and severity
- **Reproduction Steps**: Step-by-step instructions to reproduce the issue
- **Affected Components**: Which parts of FerrisScript are affected (compiler, runtime, Godot bindings, etc.)
- **Proof of Concept**: Any relevant code, logs, screenshots, or proof-of-concept demonstrations
- **Suggested Fix**: If you have ideas for how to address the vulnerability, we welcome your input

### What to Expect

We take security issues seriously and will respond according to the following timeline:

- **Acknowledgment**: We will acknowledge receipt of your report within **48 hours** (2 business days)
- **Initial Assessment**: We will provide an initial assessment of the report within **5 business days**
- **Status Updates**: We will keep you informed of our progress as we investigate and develop a fix
- **Resolution**: Once a fix is identified, we will work to release a security update promptly
- **Disclosure**: After a fix is released, we will publish a security advisory and credit you as the reporter (unless you prefer to remain anonymous)

## Disclosure Policy

We follow a **coordinated disclosure** approach:

1. You report a vulnerability to us privately
2. We work with you to understand and validate the issue
3. We develop and test a fix
4. We release the fix in a security update
5. We publish a security advisory, crediting you for the discovery
6. After the advisory is published, you are free to discuss the vulnerability publicly

We ask that you:

- Give us reasonable time to address the vulnerability before public disclosure
- Avoid exploiting the vulnerability beyond what is necessary to demonstrate it
- Make a good faith effort to avoid privacy violations, data destruction, and service disruption

## Security Advisories

Published security advisories for FerrisScript can be found on our [GitHub Security Advisories page](https://github.com/dev-parkins/FerrisScript/security/advisories).

## Scope

This security policy applies to:

- **FerrisScript Compiler** (`crates/compiler`)
- **FerrisScript Runtime** (`crates/runtime`)
- **Godot GDExtension Bindings** (`crates/godot_bind`)
- **Example Scripts** (`examples/`)
- **Documentation** (if it contains security-relevant guidance)

For security issues in dependencies, we will work with upstream maintainers as appropriate.

## Security Best Practices

When using FerrisScript in your projects:

1. **Keep Updated**: Use the latest version of FerrisScript to ensure you have the latest security fixes
2. **Validate Input**: Validate and sanitize any external input to your FerrisScript scripts
3. **Least Privilege**: Run FerrisScript with the minimum necessary permissions
4. **Code Review**: Review scripts before deployment, especially if integrating third-party code
5. **Monitor Dependencies**: Keep Rust dependencies updated using `cargo update` and monitor advisories

## Contact

For security-related questions or concerns, please contact:

- **Email**: ferrisscript@gmail.com (with "SECURITY" in subject)
- **GitHub Security**: https://github.com/dev-parkins/FerrisScript/security/advisories

For general questions, feature requests, or non-security bugs, please use:

- **GitHub Issues**: https://github.com/dev-parkins/FerrisScript/issues
- **GitHub Discussions**: https://github.com/dev-parkins/FerrisScript/discussions

---

Thank you for helping keep FerrisScript secure!
