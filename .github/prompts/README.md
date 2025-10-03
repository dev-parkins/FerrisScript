# GitHub Copilot Workstream Prompts

This directory contains **generic, reusable prompts** for GitHub Copilot to execute complex, multi-phase workstreams following a consistent methodology.

---

## 📋 Purpose

**Workstream Execution Prompts** provide a repeatable framework for Copilot agents to:

- **Gather context** by asking clarifying questions first
- **Plan systematically** with detailed execution plans and acceptance criteria
- **Execute methodically** through well-defined phases with TODO tracking
- **Validate quality** with comprehensive testing and linting
- **Document learnings** in summary documents for future reference

---

## 🚀 How to Use

### Quick Start

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:docs/v0.0.2/v0.0.2-CHECKLIST.md
- Highlighted: [paste relevant checklist items or requirements]
- Priority: [High/Medium/Low]
```

**That's it!** Copilot will:

1. Read the prompt and your context
2. Ask clarifying questions to fill in gaps
3. Create an execution plan document
4. Work through phases systematically
5. Validate and document everything

---

## 🎯 When to Use

### ✅ Use workstream prompts for

- **Multi-phase work** with clear but complex requirements
- **Test implementation** (edge cases, integration, e2e)
- **Error handling improvements** (better messages, recovery, colorization)
- **Refactoring** that requires careful planning and testing
- **Documentation** overhauls or reorganizations
- **Bug fixes** that need systematic investigation

### ❌ Don't use for

- **Simple one-line fixes** ("fix this typo")
- **Quick questions** about existing code
- **Exploratory work** without clear goals
- **Trivial changes** that don't need planning

---

## 📂 Available Prompts

### Generic Workstream Execution

- **[workstream-execution.prompt.md](./workstream-execution.prompt.md)** ⭐ **PRIMARY**
  - **Type**: Generic, reusable for any workstream
  - **Usage**: `/prompt #file:workstream-execution.prompt.md` + context
  - **Features**: Question-driven context gathering, systematic execution
  - **Status**: Ready to use

### Legacy (Version-Specific)

- **[edge-case-error-handling-v0.0.2.md](./edge-case-error-handling-v0.0.2.md)**
  - **Type**: Version-specific (v0.0.2 only)
  - **Status**: Archived - use generic prompt instead
  - **Note**: Kept as an example of a complete context document

---

## 📖 Usage Examples

### Example 1: Edge Case Testing (from v0.0.2 checklist)

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:docs/v0.0.2/v0.0.2-CHECKLIST.md
- Highlighted: "Test edge cases: empty files, comments-only files, 
  long variable names, deeply nested expressions"
- Priority: High
```

**What happens:**

1. Copilot reads the prompt and checklist
2. Asks clarifying questions (test location? coverage target? prior work?)
3. Creates execution plan with Q&A recorded
4. Works through phases (tests → validation → docs)
5. Creates summary document with learnings

### Example 2: Error Handling Improvements

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:docs/v0.0.2/v0.0.2-CHECKLIST.md
- Highlighted: "Better error messages (line numbers, context), 
  colorized output, error recovery"
- Related: #file:docs/v0.0.2/LEARNINGS.md (prior work section)
```

### Example 3: Refactoring Work

```
/prompt #file:workstream-execution.prompt.md

Context:
- #file:crates/compiler/src/parser.rs (current implementation)
- Goal: "Refactor parser to support error recovery without breaking
  existing tests"
- Constraints: "No breaking API changes, maintain performance"
```

---

## 🎯 What Makes This Effective

### Question-Driven Context Gathering

Instead of requiring you to write a 700-line custom prompt, the agent **asks you questions** to build context:

- **About the work**: What's the goal? What version? What constraints?
- **About prior work**: What's been done? What patterns exist?
- **About quality**: What tests? What linting? What coverage?
- **About process**: Branch naming? Commit format? Documentation?

### Recorded Q&A

All questions and answers are **recorded in the execution plan**, so:

- Future contributors can see the reasoning
- The agent can refer back to decisions
- The project gains institutional knowledge

### Systematic Execution

The agent works through **well-defined phases**:

1. **Planning**: Ask questions, define criteria, create TODO list
2. **Implementation**: One phase at a time, with quality checks
3. **Validation**: All tests, all linting, all criteria
4. **Documentation**: Update docs, create summary, record learnings

---

## 📝 What You Need to Provide

### Minimum Context

- **What to work on** (checklist item, highlighted text, or description)
- **Where it belongs** (version, area of codebase)
- **Why it matters** (business value, technical debt, user impact)

### Optional Context (Agent Will Ask If Needed)

- Prior work documentation
- Related files or code
- Specific requirements or constraints
- Timeline or priority

### Example Minimum Invocation

```
/prompt #file:workstream-execution.prompt.md

Work on edge case tests from v0.0.2 checklist:
- Empty files
- Comments-only files  
- Long variable names
```

**That's enough!** Agent will ask for the rest.

---

## 🎓 Best Practices

### For Users (You)

1. **Provide context files** - Attach checklists, LEARNINGS.md, related docs
2. **Highlight specific work** - Copy relevant checklist items or requirements
3. **Answer questions promptly** - Agent will ask what it needs
4. **Review execution plan** - Approve before implementation starts
5. **Stay engaged** - Check TODO updates, provide feedback

### For Copilot Agents

1. **Ask questions first** - Don't assume, gather context
2. **Record Q&A** - Document all questions and answers in execution plan
3. **Use TODO lists** - Keep user informed of progress
4. **Test incrementally** - Run quality checks after each phase
5. **Document learnings** - Create summary with discoveries and recommendations

---

## 📊 Expected Flow

### Phase 0: Context Gathering (5-10 minutes)

```
User: /prompt #file:workstream-execution.prompt.md [context]
  ↓
Agent: Reads prompt + context
  ↓
Agent: Asks clarifying questions (5-10 questions)
  ↓
User: Answers questions
  ↓
Agent: Creates execution plan with Q&A recorded
  ↓
User: Approves plan
```

### Phase 1-N: Implementation (varies)

```
Agent: Updates TODO list (Phase X in progress)
  ↓
Agent: Works on tasks in phase
  ↓
Agent: Runs quality checks
  ↓
Agent: Updates TODO list (Phase X complete)
  ↓
[Repeat for each phase]
```

### Final Phase: Validation & Documentation

```
Agent: Runs full test suite + all linting
  ↓
Agent: Verifies all acceptance criteria met
  ↓
Agent: Updates all related documentation
  ↓
Agent: Creates summary document with learnings
  ↓
Agent: Ready for PR/commit
```

---

## 🔗 Related Resources

- [workstream-execution.prompt.md](./workstream-execution.prompt.md) - The main prompt file
- [CONTRIBUTING.md](../../CONTRIBUTING.md) - Contribution guidelines
- [docs/DEVELOPMENT.md](../../docs/DEVELOPMENT.md) - Development workflow
- [GitHub Copilot Documentation](https://docs.github.com/en/copilot)

---

## 🤝 Improving the System

After completing a workstream:

1. **Review the Q&A** - Were there gaps in the question template?
2. **Check deliverables** - Was anything missed?
3. **Evaluate efficiency** - Did the process work smoothly?
4. **Update the prompt** - Add new pitfalls, improve questions
5. **Update this README** - Add new examples if helpful

**Commit format**: `docs(prompts): improve [aspect] based on [workstream] feedback`

---

## 📞 Questions?

- **How do I use this?** - See "Usage Examples" section above
- **What if my work doesn't fit?** - The prompt is generic, just provide your specific context
- **Can I customize the prompt?** - Yes! Fork it and adjust for your needs
- **What if Copilot asks too many questions?** - Provide more context upfront
- **What if Copilot doesn't ask enough?** - Remind it to ask about [specific area]

---

**Last Updated**: October 3, 2025  
**Maintainer**: FerrisScript Core Team
