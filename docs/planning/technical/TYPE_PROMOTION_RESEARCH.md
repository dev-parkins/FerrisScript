# Type Promotion & Numeric Type System Research

**Date**: October 7, 2025  
**Status**: Research & Planning  
**Target Version**: TBD (v0.1.0+ consideration)  
**Priority**: Medium-Low (Feature investigation, not blocking)

---

## 🎯 Research Objective

Investigate type promotion strategies for FerrisScript's numeric type system to:

1. Understand Godot's type expectations and compatibility
2. Analyze other game engines' numeric type systems
3. Evaluate automatic type promotion (32-bit → 64-bit) for overflow scenarios
4. Consider support for smaller types (8-bit, 16-bit) for performance
5. Leverage static typing advantages for explicit type control
6. Recommend roadmap placement for implementation

---

## 📊 Current State: FerrisScript v0.0.3

### Numeric Types Supported

- `i32` - 32-bit signed integer (-2,147,483,648 to 2,147,483,647)
- `f32` - 32-bit floating-point (IEEE 754 single precision)

### Current Limitations

- **No overflow protection**: `i32` arithmetic wraps on overflow (Rust default behavior)
- **No type promotion**: `i32 + i32 = i32` (even if result overflows)
- **No 64-bit support**: Cannot represent larger integers or higher-precision floats
- **No smaller types**: No `i8`, `i16`, `u8`, `u16` for memory optimization
- **Fixed at compile time**: No runtime type upgrading

### Example Problems

```rust
// Current FerrisScript behavior (matches Rust)
let big: i32 = 2000000000;
let result: i32 = big + big;  // Wraps to negative value (overflow!)

// What user might expect (with promotion)
let big: i32 = 2000000000;
let result: i64 = big + big;  // Auto-promotes to i64 = 4000000000
```

---

## 🎮 Game Engine Analysis

### 1. Godot Engine

#### GDScript (Dynamic Typing)

**Type System**:

- Uses `Variant` type internally (can hold any type)
- Integers: 64-bit signed (`int` = `i64` equivalent)
- Floats: 64-bit double-precision (`float` = `f64` equivalent)
- **No explicit 32-bit numeric types in GDScript**

**Type Behavior**:

```gdscript
# GDScript examples
var big = 2000000000
var result = big + big  # = 4000000000 (no overflow, uses i64)

var x = 5 / 2  # = 2.5 (automatic float conversion)
```

**Automatic Promotion**:

- Integer operations use 64-bit arithmetic (no overflow for most game values)
- Integer ÷ Integer → Float (automatic float promotion for division)
- Overflow: Wraps at 64-bit boundary (rare in practice)

#### GDExtension/C++ Bindings

**Native Types**:

```cpp
// Godot C++ (GDExtension) uses C++ native types
int32_t, int64_t   // Explicit bit widths
float, double      // 32-bit and 64-bit floats
```

**Binding Considerations**:

- GDExtension can expose C++ types to Godot
- Godot's `Variant` will store them as 64-bit internally
- **Narrowing conversions** (64 → 32) can lose data
- **Widening conversions** (32 → 64) are safe

**FerrisScript Implication**:

- FerrisScript's `i32` → Godot's `int` (i64): Safe widening ✅
- FerrisScript's `f32` → Godot's `float` (f64): Safe widening ✅
- **Current approach is compatible** with Godot expectations

---

### 2. Unity (C# / .NET)

**Type System**:

```csharp
// Unity C# numeric types
int     // i32 (32-bit signed)
long    // i64 (64-bit signed)
float   // f32 (32-bit float)
double  // f64 (64-bit double)
byte    // u8 (8-bit unsigned)
short   // i16 (16-bit signed)
```

**Behavior**:

- **No automatic promotion** by default
- Overflow: Wraps by default (can enable checked arithmetic)
- Explicit casting required: `(long)int32Value + int32Value`

**Checked Arithmetic** (Optional):

```csharp
checked {
    int result = big + big;  // Throws OverflowException if overflows
}
```

**Unity's Approach**:

- Uses 32-bit types extensively (`int`, `float`) for performance
- Explicit widening when needed (rare in game logic)
- Vector math: `Vector3` uses `float` (f32) for performance
- **Prioritizes performance over automatic promotion**

---

### 3. Unreal Engine (C++)

**Type System**:

```cpp
// Unreal Engine typedefs (UE5)
int32    // 32-bit signed (primary integer type)
int64    // 64-bit signed (rare, used for large values)
float    // 32-bit float (primary floating-point type)
double   // 64-bit double (rare, used for precision)
uint8    // 8-bit unsigned (common for small values, colors)
uint16   // 16-bit unsigned
```

**Behavior**:

- **No automatic promotion** (standard C++ rules)
- Overflow: Undefined behavior (wraps in practice)
- Explicit casting required for widening
- Uses `int32` and `float` extensively (32-bit preference)

**Unreal's Approach**:

- **Performance-first**: 32-bit types for most game logic
- 64-bit used sparingly (timestamps, large IDs)
- Smaller types (`uint8`) for memory optimization (textures, colors)
- **Explicit type control over convenience**

---

### 4. Bevy Engine (Rust)

**Type System**:

```rust
// Bevy (pure Rust) follows Rust conventions
i32, i64      // Explicit bit widths
f32, f64      // Explicit float precision
u8, u16, u32  // Unsigned variants
```

**Behavior**:

- **No automatic promotion** (Rust is explicit)
- Overflow: **Panics in debug**, wraps in release (default)
- Can opt into checked/saturating/wrapping arithmetic explicitly:

  ```rust
  let safe = big.checked_add(big).unwrap_or(i32::MAX);  // Saturates
  let wide = (big as i64) + (big as i64);                // Explicit cast
  ```

**Bevy's Approach**:

- Follows Rust philosophy: **Explicit over implicit**
- Uses `f32` for most game math (performance)
- Uses `i32` for most integer logic
- Developers explicitly widen when needed
- **Prioritizes predictability and performance**

---

### 5. Lua (Common Game Scripting Language)

**Type System**:

```lua
-- Lua has ONE numeric type: "number"
local x = 42       -- Integer (stored as 64-bit integer in Lua 5.3+)
local y = 3.14     -- Float (stored as 64-bit double)
local z = x + y    -- Automatic conversion to double
```

**Behavior**:

- **Automatic promotion**: Integer → Double when needed
- No overflow (uses 64-bit integers)
- Single "number" type simplifies scripting
- Performance cost: All numbers are 64-bit

**Lua's Approach**:

- **Convenience over performance** (scripting language)
- No type annotations (dynamic typing)
- Automatic promotion for ease of use
- **Not suitable for high-performance game core** (hence used for scripting layer)

---

### 6. JavaScript/TypeScript (Web Game Engines)

**Type System**:

```typescript
// JavaScript: All numbers are 64-bit floats (IEEE 754 double)
let x = 42;        // Stored as 64-bit double
let y = 2 ** 53;   // Max safe integer (2^53 - 1)

// TypeScript: Adds type annotations (no runtime effect)
let count: number = 42;  // Still 64-bit double at runtime
```

**Behavior**:

- **No integers** (everything is 64-bit double)
- Automatic promotion (everything is already "promoted")
- Performance: Slower than native integers
- WebAssembly: Can use explicit `i32`, `i64`, `f32`, `f64`

**JavaScript's Approach**:

- **Maximum convenience** (single number type)
- Performance cost for integer operations
- WebAssembly games use explicit types for performance

---

## 📊 Type Promotion Strategies (Comparative Analysis)

### Strategy 1: No Promotion (Explicit) - **Current FerrisScript**

**Examples**: Rust, C++, C#, Unreal, Unity

**Pros**:

- ✅ **Predictable performance**: No hidden conversions
- ✅ **Explicit control**: Developer chooses when to widen
- ✅ **Zero-cost abstraction**: Matches hardware behavior
- ✅ **Easier FFI**: Direct mapping to C ABI

**Cons**:

- ❌ **Overflow risk**: Developer must handle manually
- ❌ **Verbosity**: Explicit casts required
- ❌ **Learning curve**: Beginners may not anticipate overflow

**Example**:

```rust
// Explicit widening
let a: i32 = 2000000000;
let b: i32 = 2000000000;
let result: i64 = (a as i64) + (b as i64);  // Developer responsibility
```

---

### Strategy 2: Automatic Promotion (Dynamic) - **Lua, JavaScript**

**Examples**: Lua, JavaScript, GDScript (partially)

**Pros**:

- ✅ **Ease of use**: No manual overflow handling
- ✅ **Beginner-friendly**: Works "as expected"
- ✅ **No overflow bugs**: Values just get bigger

**Cons**:

- ❌ **Performance cost**: Larger types slower
- ❌ **Memory overhead**: Always using 64-bit
- ❌ **Unpredictable**: Hidden conversions
- ❌ **FFI complexity**: Type size varies at runtime

**Example**:

```lua
-- Lua: Automatic promotion
local big = 2000000000
local result = big + big  -- Works (uses i64 internally)
```

---

### Strategy 3: Hybrid Promotion (Compile-Time Analysis) - **Potential FerrisScript**

**Examples**: TypeScript (type widening), Kotlin (smart casts)

**Pros**:

- ✅ **Safety**: Compiler detects overflow risk
- ✅ **Performance**: Only widens when necessary
- ✅ **Explicit in types**: `i32 + i32 → i64` in signature
- ✅ **Best of both worlds**: Safety + performance

**Cons**:

- ⚠️ **Compiler complexity**: Overflow analysis is hard
- ⚠️ **Type inference complexity**: When to promote?
- ⚠️ **FFI considerations**: Return type may vary

**Example** (hypothetical FerrisScript):

```rust
// Hypothetical: Compiler promotes result type
let a: i32 = 2000000000;
let b: i32 = 2000000000;
let result = a + b;  // Compiler infers result: i64 (promotion)

// Or explicit promotion operator
let result: i64 = a +^ b;  // Operator signals promotion
```

---

### Strategy 4: Checked Arithmetic (Runtime Safety) - **Rust Option**

**Examples**: Rust (checked_add), C# (checked keyword), Swift

**Pros**:

- ✅ **Safety**: Detects overflow at runtime
- ✅ **Explicit**: Developer chooses checked vs unchecked
- ✅ **Performance when unchecked**: No cost for hot paths
- ✅ **Debugging-friendly**: Panics/errors on overflow

**Cons**:

- ⚠️ **Runtime cost**: Checks add instructions
- ⚠️ **Verbosity**: Explicit method calls
- ⚠️ **Error handling**: Must handle overflow cases

**Example**:

```rust
// Rust checked arithmetic
let result = a.checked_add(b).unwrap_or_else(|| {
    // Handle overflow: promote or error
    (a as i64) + (b as i64)
});
```

---

## 🔬 Static Typing Advantages for FerrisScript

### FerrisScript's Unique Position

As a **statically-typed language**, FerrisScript has advantages over dynamic scripting languages:

1. **Compile-Time Type Analysis**:
   - Can detect potential overflow at compile time
   - Can enforce explicit widening in signatures
   - Can warn when arithmetic might overflow

2. **Explicit Type Control**:
   - Developer declares types (not inferred at runtime)
   - Functions have explicit return types
   - No hidden runtime conversions

3. **Performance Optimization**:
   - Can use 32-bit types where safe
   - Can use smaller types (i16, u8) for memory savings
   - No runtime type checking overhead

4. **FFI Safety**:
   - Explicit types match C ABI
   - No runtime type size changes
   - Predictable memory layout

### Opportunity: Best of Both Worlds

FerrisScript could offer:

```rust
// Option 1: Explicit promotion (current)
let a: i32 = 2000000000;
let result: i64 = (a as i64) + (a as i64);

// Option 2: Checked arithmetic (future)
let result: i32 = a.checked_add(a)?;  // Error if overflow

// Option 3: Saturating arithmetic (future)
let result: i32 = a.saturating_add(a);  // Clamps to i32::MAX

// Option 4: Promotion in function signature (future)
fn add_promote(a: i32, b: i32) -> i64 {
    (a as i64) + (b as i64)
}
```

---

## 🎯 Recommended Approach for FerrisScript

### Phase 1: Core Type System (v0.1.0) - **Keep Current Approach**

**Recommendation**: **No automatic promotion** (Strategy 1)

**Rationale**:

- Matches Godot/Unity/Unreal conventions (explicit control)
- Predictable performance (important for game scripting)
- Simpler compiler implementation
- Easier FFI (types match C ABI)
- Aligns with Rust philosophy (explicit over implicit)

**Current Types**:

- `i32` - Primary integer type
- `f32` - Primary float type

**Overflow Behavior**:

- Debug: Panic on overflow (matches Rust debug)
- Release: Wrap on overflow (matches Rust release)

---

### Phase 2: Extended Type System (v0.2.0+) - **Add More Types**

**Recommendation**: Add explicit 64-bit and smaller types

**New Types**:

```rust
// 64-bit types (for large values)
i64  // 64-bit signed integer
f64  // 64-bit double-precision float

// Smaller types (for memory optimization)
i16  // 16-bit signed integer
u8   // 8-bit unsigned integer (0-255)
u16  // 16-bit unsigned integer
```

**Use Cases**:

- `i64`: Large entity IDs, timestamps, big calculations
- `f64`: High-precision physics, scientific calculations
- `i16`: Compact data structures (tile coordinates)
- `u8`: Colors (RGBA), small enums, flags
- `u16`: Medium-sized IDs, network packets

**Explicit Conversion**:

```rust
let small: i32 = 42;
let big: i64 = small as i64;    // Explicit widening
let precise: f64 = 3.14159 as f64;
```

---

### Phase 3: Checked Arithmetic (v0.3.0+) - **Add Safety Methods**

**Recommendation**: Add checked/saturating arithmetic methods

**API**:

```rust
// Checked (returns Option or error)
let result: i32? = a.checked_add(b);  // None if overflow

// Saturating (clamps to min/max)
let result: i32 = a.saturating_add(b);  // i32::MAX if overflow

// Wrapping (explicit wrap)
let result: i32 = a.wrapping_add(b);  // Wraps explicitly

// Overflowing (returns tuple)
let (result, overflowed) = a.overflowing_add(b);
```

**Rationale**:

- Gives developers explicit control over overflow
- No performance cost when not used
- Matches Rust's approach (proven in production)
- Clear intent in code

---

### Phase 4: Compiler Warnings (v0.4.0+) - **Static Analysis**

**Recommendation**: Add compile-time overflow detection

**Analysis**:

```rust
// Compiler warning: "Potential overflow in constant expression"
let result: i32 = 2000000000 + 2000000000;
//                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow detected

// Suggestion: Use i64 or checked arithmetic
let result: i64 = 2000000000_i64 + 2000000000_i64;
```

**Rationale**:

- Leverages static typing advantage
- No runtime cost
- Helps developers avoid bugs
- Educational (teaches about overflow)

---

### NOT Recommended: Automatic Promotion

**Why not automatic promotion?**

1. **Performance unpredictability**: Hidden conversions
2. **FFI complexity**: Return type varies
3. **Against explicit design**: FerrisScript is statically typed
4. **Godot compatibility**: Not needed (32→64 safe)
5. **Debugging difficulty**: Hidden type changes

**Exception**: Could consider **opt-in** promotion for specific operations:

```rust
// Hypothetical: Explicit promotion operator (if demand exists)
let result: i64 = a +! b;  // "+" with "!" means "promote if needed"
```

---

## 🎮 Godot Compatibility Analysis

### Current FerrisScript → Godot

| FerrisScript Type | Godot GDScript Type | Godot C++ Type | Conversion |
|-------------------|---------------------|----------------|------------|
| `i32` | `int` (i64) | `int64_t` | ✅ Safe widening |
| `f32` | `float` (f64) | `double` | ✅ Safe widening |

**Conclusion**: Current approach is **fully compatible** with Godot.

### With Extended Types

| FerrisScript Type | Godot GDScript Type | Godot C++ Type | Conversion |
|-------------------|---------------------|----------------|------------|
| `i32` | `int` (i64) | `int64_t` | ✅ Safe widening |
| `i64` | `int` (i64) | `int64_t` | ✅ Direct match |
| `f32` | `float` (f64) | `double` | ✅ Safe widening |
| `f64` | `float` (f64) | `double` | ✅ Direct match |
| `i16` | `int` (i64) | `int64_t` | ✅ Safe widening |
| `u8` | `int` (i64) | `int64_t` | ✅ Safe widening |

**Conclusion**: Extended types remain **fully compatible** with Godot.

### Binding Considerations

**No issues identified**:

- All FerrisScript types widen safely to Godot types
- No narrowing conversions (data loss)
- Performance: Godot uses 64-bit internally anyway
- Memory: Godot's `Variant` stores 64-bit regardless

---

## 📊 Performance Considerations

### 32-bit vs 64-bit Performance

**Modern CPUs (x86-64, ARM64)**:

- 64-bit arithmetic: **Same speed** as 32-bit (native register size)
- Memory bandwidth: 64-bit uses 2x memory (cache pressure)
- SIMD: Can process 4x f32 or 2x f64 per instruction

**Game Logic**:

- Most game math: **32-bit is sufficient** (positions, rotations, velocities)
- Rare cases need 64-bit: Timestamps, large world coordinates, entity IDs

**Recommendation**: **Use 32-bit by default**, widen when needed

### Smaller Types (i16, u8) Performance

**Benefits**:

- ✅ Memory savings: 2x (i16) or 4x (u8) smaller than i32
- ✅ Cache efficiency: More values fit in cache
- ✅ SIMD: Can process more values per instruction

**Drawbacks**:

- ⚠️ Overflow risk: Smaller range
- ⚠️ Conversion cost: May need casting
- ⚠️ Limited use cases: Most game values fit in i32

**Use Cases**:

- `u8`: Colors (RGBA), tile types, small enums
- `i16`: Tile coordinates (-32768 to 32767), audio samples
- **Not general-purpose**: Use for specific optimizations

---

## 🗓️ Roadmap Placement Recommendations

### Immediate (v0.0.3-v0.0.4): No Changes Needed ✅

**Current state is correct**:

- i32/f32 are industry-standard for game scripting
- Fully compatible with Godot
- Simple and predictable

---

### v0.1.0 (Godot Release Focus): No Type System Changes ✅

**Focus**: Tooling & Godot Integration (see `docs/planning/v0.1.0-ROADMAP.md`)

- LSP and editor support (HIGHEST PRIORITY)
- Enhanced Godot integration (signals, callbacks, types)
- Developer experience improvements

**Type System Status**:

- Current `i32`/`f32` types are **sufficient** for v0.1.0 goals
- No blocking issues for Godot compatibility
- Extended types **deferred** to post-v0.1.0 releases

**Rationale**:

- Focus on usability and adoption first
- Type system enhancements are non-blocking
- Avoid scope creep for critical release

---

### v0.2.0 (Proposed): Extended Type System

**Priority**: Medium  
**Effort**: 2-3 weeks  
**Dependencies**: Type checker, runtime
**Status**: Proposed (see `docs/planning/v0.2.0-roadmap.md`)

**Implementation**:

1. Add `i64`, `f64`, `i16`, `u8`, `u16` to lexer/parser
2. Update type checker for new types
3. Update runtime representation
4. Add explicit casting: `value as i64`
5. Update Godot bindings (safe widening)
6. Add tests for type conversions

**Rationale**:

- Requested by users for large values (timestamps, IDs)
- Enables memory optimizations (u8 for colors)
- Still explicit (no automatic promotion)
- Production-ready foundation

---

### v0.3.0 (Proposed): Checked Arithmetic

**Priority**: Low-Medium  
**Effort**: 1-2 weeks  
**Dependencies**: Extended type system
**Status**: Proposed (see `docs/planning/v0.3.0-roadmap.md`)

**Implementation**:

1. Add methods: `checked_add`, `saturating_add`, `wrapping_add`
2. Update runtime to support checked operations
3. Add error handling for overflow
4. Add tests for overflow scenarios

**Rationale**:

- Improves safety for critical calculations
- Opt-in (no performance cost when unused)
- Familiar to Rust developers
- Educational for beginners

---

### v0.4.0+ (Proposed): Compiler Warnings

**Priority**: Low  
**Effort**: 2-4 weeks  
**Dependencies**: Dataflow analysis, LSP integration

**Implementation**:

1. Add constant folding to detect literal overflows
2. Add range analysis for variable overflow detection
3. Emit warnings for potential overflows
4. Suggest fixes (use i64, checked arithmetic)
5. Integrate with LSP for in-editor warnings

**Rationale**:

- Leverages static typing advantage
- Helps developers avoid bugs
- No runtime cost
- Differentiator vs dynamic languages

---

### Future Exploration (v1.0+): Automatic Promotion (Not Recommended)

**Priority**: Very Low  
**Effort**: Unknown (complex)  
**Dependencies**: Full type inference, semantic analysis

**Considerations**:

- Only if strong user demand
- Opt-in (not default behavior)
- Explicit syntax (e.g., `+!` operator)
- Performance implications must be clear

**Rationale**:

- Not aligned with explicit design philosophy
- Adds compiler complexity
- May confuse with Godot's behavior
- **Not recommended unless proven necessary**

---

## 📚 Related Research

### Academic Papers

- **"Safe Integer Arithmetic" (LLVM Project)**: Overflow detection strategies
  - Paper: https://www.doc.ic.ac.uk/~phjk/Publications/IntegerOverflowPaper.pdf
  - LLVM Sanitizers: https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html#integer-overflow
  
- **"Dependent Types for Program Termination Verification" (Coq)**: Proving no overflow
  - Coq Documentation: https://coq.inria.fr/refman/language/core/inductive.html
  - Paper: https://dl.acm.org/doi/10.1145/3428265
  
- **"Refinement Types" (LiquidHaskell)**: Type-level range constraints
  - LiquidHaskell: https://ucsd-progsys.github.io/liquidhaskell/
  - Paper: https://goto.ucsd.edu/~rjhala/papers/liquid_types.pdf
  - Tutorial: https://ucsd-progsys.github.io/liquidhaskell-tutorial/

- **"Featherweight Java: A Minimal Core Calculus for Java and GJ"**: Type system foundations
  - Paper: https://www.cs.rice.edu/~javaplt/papers/oopsla99.pdf

- **"From System F to Typed Assembly Language"**: Type-preserving compilation
  - Paper: https://dl.acm.org/doi/10.1145/319301.319345

### Language References

- **Rust**: https://doc.rust-lang.org/std/primitive.i32.html#method.checked_add
- **Swift**: https://developer.apple.com/documentation/swift/int/2884663-addingwithoverflow
- **C#**: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/statements/checked-and-unchecked
- **Godot GDScript**: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#numbers
- **Rust Overflow Semantics**: https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow

### Engine Documentation

- **Godot Types**: https://docs.godotengine.org/en/stable/classes/class_variant.html
- **Godot GDExtension**: https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/index.html
- **Unity C# Types**: https://docs.unity3d.com/Manual/CSharpCompiler.html
- **Unreal Types**: https://docs.unrealengine.com/5.0/en-US/integer-types-in-unreal-engine/
- **Bevy Engine**: https://bevyengine.org/ (Rust game engine)

---

## ✅ Summary & Recommendations

### Current Approach (v0.0.3): ✅ **Correct**

- Keep `i32` and `f32` as primary types
- No automatic promotion (explicit control)
- Matches industry standards (Unity, Unreal, Godot-compatible)

### Next Steps (v0.1.0): **Extended Type System**

- Add `i64`, `f64`, `i16`, `u8`, `u16`
- Keep explicit casting (`as i64`)
- Maintain performance and predictability

### Future (v0.2.0+): **Safety Features**

- Add checked arithmetic methods
- Add compiler overflow warnings
- Consider opt-in promotion (low priority)

### Key Principle: "Explicit over implicit, but safe by default"

- Static typing enables better compiler analysis
- Explicit casts give developer control
- Checked arithmetic provides safety when needed
- No hidden conversions or performance surprises

---

## 📝 Open Questions

1. **Should we add unsigned types** (`u32`, `u64`)?
   - Godot doesn't use them extensively
   - Useful for bit manipulation, IDs
   - **Recommendation**: Add in v0.2.0 if demand exists

2. **Should overflow panic in release builds**?
   - Rust defaults to wrapping (performance)
   - Games may prefer deterministic behavior
   - **Recommendation**: Configurable via compiler flag

3. **Should we support type inference for promotions**?
   - `let x = big + big` → infer `i64`?
   - Adds complexity to type inference
   - **Recommendation**: Require explicit type annotation

4. **Should we add overflow operators** (`+!`, `*!`)?
   - Explicit promotion syntax
   - Avoids verbosity of `as i64`
   - **Recommendation**: Consider in v0.3.0+ if requested

---

## Document End

This research document provides the foundation for FerrisScript's type system evolution and informed the roadmap decisions for v0.2.0 and v0.3.0.
