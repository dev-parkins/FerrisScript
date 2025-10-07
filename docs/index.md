---
layout: default
title: Home
nav_order: 1
description: "FerrisScript Documentation - A Rust-inspired scripting language for Godot"
permalink: /
---

# FerrisScript Documentation

**A Rust-inspired scripting language for the Godot game engine** 🦀✨

FerrisScript brings Rust's syntax clarity and safety concepts to Godot game development, providing a familiar experience for Rust developers while offering excellent error messages and tooling support.

---

## 🚀 Quick Links

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 20px 0;">

<div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
<h3>📖 Error Codes</h3>
<p>Comprehensive reference for all compiler errors with examples and fixes.</p>
<a href="ERROR_CODES.html">Browse Error Codes →</a>
</div>

<div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
<h3>🏗️ Architecture</h3>
<p>Learn about FerrisScript's compiler architecture and design decisions.</p>
<a href="ARCHITECTURE.html">View Architecture →</a>
</div>

<div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
<h3>❓ FAQ</h3>
<p>Frequently asked questions about FerrisScript usage and features.</p>
<a href="FAQ.html">Read FAQ →</a>
</div>

<div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
<h3>👨‍💻 Development</h3>
<p>Contributing guide and development workflow documentation.</p>
<a href="DEVELOPMENT.html">Development Guide →</a>
</div>

</div>

---

## 📚 Documentation Sections

### Error Documentation

FerrisScript provides comprehensive error documentation with:
- **Structured error codes** (E001-E499) organized by category
- **Detailed explanations** with common causes and examples
- **Cross-references** to related errors
- **Direct links** from compiler error messages

[**→ Browse All Error Codes**](ERROR_CODES.html)

### Planning & Roadmap

Current development planning for v0.0.3:
- [v0.0.3 Overview](planning/v0.0.3/README.html)
- [Phase 1: Error Codes](planning/v0.0.3/PHASE_1_ERROR_CODES.html)
- [Phase 2: Error Suggestions](planning/v0.0.3/PHASE_2_ERROR_SUGGESTIONS.html)
- [Phase 3: Error Documentation](planning/v0.0.3/PHASE_3_ERROR_DOCS_RECOVERY.html)

### Development Resources

- [Architecture Overview](ARCHITECTURE.html) - System design and component interaction
- [Development Guide](DEVELOPMENT.html) - Setup, building, and testing
- [FAQ](FAQ.html) - Common questions and answers
- [Troubleshooting](TROUBLESHOOTING.html) - Common issues and solutions

---

## 🎯 Version Status

**Current Version**: v0.0.2  
**In Development**: v0.0.3 (Error System Enhancements)

### v0.0.3 Progress

- ✅ **Phase 1**: Error Code System (E001-E499)
- ✅ **Phase 2**: Error Suggestions ("Did you mean?")
- ✅ **Phase 3A**: Documentation URLs in error messages
- ✅ **Phase 3B**: Cross-references in ERROR_CODES.md
- 🚧 **Phase 3C**: Parser error recovery (in progress)
- ⏳ **Phase 3D**: Multi-error reporting
- ⏳ **Phase 3E**: Integration and testing

[View detailed roadmap →](planning/v0.0.3/README.html)

---

## 🔍 Quick Error Code Lookup

Common error codes by category:

### Lexical Errors (E001-E099)
- [E001: Invalid Character](ERROR_CODES.html#e001-invalid-character)
- [E002: Unterminated String](ERROR_CODES.html#e002-unterminated-string)
- [E003: Invalid Number Format](ERROR_CODES.html#e003-invalid-number-format)

### Syntax Errors (E100-E199)
- [E100: Unexpected Token](ERROR_CODES.html#e100-unexpected-token)
- [E101: Invalid Top-Level Item](ERROR_CODES.html#e101-invalid-top-level-item)
- [E102: Expected Expression](ERROR_CODES.html#e102-expected-expression)

### Type Errors (E200-E299)
- [E200: Type Mismatch](ERROR_CODES.html#e200-type-mismatch)
- [E201: Undefined Variable](ERROR_CODES.html#e201-undefined-variable)
- [E202: Undefined Function](ERROR_CODES.html#e202-undefined-function)

### Runtime Errors (E400-E499)
- [E400: Cannot Assign to Immutable Variable](ERROR_CODES.html#e400-cannot-assign-to-immutable-variable)
- [E401: Undefined Variable (Runtime)](ERROR_CODES.html#e401-undefined-variable)
- [E413: Division by Zero](ERROR_CODES.html#e413-division-by-zero)

[**→ View All Error Codes**](ERROR_CODES.html)

---

## 🛠️ Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/dev-parkins/FerrisScript.git
cd FerrisScript

# Build the project
cargo build --release

# Run tests
cargo test --workspace
```

### Quick Example

```rust
// hello.ferris
fn _ready() {
    print("Hello from FerrisScript!");
}
```

### Error Messages

FerrisScript provides helpful error messages with source context:

```
error[E201]: Undefined variable
  --> example.ferris:5:10
   |
 5 |     let x = velocty;
   |             ^^^^^^^ not found in this scope
   |
help: did you mean 'velocity'?
   = note: see https://github.com/dev-parkins/FerrisScript/blob/main/docs/ERROR_CODES.md#e201-undefined-variable for more information
```

---

## 🤝 Contributing

We welcome contributions! Check out:
- [Contributing Guide](https://github.com/dev-parkins/FerrisScript/blob/main/CONTRIBUTING.md)
- [Development Setup](DEVELOPMENT.html)
- [Open Issues](https://github.com/dev-parkins/FerrisScript/issues)

---

## 📄 License

FerrisScript is distributed under the MIT License.

---

## 🔗 Links

- [GitHub Repository](https://github.com/dev-parkins/FerrisScript)
- [Issue Tracker](https://github.com/dev-parkins/FerrisScript/issues)
- [Discussions](https://github.com/dev-parkins/FerrisScript/discussions)

---

<p style="text-align: center; color: #666; font-size: 0.9em;">
Last updated: {{ site.time | date: "%B %d, %Y" }}
</p>
