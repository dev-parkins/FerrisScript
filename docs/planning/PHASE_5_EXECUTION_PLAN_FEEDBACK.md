Nice, solid plan — you’ve covered the big pieces. I read **PHASE_5_EXECUTION_PLAN.md** top-to-bottom and flagged everything that looks right, then called out inaccuracies, omissions, and places that need extra eyes or stronger decisions. I also give concrete, low-friction suggestions you can act on immediately.

---

# Quick summary (one-sentence)

The plan is overall correct and well-structured — main risks are Godot binding details, lifecycle/metadata placement (static vs per-instance), parser edge-cases, and a few semantic choices that should be explicitly decided now (e.g., clamp vs reject, default-value handling, when metadata is created). Addressing those will reduce rework in runtime/Godot integration.

---

# Things that are accurate / well done

* Sub-phase breakdown (Parser → Type Checker → Runtime/Godot) — correct ordering and low-risk approach.
* Test-first checkpoints and separating MVP from robustness tests — excellent.
* Error-code coverage (E801–E815) — thorough and appropriate split across parser/type/runtime.
* Explicit mention that struct literal support must be present for default values — good call.
* Listing exact files and functions to change — actionable and precise.

---

# Issues / inaccuracies / things that need attention

### 1) **Godot Inspector formats & flags — needs a concrete mapping**

You describe `hint_string` formats but don’t lock down exact serialization formats expected by Godot. Godot expects particular hint string encodings (e.g. `min,max,step` for range; comma-separated values for enum; comma-separated extension list for file). Make these exact and canonical in the plan (examples below). Also explicitly track `PropertyUsage` flags (e.g., `PROPERTY_USAGE_EDITOR`, `PROPERTY_USAGE_STORAGE`) because those affect persistence and whether the property shows in the Inspector.

**Action:** Add a small mapping table with examples (exact string formats) and recommended `usage` flags per property type.

---

### 2) **Metadata placement: runtime HashMap vs static (compile-time) metadata**

You recommended Option 1 (runtime HashMap) earlier; this plan uses HashMap in `ScriptInstance`. That works but has trade-offs:

* Runtime HashMap per instance: easy and flexible but more memory, potential duplicate metadata across many instances.
* Static (compile-time) metadata + per-instance runtime state for current values: much less memory and simpler `get_property_list()` because the engine can query the static registration.

**Recommendation:** Use a hybrid:

* **Static/compiled metadata** (shared, embedded in the Script binary) for structure: name, type, hint, hint_string, usage.
* **Per-instance store** for *current values* and ephemeral flags (dirty, edited-by-editor).
  This reduces memory and simplifies exposing property list (no need to reconstruct hint strings per instance).

**Action:** Update runtime design to plan for compile-time metadata + per-instance values.

---

### 3) **When is `get_property_list()` invoked? lifecycle problem**

Godot may query `get_property_list()` *before* script `init` or before runtime initialization. If metadata lives only in runtime fields initialized later, the Inspector won’t see properties. Your plan assumes metadata in `ScriptInstance` at runtime — ensure metadata is available at the earliest point Godot could query it.

**Action:** Ensure metadata registration occurs at script load time (static metadata available when Godot builds the inspector), not only at instance initialization.

---

### 4) **Default values & struct-literal defaults**

You list supporting struct-literal defaults in examples, but you need an explicit strategy for serializing those defaults into Godot `Variant`s (e.g., Vector2, Color, Rect2) and for storing them in static metadata. Also decide whether default struct literal must be a compile-time constant (recommended) or can be an expression (harder).

**Recommendation:** Only accept compile-time evaluable defaults for exported properties (literals or struct literals) for initial registration. If expression defaults are allowed, you must compute and register default at instance initialization.

**Action:** Add rule: defaults for exports must be constant literals or struct literals; otherwise default omitted and Inspector shows fallback.

---

### 5) **Clamp vs Reject vs Coerce semantics for range**

Plan states “Range hints clamp values in runtime.” That is a policy decision — Godot itself does not automatically clamp on set; the editor enforces ranges in the UI but runtime behavior depends on your code.

Options:

* **Clamp on set** (automatic) — friendly, but silently modifies values.
* **Reject on set** (error) — strict, surfaces incorrect sets.
* **Warn + clamp** — middle ground.

**Recommendation:** Default to **clamp** for Inspector UI sets (good UX), but **type-check/emit diagnostic** if script sets values programmatically out-of-range (or allow both with a configurable policy). Document exactly which choice you make.

**Action:** Add a small decision note in plan and tests for both Inspector-set and script-set behaviors.

---

### 6) **Enum & File hint representation**

Make explicit how enum values are serialized to Godot hint strings. Godot expects a comma-separated list without quotes. Similarly file hints commonly become a semicolon/comma-separated extension string. Decide whether to store extensions as `*.png,*.jpg` or `png,jpg`; be consistent.

**Action:** Add explicit example formats in the plan.

---

### 7) **Variant ↔ Value conversions for nested types**

You mention conversions but don’t call out edge-cases:

* Godot's `Variant` may represent `Vector2`, `Color`, `Transform2D`, or generic `Dictionary`. If a user edits a nested field in inspector, you must correctly convert between `Variant` and `Value`, including nested field updates without losing other fields.

**Action:** Add tests that round-trip Variant→Value→Variant for all struct types and for nested partial updates (e.g., set `rect.position.x` from inspector if supported).

---

### 8) **Property naming collisions & scope**

Plan doesn’t say what happens if two exports have the same name in the same script or across inherited scripts. Godot property names must be unique per object; decide on behavior (error, rename, namespace). Also consider exported properties on inner scopes or inside functions — you have E811 but add explicit rules.

**Action:** Add collision rules:

* Duplicate in same scope → error (E810)
* Shadowing across inheritance → either allowed with override semantics or error; pick and document.

---

### 9) **Test strategy: Godot headless integration**

Your runtime tests assume Inspector presence. Running these in CI requires a headless Godot or mocking. Plan should call out how you’ll run Inspector integration tests in CI (GDExtension unit tests, a Godot process, or mocked godot_bind). Without this note, integration tests will be flaky.

**Action:** Add recommended approach:

* Unit tests: mock godot_bind or test conversion helpers.
* Integration tests: run a headless Godot job or use GUT/GDExtension test harness in CI.

---

### 10) **Property persistence across script reloads / serialization**

You claim “Properties persist across script reloads” — that only holds if:

* You register as persistent (`PROPERTY_USAGE_STORAGE`) and/or
* Godot serializes the property into the scene resource (when saved).

Make explicit how you’ll mark properties for storage and ensure default values vs modified values are handled.

**Action:** Document `usage` flags and test that saving a scene preserves edited exported values.

---

### 11) **Mutability: exported `let` vs `let mut` and setter behavior**

Plan uses `let` everywhere. If you support immutable exports, what happens when the Inspector tries to set them? Decide:

* Disallow `@export` on immutable variables (or)
* Allow but create setter wrapper if variable marked mutable.

**Action:** Add a rule (and test) for immutability: either forbid Inspector sets on immutable or generate an internal setter with a diagnostic.

---

### 12) **Signals and change notifications (deferred)**

You defer property-change notifications, but this is a common pattern (emit `property_changed`). Since users will expect this, at least document a recommended pattern and tests for emitting signals from set handlers in the future.

**Action:** Add "future work" note and example pattern showing how user can do notifications manually until built-in support is added.

---

### 13) **Thread-safety and GDExtension conventions**

GDExtension calls may happen on specific threads; ensure conversions and hashmaps are safe and follow Godot runtime threading model. If your runtime uses interior mutability, ensure it's synchronized.

**Action:** Add a short TODO reminding implementers to check GDExtension thread expectations.

---

### 14) **Error messages & UX**

Some E80x messages may be generated at parse-time vs runtime. Ensure messages are actionable (example and quick-fix suggestion). E.g., E804 should suggest valid types for range.

**Action:** Add example messages for each error code (small code + explanation) — tests can assert on those.

---

# Concrete suggestions / small edits to add to the document

1. **Add mapping table for Godot `hint_string` formats** (example):

```
Range hint_string: "min,max,step"  // e.g. "0,100,1" or "0.0,20.0,0.5"
Enum hint_string: "Val1,Val2,Val3" // e.g. "Easy,Medium,Hard"
File hint_string: "*.png,*.jpg"     // e.g. "*.png,*.jpg"
```

2. **Change metadata design** to hybrid (compile-time metadata + per-instance value store). Add fields to `PropertyMetadata`:

```rust
pub struct PropertyMetadata {
    pub name: String,
    pub type_info: TypeInfo,
    pub hint: Option<PropertyHint>,
    pub default_variant: Variant,      // compile-time serialized default
    pub usage: PropertyUsageFlags,     // PROPERTY_USAGE_*
}
```

3. **Decide clamp policy** and add to plan (with tests for both inspector sets and script sets).

4. **Add CI note**: how to run integration tests (mocking vs headless Godot).

5. **Add rule for exported immutability**:

* If `let` (immutable) and `@export` → allowed, Inspector read-only.
* If `let mut` and `@export` → Inspector read/write.
  Or disallow immutable exports — pick one and add tests.

6. **Add duplicate-name resolution rule**: duplicate in same scope → E810; inherited override allowed with `override` annotation (future).

7. **Add lifecycle note**: Ensure `get_property_list()` pulls from static/compile metadata available at script load.

---

# Tests to add / strengthen (explicit)

* Round-trip Variant ↔ Value for all struct types and nested updates.
* Inspector set before `_ready()` — does the runtime apply the value correctly?
* Export of immutable variable: confirm Inspector is read-only or throws compile-time error.
* Save scene: write exported values in scene resource → reload scene → values preserved.
* Type coercion: setting `i32` property from Inspector with float Variant — either reject or coerce. Define behavior and test.
* Mocked Godot test harness: unit tests for `property_hint_to_godot()` string generation.

---

# Tiny example you can paste into the plan to be explicit (copyable)

Add a small **Godot hint mapping** box near the Godot Binding section:

```md
### Godot hint string & usage mapping (canonical)
- range(min,max,step) → hint_string: `"min,max,step"` (no parentheses)
  - example: `@export(range(0.0,20.0,0.5))` → `"0.0,20.0,0.5"`
- enum("A","B") → hint_string: `"A,B"`
  - example: `@export(enum("Easy","Medium"))` → `"Easy,Medium"`
- file("*.png","*.jpg") → hint_string: `"*.png,*.jpg"`
  - example: `@export(file("*.png","*.jpg"))` → `"*.png,*.jpg"`

For each exported property also set `usage` flags; recommended default: `PROPERTY_USAGE_DEFAULT | PROPERTY_USAGE_STORAGE`.
```

---

# Final recommendation & next step

You’re mostly ready to implement. Before coding, update the plan with:

1. The hybrid metadata approach (compile-time metadata + per-instance runtime values).
2. The clamp vs reject policy for ranges.
3. Exact `hint_string` formats and `usage` flags.
4. CI approach for Godot integration tests (mocking/headless).
5. A clear rule for immutability vs mutability + Inspector setter behavior.