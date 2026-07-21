# Agentic Documentation

This directory contains agent-agnostic workflow documentation for AI coding assistants working on FerrisScript.

## 📋 Overview

FerrisScript supports multiple AI coding agents through a modern, standards-based documentation structure:

- **AGENTS.md** (root) — Universal entry point for all agents (~120 lines)
- **CLAUDE.md** (root) — Claude Code-specific notes (imports AGENTS.md)
- **docs/agent-skills/** — Progressive disclosure for deep crate work
- **.github/prompts/** — Workflow guides and execution patterns

## 🎯 Supported Agents

| Agent | Primary File | Status |
|-------|-------------|--------|
| **Claude Code** | `CLAUDE.md` → `AGENTS.md` | ✅ Full support |
| **OpenCode** | `AGENTS.md` | ✅ Full support |
| **GitHub Copilot** | `AGENTS.md` | ✅ Full support |
| **Cursor** | `.cursor/rules/` + `AGENTS.md` | ✅ Full support |
| **DeepSeek / Others** | `AGENTS.md` | ✅ Full support |

## 📁 File Structure

```
.github/prompts/
├── README.md                          # This file
├── WORKFLOW.md                        # Agent-agnostic execution guide (2.0)
├── PR_TEMPLATE_SYSTEM.md              # Branch naming and PR automation
└── archive/
    └── workstream-execution.prompt.md # Legacy Copilot-specific prompt (2,307 lines)
```

## 🚀 Quick Start

### For Any Agent

1. **Read AGENTS.md** — Project-wide instructions (commands, architecture, conventions)
2. **Load relevant skill** — Read from `docs/agent-skills/` based on which crate you're working in:
   - `compiler-conventions.md` — Parser, type checker, AST patterns
   - `runtime-patterns.md` — Interpreter, scoping, builtins
   - `godot-bindings.md` — GDExtension integration
   - `testing-guide.md` — Test patterns and coverage
3. **Follow WORKFLOW.md** — Structured execution methodology

### For Claude Code

CLAUDE.md imports AGENTS.md and adds Claude-specific notes. Claude Code will automatically:

- Read CLAUDE.md at session start
- Follow @AGENTS.md import
- Load skills on demand from `docs/agent-skills/`

### For OpenCode

OpenCode reads AGENTS.md directly. No additional configuration needed.

### For Cursor

Cursor reads both `.cursor/rules/` and `AGENTS.md`. The rules directory contains FerrisScript-specific patterns.

## 📖 Documentation Philosophy

### Progressive Disclosure

We use a **three-layer architecture** to minimize context window usage:

| Layer | Always Loaded? | Content | Size |
|-------|---------------|---------|------|
| **Layer 1**: AGENTS.md / CLAUDE.md | Yes | Commands, conventions, boundaries | ~120 lines |
| **Layer 2**: docs/agent-skills/ | On demand | Crate-specific patterns | ~100 lines each |
| **Layer 3**: Full docs | On demand | ARCHITECTURE.md, DEVELOPMENT.md | Loaded when needed |

**Why this matters:**

- Bloated files (>200 lines) cause agents to drop instructions
- Every token in AGENTS.md competes with the actual task for attention
- Progressive disclosure keeps context focused and relevant

### Size Limits

- **AGENTS.md**: ~120 lines (under 150 recommended)
- **CLAUDE.md**: ~50 lines (thin stub + Claude-specific notes)
- **Skill files**: ~100 lines each
- **WORKFLOW.md**: ~300 lines (comprehensive but focused)

**Rule of thumb:** Start with 30-50 lines. Add a line every time an agent makes the same mistake twice.

## 🔄 Migration from Legacy System

### What Changed

**Old system (v1.0):**

- Single 2,307-line `workstream-execution.prompt.md`
- GitHub Copilot-specific syntax (`/prompt`, `#file:`, `manage_todo_list`)
- "Premium request" terminology
- Heavy repetition and over-engineering

**New system (v2.0):**

- Agent-agnostic WORKFLOW.md (~300 lines)
- Universal AGENTS.md entry point
- Progressive disclosure via skill files
- Multi-agent support (Claude Code, OpenCode, Cursor, etc.)

### Migration Guide

**If you were using the old system:**

1. **Replace** `/prompt #file:workstream-execution.prompt.md` with reading `WORKFLOW.md`
2. **Remove** Copilot-specific syntax from your prompts
3. **Use** AGENTS.md as the entry point for all agents
4. **Load skills** from `docs/agent-skills/` as needed

**The old file is archived** at `.github/prompts/archive/workstream-execution.prompt.md` for reference.

## 🎓 Best Practices

### For Agent Users

1. **Start with AGENTS.md** — It's the universal entry point
2. **Load skills on demand** — Don't load all skills at once
3. **Keep context focused** — Only load what's relevant to current task
4. **Follow WORKFLOW.md** — Structured approach reduces errors
5. **Document assumptions** — Use `⚠️ ASSUMPTION:` markers

### For Contributors

1. **Update AGENTS.md** when project-wide conventions change
2. **Update skill files** when crate-specific patterns change
3. **Keep files concise** — Under 150 lines for main files
4. **Test with multiple agents** — Ensure compatibility
5. **Follow progressive disclosure** — Link to docs, don't paste them

## 📊 Quality Standards

All agentic workflows must:

- ✅ Pass `cargo test --workspace`
- ✅ Pass `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- ✅ Pass `cargo fmt --all -- --check`
- ✅ Pass `npm run docs:lint` (for markdown changes)
- ✅ Maintain or improve test coverage
- ✅ Follow conventional commits
- ✅ Update CHANGELOG.md for user-facing changes

## 🔗 Related Resources

- **AGENTS.md** (root) — Universal agent instructions
- **CLAUDE.md** (root) — Claude Code-specific notes
- **docs/agent-skills/** — Progressive disclosure skills
- **CONTRIBUTING.md** — Human contribution guidelines
- **docs/DEVELOPMENT.md** — Development workflow
- **docs/ARCHITECTURE.md** — Technical design

## 📝 Version History

- **v2.0** (July 2026) — Agent-agnostic rewrite, progressive disclosure, multi-agent support
- **v1.0** (October 2025) — Initial GitHub Copilot-specific workstream system

---

**Last Updated**: July 2026  
**Maintainer**: FerrisScript Core Team
