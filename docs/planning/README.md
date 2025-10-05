# FerrisScript Release Planning 🦀

This folder contains detailed roadmaps for each development version on the path to v0.1.0.

**Purpose**: Strategic planning documents that break down the reprioritized v0.1.0 roadmap into actionable patch releases.

**Last Updated**: October 2025

---

## 📚 Available Roadmaps

### [v0.0.2 - Foundation & Polish](v0.0.2-roadmap.md)

**Timeline**: 2-3 weeks  
**Focus**: Community infrastructure, syntax highlighting, error improvements

**Key Deliverables**:

- ✅ Community files (CONTRIBUTING, CODE_OF_CONDUCT, etc.)
- 🔥 Syntax highlighting for VS Code
- GitHub project setup (labels, milestones, badges)
- Error message improvements (foundation)

**Status**: 🟡 Planning  
**Prerequisites**: v0.0.1 (released)

---

### [v0.0.3 - Editor Experience Alpha](v0.0.3-roadmap.md)

**Timeline**: 2-3 weeks  
**Focus**: Enhanced editor support and diagnostics

**Key Deliverables**:

- 🔥 Enhanced error diagnostics (error codes, "did you mean?")
- VS Code extension polish (completion, hover, problem panel)
- Development scripts (test.sh, bench.sh, format.sh, etc.)

**Status**: 🟡 Planning  
**Prerequisites**: v0.0.2 (syntax highlighting foundation)

---

### [v0.0.4 - Godot API Expansion](v0.0.4-roadmap.md)

**Timeline**: 3-4 weeks  
**Focus**: Expand Godot integration without new language features

**Key Deliverables**:

- 🔥 Signal support (define, emit, connect)
- Additional callbacks (_input, _physics_process, _enter_tree, _exit_tree)
- Node query functions (get_node, get_parent, has_node, find_child)
- Additional Godot types (Color, Rect2, Transform2D)
- Custom property exports (@export)

**Status**: 🟡 Planning  
**Prerequisites**: v0.0.3 (solid editor + error reporting)

---

### [v0.0.5 - LSP Alpha](v0.0.5-roadmap.md) ⚡ **CRITICAL MILESTONE**

**Timeline**: 4-5 weeks  
**Focus**: Language Server Protocol implementation

**Key Deliverables**:

- 🔥🔥🔥 **LSP server implementation** (HIGHEST PRIORITY)
- Real-time syntax checking
- Autocomplete (keywords, variables, functions)
- Go to definition
- Hover documentation
- VS Code LSP integration

**Status**: 🟡 Planning  
**Prerequisites**: v0.0.4 (complete editor + API foundation)

**Note**: This is the most critical release as LSP is the #1 priority in the reprioritized roadmap.

---

### [v0.0.6-7 - Language Features](v0.0.6-7-roadmap.md)

**Timeline**: 4-6 weeks (can be split into two releases)  
**Focus**: Core language features

**Key Deliverables**:

- Arrays/lists with methods
- For loops with ranges and iteration
- Match expressions for state machines
- String interpolation

**Status**: 🟡 Planning  
**Prerequisites**: v0.0.5 (LSP working)

**Note**: These are marked "optional" as they can:

- Be developed in parallel with v0.0.4-5
- Ship as v0.0.6 and v0.0.7 separately
- Be combined into a single release
- Be merged directly into v0.1.0

---

### [v0.1.0 - Production Ready](v0.1.0-release-plan.md) 🚀

**Timeline**: Final 2-3 weeks  
**Focus**: Integration, polish, release

**Key Deliverables**:

- LSP polish and advanced features
- Demo game (Pong or Breakout)
- Complete documentation (language reference, tutorials, API docs)
- Example projects (20+ scripts)
- Performance validation
- Quality assurance

**Status**: 🟡 Planning  
**Prerequisites**: v0.0.5 (LSP) + v0.0.6-7 (language features)

---

## 🎯 Strategic Overview

### Reprioritized Approach

The roadmap was **strategically reprioritized** to emphasize:

1. **🔥 Editor Integration First** (LSP, syntax highlighting)
2. **🎮 Godot API Coverage** (signals, callbacks, types)
3. **✨ Language Features** (arrays, loops, match) - in parallel

**Rationale**: Developers need excellent tooling and comprehensive Godot integration to be productive, even with basic language features.

### Critical Path

```
v0.0.2 (Foundation)
  ↓
v0.0.3 (Editor Diagnostics)
  ↓
v0.0.4 (Godot API)
  ↓
v0.0.5 (LSP Alpha) ⚡ CRITICAL
  ↓
v0.0.6-7 (Language Features)
  ↓
v0.1.0 (Production Ready) 🚀
```

### Parallel Development Opportunities

- **Language features** (v0.0.6-7) can be developed alongside v0.0.4-5
- **Documentation** improvements are continuous
- **Performance** optimization is ongoing

---

## 📊 Timeline Estimates

| Version | Timeline | Cumulative | Status |
|---------|----------|-----------|--------|
| v0.0.2 | 2-3 weeks | 2-3 weeks | 🟡 Planning |
| v0.0.3 | 2-3 weeks | 4-6 weeks | 🟡 Planning |
| v0.0.4 | 3-4 weeks | 7-10 weeks | 🟡 Planning |
| v0.0.5 | 4-5 weeks | 11-15 weeks | 🟡 Planning |
| v0.0.6-7 | 4-6 weeks | 15-21 weeks | 🟡 Planning |
| v0.1.0 | 2-3 weeks | 17-24 weeks | 🟡 Planning |

**Total**: ~17-24 weeks (~4-6 months)

**Note**: Timeline assumes sequential development. With parallel work on language features, could be reduced to ~15-20 weeks (~4-5 months).

---

## 🎯 Success Criteria

### v0.1.0 Goals

**Quantitative**:

- ✅ 200+ passing tests
- ✅ Test coverage ≥ 80%
- ✅ LSP features working (autocomplete, go-to-definition, hover)
- ✅ Editor support in VS Code
- ✅ Performance within 2x of GDScript
- ✅ 100% API documentation

**Qualitative**:

- ✅ Developer experience is "pleasant"
- ✅ First-class editor integration
- ✅ Can build simple 2D games
- ✅ Error messages are helpful
- ✅ Onboarding time < 10 minutes

---

## 📝 How to Use These Roadmaps

### For Contributors

1. **Check current version**: See which version is actively being developed
2. **Pick a task**: Find tasks marked as "High Priority" in the current version
3. **Understand dependencies**: Review prerequisites and dependencies
4. **Follow the plan**: Use the task breakdown and timeline as guidance

### For Project Planning

1. **Track progress**: Update status as versions complete
2. **Adjust timelines**: Revise estimates based on actual progress
3. **Prioritize work**: Focus on critical path items first
4. **Communicate status**: Use these documents for status updates

### For Users

1. **See what's coming**: Understand future features and timeline
2. **Provide feedback**: Comment on priorities and features
3. **Plan projects**: Know when features you need will be available
4. **Follow progress**: Track development through versions

---

## 🔗 Related Documentation

- [v0.1.0 Roadmap (Main)](../v0.1.0-ROADMAP.md) - Comprehensive feature roadmap
- [v0.0.2 Checklist](../v0.0.2-CHECKLIST.md) - Detailed task checklist
- [Version Planning](../VERSION_PLANNING.md) - High-level version strategy
- [Architecture](../ARCHITECTURE.md) - Technical architecture
- [Contributing](../../CONTRIBUTING.md) - How to contribute

---

## 📞 Questions or Feedback?

- **GitHub Issues**: For bugs or feature requests
- **GitHub Discussions**: For questions and ideas
- **Pull Requests**: For contributions

---

**Maintained by**: @dev-parkins  
**Last Review**: October 2025  
**Next Review**: After v0.0.2 release
