# 🧭 FerrisScript Research Planning Board
>
> Comprehensive R&D roadmap for future ecosystem systems and integrations.
> Each section defines *why it matters*, *research goals*, and *proof of concept (PoC)* targets.

---

## 🧱 1. Build System Integration & Toolchain

### Why It Matters

Make FerrisScript feel native to Godot developers — no Rust boilerplate or CLI friction.

### Research Goals

- Define `cargo ferris` subcommand interface.
- Integrate with Godot `.import` and `.tscn` dependency graph.
- Implement incremental rebuild + hot reload system.
- Cache compiled GDExtension artifacts.

### Proof of Concept Ideas

- CLI prototype: `cargo ferris build --godot`.
- Hash-based build cache for `.ferris` → `.gdextension`.
- Godot plugin that triggers recompiles on save.

### References

- Cargo custom commands (`cargo-make`, `cargo-nextest`)
- Godot EditorPlugin system
- `cargo-godot` internals

---

## 🧠 2. Language Server (LSP) Infrastructure

### Why It Matters

Smooth editing experience with autocompletion, diagnostics, and type hints.

### Research Goals

- Implement `tower-lsp` backend.
- Support partial AST reparsing.
- Implement semantic tokens and hover docs.
- Integrate with compiler diagnostics in real time.

### Proof of Concept Ideas

- Minimal LSP server responding to “hover” requests.
- Live incremental parsing of single files.

### References

- `tower-lsp` docs
- `rust-analyzer` incremental architecture
- `salsa` dependency tracking library

---

## 🧩 3. Deterministic Runtime / Replay Systems

### Why It Matters

Differentiates FerrisScript from GDScript; vital for networking, simulation, and replays.

### Research Goals

- Design deterministic math + physics integration.
- Create replayable event log system.
- Generate per-frame state hashes for validation.

### Proof of Concept Ideas

- Record/replay small physics scene deterministically.
- Hash world state after 60 frames and compare runs.

### References

- Deterministic Lockstep Networking (Gaffer on Games)
- Fixed timestep physics in Bevy / Rapier

---

## 🧮 4. Static Type Reflection & Codegen System

### Why It Matters

Allows compile-time registration of signals, properties, and types.

### Research Goals

- Define reflection metadata schema.
- Implement codegen for Godot class registration.
- Expose schema to the editor for live type info.

### Proof of Concept Ideas

- JSON reflection output for sample `.ferris` script.
- Editor script that reads metadata and displays type info.

### References

- Unreal Header Tool
- Godot’s `ClassDB` and `SignalList`
- Rust `serde_reflection`

---

## 🧰 5. Plugin SDK & Ecosystem Layer

### Why It Matters

Sustainable ecosystem: users extend FerrisScript via plugins.

### Research Goals

- Expose compiler plugin API (AST visitors, passes).
- Define linting API for static checks.
- Package distribution: `cargo ferris install <plugin>`.

### Proof of Concept Ideas

- Simple plugin that adds a custom compile-time warning.
- Prototype plugin marketplace JSON manifest.

### References

- ESLint plugin architecture
- Rust Clippy internals
- Godot Asset Library metadata

---

## 🔬 6. Memory & Performance Diagnostics

### Why It Matters

Expose the performance advantages of Rust through in-editor tools.

### Research Goals

- Hook FerrisScript profiler into Godot’s profiler UI.
- Record memory allocations per node.
- Capture thread/job timings.

### Proof of Concept Ideas

- Basic profiler overlay showing per-node CPU time.
- Memory graph for FerrisScript objects.

### References

- Godot Profiler C++ API
- `tracing` crate for Rust instrumentation

---

## 🌐 7. Cross-Language Interoperability

### Why It Matters

Allow incremental adoption of FerrisScript in existing Godot projects.

### Research Goals

- Full Variant bridging layer (FerrisScript ↔ GDScript/C#).
- Type-safe API bindings.
- Resource wrapper generation.

### Proof of Concept Ideas

- Call GDScript function from FerrisScript.
- Pass struct to C# via Variant and inspect.

### References

- Godot Variant system
- `gdextension-bindings` docs
- Serde-based value conversions

---

## 🧩 8. Static Asset Pipeline Integration

### Why It Matters

Compile-time validation of assets prevents runtime crashes.

### Research Goals

- Detect missing assets during build.
- Type-safe resource linking macros.
- Integrate with Godot importer metadata.

### Proof of Concept Ideas

- `#[asset("res://icon.png")]` const validation macro.
- CLI that checks all referenced paths exist.

### References

- Unreal Asset Registry
- Bevy’s `AssetServer` and handles

---

## 🧮 9. Scene Graph Reflection Layer

### Why It Matters

Eliminate runtime “missing node” errors.

### Research Goals

- Compile-time validation of node paths and scene structure.
- Generate static “scene contract” files.
- Add node reflection types.

### Proof of Concept Ideas

- CLI that parses `.tscn` → JSON → verifies node paths.
- Compiler error if node not found at compile time.

### References

- Godot `SceneTree` API
- TypeScript scene graph validation in Godot projects

---

## ⚙️ 10. Incremental Compilation Infrastructure

### Why It Matters

Reduce compile latency, enabling near-instant feedback loops.

### Research Goals

- Implement dependency graph caching (via `salsa`).
- Persistent compiler daemon reused by LSP.
- Change impact analysis system.

### Proof of Concept Ideas

- Cache per-file parse results and recompile only changed.
- Benchmark incremental build times.

### References

- `salsa` crate
- `rust-analyzer` incremental design notes

---

## 🧩 11. Custom Debugger Integration

### Why It Matters

Provide a deterministic, structured debugging experience.

### Research Goals

- Step-by-step debugging with variable inspection.
- Replay timeline scrubber.
- Reverse execution capability (via deterministic replay).

### Proof of Concept Ideas

- Headless Godot scene replay with variable dumps.
- Simple breakpoint protocol over JSON-RPC.

### References

- Godot Debugger Protocol
- Rust `debug-adapter-protocol` crate

---

## 🎮 12. Gameplay Framework & Standard Library

### Why It Matters

Provide a productive out-of-the-box experience for developers.

### Research Goals

- Define `ferris_core`, `ferris_math`, `ferris_ai`, etc.
- Implement helper functions for scene access.
- Deterministic RNG utilities.

### Proof of Concept Ideas

- `ferris_core::signal` helper API.
- Deterministic timer system using fixed steps.

### References

- Bevy ECS helper APIs
- Godot `SceneTree` scripting helpers

---

## 📚 13. Documentation & Language Reference Pipeline

### Why It Matters

Documentation quality drives adoption.

### Research Goals

- Auto-generate docs from compiler metadata.
- Build interactive docs viewer.
- Integrate with Godot editor help panel.

### Proof of Concept Ideas

- `cargo ferris doc` command.
- Embed “Run Example” buttons in docs site.

### References

- Rustdoc JSON API
- Sphinx + MDBook integration

---

## 🤖 14. AI-Assisted Authoring & Guidance

### Why It Matters

Lowers barrier for newcomers and improves debugging experience.

### Research Goals

- Build error explanation catalog (like Rust’s `E0xxx` codes).
- Inline suggestion system (LLM-assisted, optional).
- Intelligent refactoring assistant for node renames and signals.

### Proof of Concept Ideas

- CLI tool `ferris explain E1023`.
- VSCode extension prototype using LSP + AI backend.

### References

- Rust error index
- GitHub Copilot Labs extension
- LangChain integration samples

---

# 📅 Prioritization Summary

| Priority | Focus Area | Impact | Complexity | Phase |
|-----------|-------------|---------|-------------|--------|
| 🥇 | LSP + Incremental Compiler | ⭐⭐⭐⭐ | 🧠🧠🧠 | 0.7 |
| 🥈 | Deterministic Runtime | ⭐⭐⭐⭐ | 🧠🧠🧠🧠 | 0.9 |
| 🥉 | Scene Graph Reflection | ⭐⭐⭐⭐ | 🧠🧠 | 1.0 |
| 🧩 | Asset Validation Pipeline | ⭐⭐⭐ | 🧠🧠 | 1.1 |
| 🧩 | Plugin SDK / Compiler API | ⭐⭐⭐ | 🧠🧠🧠 | 1.2 |
| 🧩 | Profiling & Debugger Integration | ⭐⭐⭐⭐ | 🧠🧠🧠 | 1.3 |
| 🧩 | AI & Docs Systems | ⭐⭐ | 🧠🧠 | 1.4 |

---

# 🧠 Meta Notes

- Each item should have its own `research/<topic>/README.md` folder for notes, references, and PoC experiments.
- Tag all experimental crates or modules with `#[cfg(feature = "experimental")]`.
- Track research status with labels: `planned`, `in_progress`, `validated`, `deferred`.

---

# 🧩 Deliverable Example (Template)

```markdown
## [Research Topic]
### Goal
[short summary]
### Questions
- [x] What existing systems solve this?
- [x] How would this integrate with FerrisScript architecture?
### Next Steps
1. Prototype
2. Measure
3. Validate integration
