# FerrisScript v0.0.6: Language Features (Arrays, For-Loops, Match, String Interpolation)

**Status**: Scoping (written 2026-07-21, no implementation started)
**Timeline**: 1-3 sessions (see "On estimates" below — deliberately not given in calendar weeks)

---

## Why this exists

This is the first dedicated scoping doc for this version. Prior to 2026-07-21 it existed only as a four-bullet feature list inside `ROADMAP_MASTER.md`, with no syntax spec, no semantics decisions, and no reserved error-code range — in sharp contrast to the LSP plan (`docs/planning/v0.0.7/`, ~7,000 lines across 5 documents) that used to occupy this "next" slot. A 2026-07-21 roadmap review swapped the two: this version now ships first, because **FerrisScript currently cannot express a loop over a collection** — no arrays, no iteration, no pattern matching — which blocks every other goal (a real demo game, any external user trying the language, even giving LSP something non-trivial to diagnose later). See `ROADMAP_MASTER.md`'s v0.0.6 section for the full resequencing rationale.

## Goal

Give FerrisScript the minimum language surface needed to write a real, if simple, gameplay script — something with a list of enemies, an inventory, a state machine — end to end.

## On estimates

Do not estimate this in calendar weeks. The v0.0.7 (LSP) plan's "6-7 weeks" estimate was written in October 2025 under human-sprint-cadence assumptions and had already doubled from an original "3-4 weeks" before a single line of code was written — a signal the estimation *method* was unreliable, not just the number. Meanwhile the actual evidence from this project (v0.0.1→v0.0.4 shipped in 6 calendar days; the entire v0.0.5 stabilization + major gdext upgrade + release shipped in one AI-assisted session) shows multi-week-shaped work compressing into single-session work when this maintainer works with AI assistance. Track actual session count as you go and use it to calibrate v0.0.7's estimate afterward, rather than trusting either number in isolation.

---

## Current state (grounding this plan in the actual codebase, not assumptions)

- `crates/compiler/src/type_checker.rs::Type` enum: `I32, F32, Bool, String, Vector2, Color, Rect2, Transform2D, Node, InputEvent, Void, Unknown`. No `Array`/`List` variant exists.
- `crates/compiler/src/ast.rs::Stmt`: has `Let`, `Assign`, `If`, `While`, `Return`, `ExprStmt`, function/signal declarations. No `For`, no `Match`.
- `crates/compiler/src/ast.rs::Expr`: literals, binary/unary ops, calls, field access, struct literals. No indexing (`arr[0]`), no array literal, no match expression, no string-interpolation literal.
- Error codes currently run through **E815** (exports) with **E799/E800** as the highest reserved boundary before that range. Node-query errors are E601-E613, Godot-type errors are E701-E710. No range is reserved for arrays/loops/match/interpolation.
- `docs/planning/v0.0.5/../archive` and `crates/runtime/src/lib.rs`/`type_checker.rs` already track a few open bugs relevant here: **`Stmt::Return` doesn't validate the return type against the function signature** (issue filed 2026-07-21) — worth fixing *before or alongside* this work, since match-as-expression and function returns interact, and a soundness hole here will make match/array bugs harder to diagnose.

---

## Feature 1: Arrays

### Syntax decisions

- **Type syntax**: `[T]` (e.g. `[i32]`, `[Vector2]`) — matches the original roadmap bullet and Rust-family conventions the language already leans on.
- **Literals**: `[1, 2, 3]`. Empty array literal `[]` requires either a type annotation on the `let` (`let x: [i32] = [];`) or is a compile error without one (E9xx, "cannot infer type of empty array literal") — do not attempt bidirectional type inference for this version; require the annotation.
- **Indexing**: `arr[0]`. Out-of-bounds access is a **runtime error** (new error code, not a panic — consistent with the existing division-by-zero E413 pattern), not a silently-wrapped/clamped value.
- **Mutation semantics — the single most important open decision**: does `let mut arr: [i32] = [1,2,3]; arr.push(4);` mutate in place, or is `push` a method that returns a new array (functional style)? **Recommendation: in-place mutation, gated by `mut` the same way struct field mutation already works** (`let` = immutable/read-only, `let mut` = mutable — this pattern is already established for `@export` properties per `docs/ARCHITECTURE.md`). Functional/persistent arrays would be a bigger design commitment (immutable data structures, `arr.pushed(4)`-style non-mutating methods) that nothing else in the language currently does — don't introduce a novel mutation model just for arrays.
- **Methods for v0.0.6**: `len()`, `push(item)`, `pop() -> Option<T>` (or a runtime error if empty — decide alongside whether the language has any `Option`-like type yet; if not, prefer an error over inventing `Option` as a side effect of this work). `get(i) -> T` (bounds-checked, same error as `[i]` indexing) is redundant with `[i]` — skip it unless there's a reason to distinguish "may fail" access.
- **Explicitly out of scope for v0.0.6**: generic user-defined collection types, `HashMap`/dictionary types (the roadmap's `collections.ferris` placeholder conflates "arrays" with "collections" generally — this version is arrays only), slicing (`arr[1..3]`), array-of-arrays beyond what falls out naturally from `[T]` being recursively valid.

### Error codes (proposed range: E900-E919)

- E900: Array literal type mismatch (mixed element types)
- E901: Cannot infer type of empty array literal (no annotation)
- E902: Array index out of bounds (runtime)
- E903: Cannot index into non-array type
- E904: Array method called on non-array type

---

## Feature 2: For loops

### Syntax decisions

- **For-in**: `for item in array { ... }` — iterates by value (consistent with the language's existing "clone, don't reference-count" evaluation model per `docs/ARCHITECTURE.md`'s known limitations).
- **Range**: `for i in 0..10 { ... }` (exclusive end, matching Rust). Whether to also support inclusive `0..=10` is a nice-to-have, not required for v0.0.6 — cut it if time-constrained, the language is not trying to be a complete Rust clone.
- **Break/continue**: standard, no labeled loops (`'outer: for ...`) — the language has no `while`-loop labels today either, don't introduce label syntax for the first loop construct that needs it.
- **Scoping**: the loop variable (`item`, `i`) is scoped to the loop body only, immutable by default within the body unless the language's existing `mut` rules are extended to loop variables (recommend: loop variables follow the same immutable-by-default rule as `let`, since mutating the iteration variable of a `for-in` doesn't affect the underlying array anyway given by-value iteration above).

### Error codes (proposed range: E920-E939)

- E920: `for-in` target is not iterable (not an array or range)
- E921: `break`/`continue` used outside a loop

---

## Feature 3: Match expressions

### Syntax decisions

- **Match as expression** (not just statement) — `let x = match y { ... };` — per the original roadmap bullet. This is more work than statement-only match but is consistent with `if` already being usable as an expression-like construct in the language (verify this against the current parser before committing — if `if` is statement-only today, match should probably also start statement-only and become an expression in a later version, rather than match leapfrogging if in expressiveness).
- **Patterns for v0.0.6**: literal patterns (`match x { 1 => ..., 2 => ..., _ => ... }`) and the wildcard `_`. **Explicitly defer enum/struct pattern destructuring** — the language has no user-defined enums yet (only built-in struct types like `Color`/`Rect2`), so there's nothing to destructure beyond literals. Don't build pattern-matching infrastructure for a type system feature (enums) that doesn't exist yet.
- **Exhaustiveness checking**: required for literal-int/bool matches where the domain is enumerable in a useful way is impractical (i32 has 2^32 values) — in practice this means **exhaustiveness checking reduces to "does this match have a `_` wildcard arm or a `bool` match covering both `true`/`false`."** Don't try to build general exhaustiveness analysis (e.g. detecting `match x { 1 => .., 2 => .. }` on an i32 as "non-exhaustive" is trivially true and not useful to flag beyond "you're missing a `_` arm").

### Error codes (proposed range: E940-E959)

- E940: Non-exhaustive match (missing `_` wildcard, or missing a `bool` arm)
- E941: Unreachable match arm (duplicate literal pattern)
- E942: Match arms have inconsistent types (when used as an expression)

---

## Feature 4: String interpolation

### Syntax decisions

- **Syntax**: `"Hello {name}"` per the original roadmap bullet (Rust itself uses this exact syntax as of `format!("Hello {name}")` shorthand — consistent with the language's Rust-inspired positioning).
- **Expression support**: `"{x + 1}"` (arbitrary expressions inside braces) vs. identifier-only (`"{x}"`, no expressions). **Recommendation: identifiers only for v0.0.6**, matching the "String Interpolation (1-2 PRs)" small sizing already in the roadmap — full expression support inside string literals is a lexer/parser interaction (re-entering expression parsing mid-string-token) that's disproportionate scope for what's meant to be the smallest of the four features here. Expand to full expressions in a later version if identifier-only proves limiting in practice.
- **Escaping**: literal `{` / `}` via `{{` / `}}`, matching Rust's `format!` convention (keeps this consistent with the "Rust-inspired" pitch and avoids inventing a new escaping convention).

### Error codes (proposed range: E960-E969)

- E960: Undefined variable inside string interpolation
- E961: Unmatched `{` in string literal (missing `}` or invalid escape)

---

## Suggested implementation order

1. **Arrays** first — everything else (for-in, and eventually match on array contents) depends on there being something to iterate over. Also the most self-contained: new `Type::Array(Box<Type>)` variant, new `Expr::ArrayLiteral`/`Expr::Index`, new methods.
2. **For loops** second — directly depends on arrays existing (for-in) plus ranges (independent of arrays, could be done in either order, but pairing them keeps "iteration" as one coherent unit of work).
3. **Match expressions** third — independent of arrays/loops, could run in parallel if splitting work, but sequencing after them keeps error-code ranges and testing patterns consistent with what's just been established.
4. **String interpolation** last — smallest, most isolated (lexer-level change), least likely to interact with the other three.

Each feature should land as its own PR (or small PR sequence) with its own tests, mirroring the pattern already used for v0.0.4's phases (signals, lifecycle callbacks, node queries, etc. each shipped as separable PRs) rather than one large PR bundling all four.

## Test plan

- Unit tests in `crates/compiler/src/*.rs` for lexer/parser/type-checker coverage of each feature (matching the existing pattern — the compiler crate currently has 543+ tests structured this way).
- Runtime tests in `crates/runtime/src/lib.rs` for evaluation semantics (array mutation, loop execution, match dispatch, interpolation formatting).
- **At least one `godot_test/scripts/*.ferris` example per feature that exercises a real gameplay pattern** (an enemy list iterated with `for`, a simple state machine via `match`, a score/health display via string interpolation) — not just a syntax-demonstration script. This directly addresses a gap the July 2026 roadmap review flagged: current examples don't demonstrate real usage because the language couldn't express real usage yet.
- Ensure new `ferris-test` corpus scripts use the harness's structured `// ASSERT:` metadata format (literal substring matching), not free-form prose — a July 2026 tech-debt review found most of the current corpus's "failures" were exactly this mistake, not real bugs.

## Explicitly out of scope for v0.0.6

- Generics of any kind (arrays are `[T]` for a fixed concrete `T`, not `Array<T>` with user-provided type parameters beyond the one being iterated)
- Dictionaries/maps/hashmaps (tracked separately, likely v0.2.0+ per the "Extended Types" version)
- User-defined enums (match's pattern vocabulary stays literal-only until these exist)
- Full expression interpolation (`"{x + 1}"`) — identifiers only this round
- Slicing, array concatenation, sorting, or other higher-order array methods beyond `len`/`push`/`pop`

## Dependencies

v0.0.5 complete (shipped 2026-07-21).

## See also

- `ROADMAP_MASTER.md` — version summary, resequencing rationale
- `docs/planning/v0.0.7/` — the LSP plan this version displaced; its compiler-prerequisites work (spans, symbol table) is independent of this version and not a blocker
