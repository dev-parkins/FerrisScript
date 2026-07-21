# Performance Benchmarks Baseline

This document records the baseline performance metrics for FerrisScript v0.0.1, established using [criterion.rs](https://github.com/bheisler/criterion.rs).

**Date**: October 2, 2025  
**Platform**: Windows (test environment)  
**Build**: `cargo bench` (release mode, optimizations enabled)  
**Version**: v0.0.1

---

## Compiler Benchmarks

### Lexer Performance

Tokenization speed for different input sizes:

| Input Size | Time (avg) | Throughput |
|------------|------------|------------|
| **Small** (single line) | **384 ns** | ~2.6M ops/sec |
| **Medium** (function def) | **1.50 ┬╡s** | ~667K ops/sec |
| **Large** (bounce example) | **3.74 ┬╡s** | ~267K ops/sec |

**Analysis**:

- Lexer performance scales linearly with input size
- Small inputs (~20 chars) tokenize in under 400 nanoseconds
- Medium inputs (~150 chars) tokenize in ~1.5 microseconds
- Large inputs (~400 chars) tokenize in ~3.7 microseconds
- Performance is excellent for typical game scripting workloads

### Parser Performance

Parsing speed (includes tokenization):

| Input Size | Time (avg) | Throughput |
|------------|------------|------------|
| **Small** | **600 ns** | ~1.67M ops/sec |
| **Medium** | **3.10 ┬╡s** | ~323K ops/sec |
| **Large** | **7.94 ┬╡s** | ~126K ops/sec |

**Analysis**:

- Parser adds ~200-220 ns overhead over lexing for small inputs
- Medium and large inputs show roughly 2x parsing overhead
- Parser maintains good performance even on complex nested structures

### Type Checker Performance

Type checking speed (includes tokenization + parsing):

| Input Size | Time (avg) | Throughput |
|------------|------------|------------|
| **Small** | **851 ns** | ~1.17M ops/sec |
| **Medium** | **3.58 ┬╡s** | ~279K ops/sec |

**Analysis**:

- Type checker adds ~250 ns overhead for small inputs
- Medium inputs: ~480 ns type checking overhead
- Type checking is efficient and doesn't significantly slow compilation

### Full Pipeline Performance

Complete compilation (lex + parse + type check):

| Benchmark | Time (avg) |
|-----------|------------|
| **Medium function** | **3.60 ┬╡s** |

**Analysis**:

- End-to-end compilation of a medium function: ~3.6 microseconds
- This means FerrisScript can compile ~278K functions per second
- Excellent for game scripting where functions are compiled once

---

## Runtime Benchmarks

### Compilation (AST Generation)

| Input | Time (avg) | Throughput |
|-------|------------|------------|
| **Simple function** | **2.14 ┬╡s** | ~468K ops/sec |
| **Complex function** (with recursion) | **3.08 ┬╡s** | ~325K ops/sec |

### Execution Performance

#### Basic Operations

| Operation | Time (avg) | Throughput |
|-----------|------------|------------|
| **Simple arithmetic** (x + y) | **1.05 ┬╡s** | ~954K ops/sec |
| **Control flow** (if/else) | **1.31 ┬╡s** | ~763K ops/sec |

**Analysis**:

- Function call with simple arithmetic: ~1 microsecond
- Control flow adds ~250 ns overhead
- Very fast for typical game logic operations

#### Loop Performance

| Iterations | Time (avg) | Per-iteration |
|------------|------------|---------------|
| **10 iterations** | **3.54 ┬╡s** | ~354 ns/iter |
| **100 iterations** | **17.76 ┬╡s** | ~178 ns/iter |

**Analysis**:

- Loop overhead: ~3.5 ┬╡s for 10 iterations (includes setup)
- Steady-state performance: ~180 ns per iteration
- Good scaling for typical game loop operations

#### Recursion Performance

| Depth | Time (avg) | Per-call |
|-------|------------|----------|
| **5 levels** | **5.03 ┬╡s** | ~1 ┬╡s/call |
| **10 levels** | **9.50 ┬╡s** | ~950 ns/call |
| **20 levels** | **18.49 ┬╡s** | ~925 ns/call |

**Analysis**:

- Recursive function calls: ~1 microsecond per call
- Good scaling - performance remains consistent at deeper depths
- Suitable for algorithms requiring recursion (e.g., tree traversal)

### Variable Operations

| Operation | Time (avg) |
|-----------|------------|
| **Local variables** (3 vars + addition) | **1.53 ┬╡s** |
| **Mutable updates** (4 updates) | **1.49 ┬╡s** |

**Analysis**:

- Variable declaration and access: ~510 ns per variable
- Mutable variable updates: ~370 ns per update
- Very efficient variable handling

### Type Operations

| Type | Time (avg) | Operations |
|------|------------|------------|
| **Integer arithmetic** | **3.41 ┬╡s** | +, -, *, / (4 ops) |
| **Float arithmetic** | **3.32 ┬╡s** | +, -, *, / (4 ops) |
| **Boolean logic** | **1.81 ┬╡s** | >, !=, &&, \|\| |

**Analysis**:

- Integer operations: ~850 ns per operation
- Float operations: ~830 ns per operation (slightly faster!)
- Boolean operations: ~450 ns per operation
- All type operations are fast and consistent

---

## Performance Summary

### Key Findings

1. **Compilation Speed**:

   - Small scripts compile in under 1 microsecond
   - Medium scripts (~150 chars) compile in ~3.6 microseconds
   - Suitable for hot-reload scenarios in game development

2. **Execution Speed**:

   - Simple function calls: ~1 microsecond
   - Loop iterations: ~180 nanoseconds per iteration
   - Recursive calls: ~925 nanoseconds per call

3. **Scaling**:

   - Lexer, parser, and type checker all scale linearly
   - Runtime performance remains consistent across different depths
   - No performance cliffs observed

4. **Comparison to Native**:

   - Function call overhead is reasonable (~1 μs vs ~10 ns for native Rust)
   - Good enough for game scripting (1000s of calls per frame are feasible)
   - Arithmetic operations add ~850 ns vs <1 ns native (acceptable for scripting)

### Bottleneck Analysis

Potential optimization targets (in order of impact):

1. **Function call overhead** (~1 μs per call)
   - Most significant overhead in the runtime
   - Could investigate call site optimization or JIT compilation

2. **Loop overhead** (~3.5 μs setup cost)
   - While per-iteration cost is good, setup could be optimized
   - Consider loop unrolling or specialized bytecode

3. **Variable operations** (~370-510 ns per operation)
   - HashMap lookups could be optimized
   - Consider arena allocation or flat variable storage

### Godot Integration Considerations

For Godot 4 integration:

- **60 FPS** = 16.67 ms per frame
- At current performance:
  - Can execute ~16,000 simple function calls per frame
  - Can run ~93,000 loop iterations per frame
  - Can perform ~20,000 arithmetic operations per frame

**Verdict**: Current performance is more than adequate for typical game scripting workloads in Godot 4.

---

## Benchmark Reproducibility

To reproduce these benchmarks:

```bash
# Run compiler benchmarks
cargo bench --package ferrisscript_compiler

# Run runtime benchmarks
cargo bench --package ferrisscript_runtime

# Run all benchmarks
cargo bench --workspace
```

Benchmarks generate HTML reports in `target/criterion/` for detailed analysis.

---

## Future Optimization Targets

### Short-term (v0.1.0)

- [ ] Optimize HashMap variable lookups with faster alternatives
- [ ] Implement instruction caching for frequently called functions
- [ ] Add benchmark for Godot property access (when integration is ready)

### Medium-term (v0.2.0)

- [ ] Investigate bytecode compilation instead of AST interpretation
- [ ] Add SIMD optimizations for Vector2/Vector3 operations
- [ ] Profile and optimize hot paths identified in real game scenarios

### Long-term (v1.0.0)

- [ ] Consider JIT compilation for frequently executed functions
- [ ] Evaluate LLVM backend for native code generation
- [ ] Implement call site caching and inline caching techniques

---

**Note**: These benchmarks represent the baseline performance of v0.0.1. Future optimizations should be measured against these metrics to track progress.
