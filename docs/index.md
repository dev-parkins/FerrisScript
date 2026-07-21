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
<a href="ERROR_CODES">Browse Error Codes →</a>
</div>

<div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
<h3>🏗️ Architecture</h3>
<p>Learn about FerrisScript's compiler architecture and design decisions.</p>
<a href="ARCHITECTURE">View Architecture →</a>
</div>

<div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
<h3>❓ FAQ</h3>
<p>Frequently asked questions about FerrisScript usage and features.</p>
<a href="FAQ">Read FAQ →</a>
</div>

<div style="border: 1px solid #ddd; padding: 15px; border-radius: 5px;">
<h3>👨‍💻 Development</h3>
<p>Contributing guide and development workflow documentation.</p>
<a href="DEVELOPMENT">Development Guide →</a>
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

[**→ Browse All Error Codes**](ERROR_CODES)

### Planning & Roadmap

See the [Master Roadmap](https://github.com/dev-parkins/FerrisScript/blob/main/docs/planning/ROADMAP_MASTER.md) for current version status and what's planned next.

### Development Resources

- [Architecture Overview](ARCHITECTURE) - System design and component interaction
- [Development Guide](DEVELOPMENT) - Setup, building, and testing
- [FAQ](FAQ) - Common questions and answers
- [Troubleshooting](TROUBLESHOOTING) - Common issues and solutions

---

## 🎯 Version Status

**Current Version**: v0.0.5 ("Stabilization & Engine Modernization")  
**Godot**: 4.2+ (tested against 4.7) · **gdext**: 0.5.4 · **Rust**: 1.94+ (Edition 2024)

[View detailed roadmap →](https://github.com/dev-parkins/FerrisScript/blob/main/docs/planning/ROADMAP_MASTER.md)

---

## 🔍 Quick Error Code Lookup

Common error codes by category:

### Lexical Errors (E001-E099)

- [E001: Invalid Character](ERROR_CODES#e001-invalid-character)
- [E002: Unterminated String](ERROR_CODES#e002-unterminated-string)
- [E003: Invalid Number Format](ERROR_CODES#e003-invalid-number-format)

### Syntax Errors (E100-E199)

- [E100: Unexpected Token](ERROR_CODES#e100-unexpected-token)
- [E101: Invalid Top-Level Item](ERROR_CODES#e101-invalid-top-level-item)
- [E102: Expected Expression](ERROR_CODES#e102-expected-expression)

### Type Errors (E200-E299)

- [E200: Type Mismatch](ERROR_CODES#e200-type-mismatch)
- [E201: Undefined Variable](ERROR_CODES#e201-undefined-variable)
- [E202: Undefined Function](ERROR_CODES#e202-undefined-function)

### Runtime Errors (E400-E499)

- [E400: Cannot Assign to Immutable Variable](ERROR_CODES#e400-cannot-assign-to-immutable-variable)
- [E401: Undefined Variable (Runtime)](ERROR_CODES#e401-undefined-variable)
- [E413: Division by Zero](ERROR_CODES#e413-division-by-zero)

[**→ View All Error Codes**](ERROR_CODES)

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
- [Development Setup](DEVELOPMENT)
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
