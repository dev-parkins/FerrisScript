# FerrisScript Version Planning Summary 🦀

**Purpose**: High-level overview of version planning  
**Last Updated**: October 2025  
**Status**: v0.0.1 Released ✅

---

## 📦 Version Strategy

### Semantic Versioning (SemVer)

FerrisScript follows [Semantic Versioning 2.0.0](https://semver.org/):

```
MAJOR.MINOR.PATCH

Example: v0.1.2
         │ │ └─ Patch: Bug fixes, documentation (backward compatible)
         │ └─── Minor: New features (backward compatible)
         └───── Major: Breaking changes
```

### Pre-1.0 Versioning

During v0.X.Y (pre-stable):

- **v0.0.X**: Initial development, frequent breaking changes allowed
- **v0.X.0**: Feature releases, breaking changes minimized
- **v1.0.0**: First stable release, strict SemVer from here

---

## 🗺️ Version Roadmap

### ✅ v0.0.1 - "Ferris Awakens" (Released)

**Released**: October 2025  
**Status**: ✅ Complete  
**Focus**: Proof of concept

**Highlights**:

- Core language features (variables, functions, control flow)
- Basic Godot integration (GDExtension, callbacks)
- 96 tests, full CI/CD
- Complete documentation

**Deliverables**:

- [x] Working compiler, runtime, Godot binding
- [x] README, ARCHITECTURE, RELEASE_NOTES
- [x] Example scripts and test project
- [x] GitHub release with binaries

**Stats**:

- Tests: 96
- Example scripts: 11
- Documentation files: 15
- Lines of code: ~3,500

---

### 🔜 v0.0.2 - "Polish & Community" (Next)

**Target**: November 2025  
**Status**: 🟡 Planning  
**Focus**: Bug fixes, documentation, community infrastructure

**Goals**:

1. Fix bugs discovered in v0.0.1
2. Add community contribution infrastructure
3. Improve documentation and error messages
4. Add basic tooling (syntax highlighting)
5. **NO new language features**

**Key Deliverables**:

- [ ] CONTRIBUTING.md, CODE_OF_CONDUCT.md
- [ ] Issue/PR templates
- [ ] FAQ and troubleshooting guides
- [ ] VS Code syntax highlighting
- [ ] Improved error messages
- [ ] Test coverage reporting
- [ ] Performance benchmarks

**Success Criteria**:

- All tests passing (110+ tests)
- Test coverage ≥ 80%
- Zero clippy warnings
- Contributors can easily get started

**Estimated Timeline**: 10-15 days focused work

📋 **Historical Checklist**: [v0.0.2/v0.0.2-CHECKLIST.md](v0.0.2/v0.0.2-CHECKLIST.md) *(archived for reference)*

---

### 🚀 v0.1.0 - "Feature Complete Core" (Future)

**Target**: Q1-Q2 2026  
**Status**: 🔵 Future Planning  
**Focus**: New language features, enhanced Godot integration

**Goals**:

1. Make FerrisScript usable for real 2D games
2. Add essential language features (arrays, loops, match)
3. Improve developer experience (LSP, better errors)
4. Build demo game to validate use cases

**Key Features**:

**Language**:

- [ ] Arrays/lists (`[i32]`, indexing, push/pop)
- [ ] For loops (`for i in 0..10`)
- [ ] Match expressions (pattern matching)
- [ ] String interpolation (`f"Score: {score}"`)
- [ ] Enums (state machines)
- [ ] Structs (custom data types)

**Godot Integration**:

- [ ] More types (Color, Rect2, Transform2D)
- [ ] Signal support (emit, connect)
- [ ] Additional callbacks (_input,_physics_process)
- [ ] Custom properties (@export)
- [ ] Node query functions (get_node, get_children)

**Tooling**:

- [ ] Language Server Protocol (LSP)
- [ ] VS Code extension (autocomplete, errors)
- [ ] Hot reload
- [ ] Much better error messages

**Success Criteria**:

- Can build a complete 2D game (Pong/Breakout)
- 200+ tests passing
- LSP provides good developer experience
- Performance within 2x of GDScript

**Estimated Timeline**: 5-7 months (20-29 weeks)

📋 **Full Roadmap**: See [v0.1.0-ROADMAP.md](v0.1.0-ROADMAP.md)

---

### 🔮 v0.2.0+ - "Beyond" (Far Future)

**Target**: H2 2026 or beyond  
**Status**: 🔵 Conceptual  
**Focus**: Advanced features, 3D support, optimization

**Potential Features**:

- 3D game support (Camera3D, MeshInstance3D)
- Advanced tooling (debugger, profiler)
- Performance (bytecode, JIT compilation)
- Advanced language features (traits, generics)
- Package system

**Timeline**: 6-12 months after v0.1.0

---

## 📊 Feature Matrix

| Feature | v0.0.1 | v0.0.2 | v0.1.0 | v0.2.0+ |
|---------|--------|--------|--------|---------|
| **Language** | | | | |
| Variables (let/mut) | ✅ | ✅ | ✅ | ✅ |
| Basic types (i32, f32, bool, String) | ✅ | ✅ | ✅ | ✅ |
| Functions | ✅ | ✅ | ✅ | ✅ |
| If/else | ✅ | ✅ | ✅ | ✅ |
| While loops | ✅ | ✅ | ✅ | ✅ |
| Arrays | ❌ | ❌ | ✅ | ✅ |
| For loops | ❌ | ❌ | ✅ | ✅ |
| Match expressions | ❌ | ❌ | ✅ | ✅ |
| Enums | ❌ | ❌ | ✅ | ✅ |
| Structs | ❌ | ❌ | ✅ | ✅ |
| Traits | ❌ | ❌ | ❌ | 🤔 |
| **Godot** | | | | |
| GDExtension | ✅ | ✅ | ✅ | ✅ |
| Vector2, Node | ✅ | ✅ | ✅ | ✅ |
| _ready,_process | ✅ | ✅ | ✅ | ✅ |
| Self binding | ✅ | ✅ | ✅ | ✅ |
| More types (Color, etc.) | ❌ | ❌ | ✅ | ✅ |
| Signals | ❌ | ❌ | ✅ | ✅ |
| Custom properties | ❌ | ❌ | ✅ | ✅ |
| 3D support | ❌ | ❌ | ❌ | 🤔 |
| **Tooling** | | | | |
| Tests | ✅ | ✅ | ✅ | ✅ |
| CI/CD | ✅ | ✅ | ✅ | ✅ |
| Syntax highlighting | ❌ | ✅ | ✅ | ✅ |
| Test coverage | ❌ | ✅ | ✅ | ✅ |
| LSP | ❌ | ❌ | ✅ | ✅ |
| Hot reload | ❌ | ❌ | ✅ | ✅ |
| Debugger | ❌ | ❌ | ❌ | 🤔 |
| REPL | ❌ | ❌ | 🤔 | ✅ |

**Legend**: ✅ Implemented | 🤔 Considering | ❌ Not planned for this version

---

## 🎯 Development Philosophy

### Incremental Progress

- Small, frequent releases (v0.0.1, v0.0.2, v0.0.3...)
- Each patch builds confidence for next minor
- Multiple patches between minors (v0.0.X → v0.1.0)

### User-Driven Priorities

- Monitor GitHub Issues for pain points
- Community feedback shapes roadmap
- Real-world use cases guide features

### Quality Over Speed

- Every feature well-tested
- Documentation written with feature
- Performance validated
- Breaking changes minimized

### Sustainable Development

- Avoid burnout with realistic timelines
- Accept contributions to scale work
- Focus on maintainable code
- Technical debt addressed regularly

---

## 📋 Planning Documents

| Document | Purpose | Target Audience | Status |
|----------|---------|-----------------|--------|
| [v0.0.2/v0.0.2-CHECKLIST.md](v0.0.2/v0.0.2-CHECKLIST.md) | v0.0.2 release plan | Contributors | 📁 Archived |
| [v0.1.0-ROADMAP.md](v0.1.0-ROADMAP.md) | Feature roadmap for v0.1.0 | All stakeholders | 🎯 Active |
| [DOCUMENTATION_INVENTORY.md](DOCUMENTATION_INVENTORY.md) | Documentation audit | Maintainers | 📋 Reference |
| This file | High-level version strategy | Everyone | 📖 Living |

---

## 🚦 Current Status & Next Steps

### ✅ Just Completed

- v0.0.1 released successfully
- Logo and branding integrated
- CI/CD working perfectly
- All documentation up to date

### 🎯 Immediate Next Steps

1. **Community Engagement**
   - Announce v0.0.1 on Reddit, Twitter, Godot forums
   - Monitor for feedback and bug reports
   - Create GitHub Discussions topics

2. **Start v0.0.3 Planning** *(v0.0.2 is in progress)*
   - Create version-specific checklist in `docs/v0.0.3/`
   - Prioritize based on community feedback
   - Create GitHub issues for tasks
   - Set realistic timeline

3. **Begin Documentation Work**
   - Create CONTRIBUTING.md (high priority)
   - Create FAQ.md based on questions
   - Improve error messages
   - Add code coverage reporting

### 📅 Timeline Preview

```
October 2025     ✅ v0.0.1 Released
November 2025    🔜 v0.0.2 (planned)
December 2025    🔜 v0.0.3-4 (maintenance)
Q1 2026          🔜 v0.1.0-alpha releases
Q2 2026          🔜 v0.1.0 (target)
H2 2026          🔮 v0.2.0+ (TBD)
```

---

## 💡 Decision Framework

When planning features, ask:

1. **Does this enable real game development?**
   - If no: Defer to later version
   - If yes: Consider for v0.1.0

2. **Is this a breaking change?**
   - If yes: Bundle with other breaking changes
   - If no: Can ship in patch

3. **What's the implementation cost vs value?**
   - High value, low cost: Do soon
   - High value, high cost: Plan carefully
   - Low value, any cost: Defer or reject

4. **Does this maintain Rust's philosophy?**
   - Safety, speed, ergonomics
   - If it compromises these: Reconsider

---

## 🤝 Contributing to Roadmap

The roadmap is not set in stone! We welcome input:

1. **GitHub Discussions**: Share use cases and priorities
2. **GitHub Issues**: Propose specific features
3. **Pull Requests**: Implement planned features
4. **Community Feedback**: Tell us what matters most

**Roadmap Repository**: All roadmap documents are in `docs/` and tracked in git.

---

## 📚 Related Documentation

- [README.md](../README.md) - Project overview
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical design
- [RELEASE_NOTES.md](../RELEASE_NOTES.md) - Release history
- [RELEASING.md](../RELEASING.md) - How to create releases
- [v0.0.2/](v0.0.2/) - v0.0.2 archived documentation
- [v0.1.0-ROADMAP.md](v0.1.0-ROADMAP.md) - Feature roadmap

---

**The journey of a thousand features begins with a single commit!** 🦀✨
