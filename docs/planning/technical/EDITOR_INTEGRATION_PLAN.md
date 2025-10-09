# FerrisScript Editor Integration Plan

**Status**: Technical Specification (Proposed)  
**Source**: Research agent suggestion  
**Date**: October 9, 2025  
**Purpose**: Technical blueprint for integrating FerrisScript into Godot editor and external editors

---

## ⚠️ Strategic Context

This document represents a **comprehensive vision** for editor integration that spans **multiple versions**:

- **v0.0.5**: LSP for external editors (VSCode, etc.)
- **v0.1.0**: Manifest generation system
- **v0.2.0**: Godot editor plugins (Inspector, Project panel)
- **v0.2.5+**: Debug telemetry and scene validation

**Key Insight**: This is NOT a single-version feature. It's a multi-phase roadmap that must be carefully sequenced to avoid dependency hell.

---

## 1. Goals & High-Level Requirements

**Primary goals**

- Surface compile-time diagnostics in the Godot editor.
- Show typed signals/properties in the Inspector and Node tab.
- Provide in-editor build/rebuild/test workflow.
- Enable scene-aware compile-time validation (tscn → FerrisScript).
- Provide a usable LSP for external editors; optionally mirror into Godot script editor.
- Provide runtime debug telemetry and minimal step-through mapping.

**Non-goals (initial)**

- Fully fledged visual debugger from day 1 (later roadmap).
- Deep, on-the-fly AST transforms inside the editor — compilation occurs via FerrisScript toolchain.

---

## 2. Manifest & Metadata Design

FerrisScript emits a manifest (prebuilt by the FerrisScript compiler) that Godot editor plugins read.

**Primary manifest file**: `res://ferris_manifest.json` (compiled by build step; committed or generated on build)

### JSON schema (example)

```json
{
  "version": "1.0",
  "package": "my_game",
  "nodes": {
    "FerrisScriptNode": {
      "signals": [
        {
          "name": "on_ready",
          "args": []
        },
        {
          "name": "on_message",
          "args": [
            { "name": "msg", "type": "String" }
          ]
        }
      ],
      "methods": [
        {
          "name": "do_action",
          "ret": "void",
          "args": [
            { "name": "power", "type": "i32" }
          ]
        }
      ],
      "properties": [
        { "name": "speed", "type": "f32", "default": 120.0 }
      ]
    }
  },
  "scenes": {
    "res://player.tscn": { "validated": true, "issues": [] }
  }
}
```

**Key points**

- `nodes` keyed by the binding class name used in `register_class()` (matches ClassDB name).
- `signals` list with arg names & basic types.
- `methods` & `properties` allow inspector and completion hints.
- `scenes` is optional output of static scene validation step.

**Variant type mapping** (used by editor to show types):

- `i32` → shown as `int`
- `f32` → `float`
- `bool` → `bool`
- `String` → `String`
- `Vector2` → `Vector2`
- `Ref<T>` or `Resource` → shown as `Resource` and open type hint

### 🚨 Dependency Alert: Manifest Generation (v0.1.0)

**Blocks**: All Godot editor integration features  
**Requires**: Compiler changes to emit JSON  
**Estimated Effort**: 3-4 premium requests  
**Risk**: Medium - new output format, needs schema design

---

## 3. Editor Plugin Architecture

We provide a small set of Godot plugins (GDExtension or script-based) that read the manifest and wire IDE features.

### Plugins

1. **FerrisProjectPlugin** (EditorPlugin)
   - UI: dock panel with `Build / Rebuild / Test / View Manifest` buttons.
   - Hooks to call external toolchain (e.g. run `cargo ferris build`).
   - Shows build output (console) in a docked terminal.
   - Registers `ferris_manifest.json` watchers to refresh UI on change.

2. **FerrisInspectorPlugin** (EditorInspectorPlugin)
   - When an object is selected whose script/class matches a `nodes` key in the manifest, shows typed properties & signal list.
   - Adds "Connect Signal" UI using manifest types (so when connecting, the editor suggests function signatures).
   - Shows `View Source` and `Go to Definition` links if source map available.

3. **FerrisSceneVerifier** (EditorScript or EditorPlugin)
   - Optional scene validation runner that parses `.tscn` (Godot's text scenes) and checks node existence/types referenced from manifest annotations (or from inline `#[scene]` annotations present in FerrisScript).
   - Writes results back to manifest under `scenes`.

4. **FerrisDebugPanel** (EditorPlugin)
   - Displays runtime metrics (if the running game publishes them).
   - Shows last compile artifacts and quick links to source.

### 🚨 Dependency Alert: Godot Plugin Development

**New Skillset Required**: GDScript or GDExtension plugin development  
**Estimated Effort**: 12-16 premium requests total (all 4 plugins)  
**Risk**: High - unfamiliar territory, limited documentation  
**Mitigation**: Start with minimal FerrisProjectPlugin first (v0.2.0)

### Implementation notes

- Plugins should be pure Godot plugins that call out to the FerrisScript CLI via `OS.execute()` or spawn background processes via `Process` (GDExtension). Use spawn+async to avoid blocking editor UI.
- Use `FileSystem` change watchers (or a simple timer poll) to reload manifest on changes.
- All plugins must gracefully degrade if `res://ferris_manifest.json` is missing (show helpful CTA: "Generate manifest by running `cargo ferris build`").

---

## 4. Build Tooling & CLI Hooks

**FerrisScript toolchain** should provide:

- `ferris build` or `cargo ferris build` → emits `.gdextension` or plugin artifact + `res://ferris_manifest.json`.
- `ferris lint` → outputs diagnostics in JSON for editor ingestion.
- `ferris test` → runs FerrisScript tests and reports pass/fail.

**Recommended manifest placement**

- Write manifest to `res://ferris/manifest.json` (subfolder) to avoid clutter; plugin reads this path via project setting `ferris.manifest_path`.

**Editor build invocation**

- `FerrisProjectPlugin` invokes build and parses JSON diagnostics:
  - On success: update manifest and show success toast.
  - On failure: show diagnostics in dock (with file/line links if path info present) and mark last build errors.

### 🚨 Dependency Alert: CLI Tooling (v0.1.0+)

**Blocks**: Editor plugin build workflow  
**Requires**: New CLI tool infrastructure  
**Estimated Effort**: 4-6 premium requests  
**Risk**: Medium - need to design CLI interface, JSON output format

---

## 5. LSP Integration (Language Server)

**Goal:** Provide external editor experience and optionally feed data to Godot's script editor.

### Server responsibilities

- Provide completions, hover docs, go-to-definition, workspace symbol search.
- Provide diagnostics by parsing FerrisScript compile output (errors/warnings).
- Optionally provide source → compiled mapping (for runtime debug mapping).

### Suggested protocol & tools

- Use **Language Server Protocol**; implement in Rust with `tower-lsp` (or similar).
- Provide capability to produce a `ferris_manifest.json` from a workspace request.
- Expose `ferris.lsp/manifest` custom notification to tell the Godot plugin where to read up-to-date metadata (optional).

### Example LSP features mapping

- `textDocument/publishDiagnostics` → compile-time errors for Godot editor or VSCode.
- `textDocument/completion` → completions using manifest `methods/properties/signals`.
- `textDocument/hover` → show docstring & type info derived from AST.
- `workspace/executeCommand` → run `ferris build` and return results.

**Note:** Godot's built-in script editor has limited LSP support; the recommended path is making LSP support for external editors (VSCode, Neovim) and feeding the manifest to the Godot plugin for in-editor UX.

### ✅ LSP v0.0.5 Scope (Confirmed)

**Focus**: External editors (VSCode) ONLY  
**No manifest dependency**: LSP can work with compiler output directly  
**Estimated Effort**: 11-16 premium requests (already in roadmap)  
**Risk**: Medium - new technology but well-documented

---

## 6. Inspector & Node Tab Integration Details

**Inspector enhancements**

- When a Node has a FerrisScript script:
  - If manifest lists `properties`, show typed editor controls (numeric slider, bool checkbox, resource pickers).
  - Provide a compact "FerrisScript" section summarizing signals & methods with quick "Connect" buttons.

**Node Tab (Signals)**

- Because Godot's Node dock reads ClassDB, we can't dynamically add signals at runtime for the editor. Instead:
  - The Rust GDExtension should call `builder.add_signal(...)` during `register_class()` using manifest data (as previously planned).
  - The plugin should ensure the manifest and GDExtension are in sync; show warning if manifest signals differ from loaded ClassDB.

**Editor connect flow**

- When connecting a signal via the Node tab, the Godot editor will show functions with matching signature. Since we register signals in `register_class()`, they appear normally.
- The `FerrisInspectorPlugin` enhances the connect dialog by allowing filters on manifest-driven method templates.

### 🚨 Dependency Alert: Metadata Registry (v0.1.0)

**Blocks**: Inspector integration, signal registration visibility  
**Requires**: `FerrisMetadataRegistry` in godot_bind crate  
**Estimated Effort**: 2-3 premium requests  
**Risk**: Low - straightforward Rust implementation

---

## 7. Debugging & Runtime Hooks

**Telemetry (lightweight)**

- Add optional runtime telemetry API in FerrisScript runtime that publishes to Godot's `Remote` or custom `DebugBus`:
  - `FerrisDebug.publish_metric(name: &str, value: Variant)`
  - `FerrisDebug.publish_event(name: &str, payload: VariantDict)`

**Debugging mapping**

- Ferris compiler should emit a source map (`.ferris.map.json`) mapping compiled function offsets to FerrisScript source lines.
- `FerrisDebugPanel` can display current call-site mapping when a frame event is received.

**Breakpoints & stepping (longer-term)**

- Prototype approach:
  - FerrisScript runtime listens for debug commands from `EditorPlugin` via WebSocket or local TCP.
  - Implement basic `pause/resume` and `stack` introspection first; full stepping later.

### 🚨 Dependency Alert: Debug Infrastructure (v0.2.5+)

**New Subsystem**: Runtime instrumentation  
**Estimated Effort**: 8-12 premium requests  
**Risk**: High - complex runtime integration, protocol design  
**Recommendation**: Defer to v0.2.5+ (not critical for v0.1.0)

---

## 8. Security & Safety Considerations

- **Editor plugin sandboxing:** Plugins invoking external processes must not expose unsafe behavior to users. Only run toolchain from project root.
- **Manifest trust model:** Treat manifest as build artifact; if missing/old, show clear warnings rather than silently assuming correctness.
- **Modding scenario:** If exposing FerrisScript to mods, provide restricted subset and ensure FS/OS calls are gated.

---

## 9. UX & Error Handling Guidelines

- **Graceful degrade:** When the manifest or build tool is missing, show clear CTAs: "Install FerrisScript toolchain" or "Run build".
- **Diagnostic linking:** Errors should include `file`, `line`, and `column` fields, and `FerrisProjectPlugin` should convert them to clickable links.
- **Non-blocking UI:** All external process calls must be async. Provide progress spinners and cancel buttons.

---

## 10. CI / Pipeline Integration

**CI tasks**

- `ferris build` in CI to ensure compile-time checks pass.
- `ferris lint` + `ferris test` as part of PR gating.
- Generate `ferris_manifest.json` as CI artifact (for release packaging).

**Packaging**

- Produce `ferris_gdextension.zip` or `.gdpack` that includes:
  - compiled extension binaries
  - `res://ferris/manifest.json`
  - editor plugin scripts (optional)
- Provide Godot Marketplace packaging instructions.

---

## 11. Example Implementation Snippets

### A. Editor plugin: respawn build (pseudo-GDScript for EditorPlugin)

```gdscript
# res://addons/ferris/plugin.gd
tool
extends EditorPlugin

var build_proc = null
var manifest_path = "res://ferris/manifest.json"

func _enter_tree():
    add_control_to_dock(DOCK_SLOT_RIGHT_UL, _build_dock())
    _watch_manifest()

func _on_build_pressed():
    # spawn build: cargo ferris build
    var args = ["ferris", "build", "--manifest-path", "project/Cargo.toml"]
    build_proc = Process.new()
    build_proc.launch("sh", ["-c", args.join(" ")], false)
    _show_console("Building...")

func _watch_manifest():
    # simple polling for example
    set_process(true)

func _process(delta):
    if FileAccess.file_exists(manifest_path):
        # reload and update UI
        _reload_manifest()
```

### B. Rust LSP startup command (example)

```
ferris-lsp --workspace /path/to/project --port 6009
```

- The LSP supports a custom request `workspace/ferrisManifest` returning the latest manifest JSON.

---

## 12. Testing Strategy for Editor Integration

- **Unit tests (Ferris toolchain)**
  - Manifest generation for many AST patterns.
  - Scene verification tests: test `.tscn` variations.

- **Integration tests (Editor Plugin)**
  - Mock manifest file and run plugin automation to ensure UI updates.
  - Simulate build failure and check plugin diagnostic display.

- **E2E tests**
  - Use headless Godot runs in CI to spawn editor, load plugin, and simulate a build flow (use Godot 4 headless + CLI automation).

---

## 13. Roadmap Alignment & Milestones

**Phase 5 (v0.1.0)**

- Ferris compiler emits `ferris_manifest.json`.
- FerrisProjectPlugin reads manifest and displays it.
- `register_class()` uses manifest via `FerrisMetadataRegistry` (Rust side) so signals show in Node tab.

**Phase 6 (v0.2.0)**

- Implement Ferris LSP with hover/completion.
- Add Inspector typed views and "Connect Signal" enhancements.

**Phase 7 (v0.2.5+)**

- FerrisSceneVerifier with compile-time scene checks.
- Telemetry & DebugPanel prototype.

---

## 14. Deliverables (first sprint)

1. `ferris_manifest.json` writer in the Ferris compiler.
2. `FerrisMetadataRegistry` reader in Rust GDExtension (already drafted).
3. `FerrisProjectPlugin` Godot plugin — build button + manifest viewer.
4. Minimal `FerrisInspectorPlugin` that lists signals & properties from manifest.
5. CI step to run `ferris build` and publish manifest as artifact.

---

## 15. Example Folders & file map (recommended)

```
/project-root
  /ferris
    manifest.json            # generated
    /src
      main.ferris
  /godot
    /addons
      /ferris_plugin
        plugin.cfg
        plugin.gd
        inspector_plugin.gd
  Cargo.toml
  ferris-config.toml
```

---

## 16. Final notes & recommended libs/tools

- Use `miette`/`ariadne` in Ferris compiler for nice diagnostics (helpful when rendering inside Godot).
- For LSP: `tower-lsp` or similar Rust LSP lib.
- For plugin communication: use JSON manifest + file watchers rather than ad-hoc sockets (simpler, cross-platform).
- Keep manifest format small & stable; version it (`"manifest_version": 1`) to allow forward changes.

---

## 🎯 Strategic Assessment (Tech Lead Analysis)

### What This Plan Gets Right ✅

1. **Separation of Concerns**: LSP for external editors vs Godot editor plugins
2. **Manifest-Driven Architecture**: Single source of truth for metadata
3. **Phased Approach**: Recognizes this spans multiple versions
4. **Graceful Degradation**: Plans for missing dependencies

### What Needs Adjustment ⚠️

1. **Scope Creep Risk**: This is a 6+ month vision, not a single feature
2. **Dependency Complexity**: Manifest blocks ALL Godot editor features
3. **Skillset Requirements**: Godot plugin development is new territory
4. **Testing Complexity**: E2E testing with headless Godot is ambitious

### Recommended Sequencing 📋

**v0.0.5 (Current Priority)**:

- ✅ LSP for external editors ONLY
- ✅ No manifest dependency
- ✅ Focus on VSCode extension

**v0.1.0**:

- ✅ Manifest generation system
- ✅ Metadata registry in GDExtension
- ✅ Basic CLI tooling (`ferris build`, `ferris lint`)

**v0.2.0**:

- ✅ FerrisProjectPlugin (build panel)
- ✅ FerrisInspectorPlugin (property/signal display)
- ✅ Enhanced LSP (workspace symbols, rename)

**v0.2.5+**:

- ⏳ FerrisSceneVerifier (scene validation)
- ⏳ FerrisDebugPanel (telemetry)
- ⏳ Debug infrastructure (breakpoints, stepping)

### Critical Dependencies to Track 🚨

1. **Manifest Generation** (v0.1.0) blocks:
   - Inspector integration
   - Signal registration visibility
   - All Godot editor plugins

2. **Metadata Registry** (v0.1.0) blocks:
   - Runtime signal registration
   - Editor property display

3. **CLI Tooling** (v0.1.0) blocks:
   - Editor build workflow
   - Diagnostic integration

4. **Scene Parser** (v0.2.5+) NEW SUBSYSTEM:
   - Need `.tscn` parser (Godot's text format)
   - Compile-time validation
   - High complexity, defer to later

5. **Debug Instrumentation** (v0.2.5+) NEW SUBSYSTEM:
   - Runtime hooks
   - Protocol design
   - High complexity, defer to later

### Risk Assessment 📊

| Component | Risk Level | Mitigation |
|-----------|------------|------------|
| LSP (v0.0.5) | Medium | Well-documented protocol, start minimal |
| Manifest Gen (v0.1.0) | Low | Straightforward JSON output |
| Godot Plugins (v0.2.0) | High | New skillset, limited docs - start small |
| Scene Validation (v0.2.5+) | High | New parser needed - defer |
| Debug Infra (v0.2.5+) | Very High | Complex runtime integration - defer |

---

## 📝 Document Status

**Purpose**: Technical specification and dependency analysis  
**Audience**: Implementation team (solo dev + future contributors)  
**Status**: Proposed - pending strategic approval  
**Maintenance**: Update after each version as features ship

**Related Documents**:

- `ROADMAP_MASTER.md` - Version sequencing
- `ROADMAP_CONSOLIDATION_ANALYSIS.md` - Strategic analysis
- `v0.0.5-roadmap.md` - LSP implementation details
- `v0.1.0-ROADMAP.md` - Metadata system plans

**Changelog**:

- 2025-10-09: Initial technical specification with dependency analysis

---

**Author**: Technical Lead (incorporating research agent suggestions)  
**Last Updated**: October 9, 2025
