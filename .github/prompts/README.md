# GitHub Copilot Workstream Prompts

This directory contains **generic, reusable prompts** for GitHub Copilot to execute complex, multi-phase workstreams following a consistent methodology.

---

## 📋 Purpose

**Workstream Execution Prompts** provide a repeatable framework for Copilot to:

- **Gather context** by asking clarifying questions first
- **Plan systematically** with detailed execution plans and acceptance criteria
- **Execute methodically** through well-defined phases with TODO tracking
- **Validate quality** with comprehensive testing and linting
- **Document learnings** in summary documents for future reference

---

## 🚀 Quick Start

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:docs/planning/v[VERSION]/checklist.md
- Highlighted: [paste relevant requirements]
- Priority: [High/Medium/Low]
```

**That's it!** Copilot will ask clarifying questions, create an execution plan, and work systematically through phases.

---

## 🎯 When to Use

| ✅ Use For | ❌ Don't Use For |
|------------|------------------|
| Multi-phase work with complex requirements | Simple one-line fixes |
| Test implementation (edge cases, integration) | Quick questions about code |
| Error handling improvements | Exploratory work without goals |
| Refactoring needing careful planning | Trivial changes |
| Documentation overhauls | Simple typo fixes |
| Systematic bug investigation | Quick experiments |

---

## 📂 Files in This Directory

- **[workstream-execution.prompt.md](./workstream-execution.prompt.md)** ⭐ **MAIN PROMPT**
  - Complete execution methodology
  - Question templates (25+ questions across 5 categories)
  - Quality checklists
  - Best practices and common pitfalls
  - LEARNINGS.md template
  - Deferral recommendations guidance
  - **Read this file** for full documentation

- **[PR_TEMPLATE_SYSTEM.md](./PR_TEMPLATE_SYSTEM.md)** 📋 **REFERENCE**
  - Branch naming conventions (`bugfix/*`, `feature/*`, `docs/*`)
  - PR template automation
  - Quick commands reference

- **[CONSOLIDATION_ANALYSIS.md](./CONSOLIDATION_ANALYSIS.md)** 📊 **INTERNAL**
  - Analysis of prompts folder organization
  - Consolidation decisions and rationale

---

## 📖 Usage Examples

### Example 1: Edge Case Testing

```
/prompt #file:workstream-execution.prompt.md

Work on edge case tests:
- Empty files
- Comments-only files
- Long variable names
```

### Example 2: Error Handling

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:docs/planning/v0.0.3/PHASE_3C_ERROR_RECOVERY.md
- Goal: Implement parser error recovery
```

### Example 3: Documentation

```
/prompt #file:workstream-execution.prompt.md

Reorganize v0.0.3 documentation by category
(dev guide, architecture, learnings)
```

---

## � Full Documentation

**See [workstream-execution.prompt.md](./workstream-execution.prompt.md)** for:

- Complete question templates
- Execution methodology
- Quality checklists
- Best practices
- Common pitfalls to avoid
- LEARNINGS.md template
- Deferral recommendations
- Detailed examples

**Everything Copilot needs is in that one file.**

---

## 🔗 Related Resources

- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Contribution guidelines
- [docs/DEVELOPMENT.md](../../docs/DEVELOPMENT.md) - Development workflow
- [GitHub Copilot Documentation](https://docs.github.com/en/copilot)

---

## 🤝 Improving the System

After completing a workstream, update the main prompt with learnings:

```bash
# Update prompt with new insights
git checkout -b docs/improve-workstream-prompt
# Edit .github/prompts/workstream-execution.prompt.md
git commit -m "docs(prompts): add [insight] from [workstream]"
```

**Single source of truth**: All execution guidance lives in `workstream-execution.prompt.md`

---

**Last Updated**: October 7, 2025  
**Maintainer**: FerrisScript Core Team
