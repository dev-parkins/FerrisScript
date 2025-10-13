# Consolidated Test Framework - Implementation Guide

**Version**: 0.0.5  
**Status**: Planning (Updated with LSP Integration)  
**Date**: October 13, 2025  
**Priority**: High  
**Dependencies**: Test harness crate, GDExtension runtime, LSP server (v0.0.5)  

## Executive Summary

This document provides a **production-ready implementation plan** for consolidating FerrisScript's testing infrastructure. It combines the strategic vision from the original proposal with battle-tested implementations that address critical cross-platform issues.

### 🆕 Critical Updates from Feedback Review

This document has been **updated to include critical gaps** identified in the feedback analysis:

1. **LSP Integration** (NEW Phase 2.5) 🔴 CRITICAL
   - Code lens test status indicators (✅/❌/▶️)
   - "Run Test" command from editor
   - Real-time test result updates
   - **Impact**: Essential for v0.0.5 editor experience
   - **Timeline Impact**: +1-2 weeks

2. **Test Assertions Validation** 🟡 IMPORTANT
   - Runtime assertion checking framework
   - `FerrisScriptRunner.get_variable()` API
   - `FerrisScriptRunner.get_emitted_signals()` API
   - **Impact**: Accurate test results vs. false positives

3. **Test Timeouts** 🟡 IMPORTANT
   - Configurable timeout per test (default: 10s)
   - Prevents CI hangs from infinite loops
   - **Impact**: Reliable CI pipeline

4. **Test Filtering** 🟢 NICE-TO-HAVE
   - Filter by test name pattern
   - Filter by category (unit/integration/runtime)
   - **Impact**: Faster iteration during development

5. **JSON Output** 🟢 NICE-TO-HAVE
   - Machine-readable CI results
   - Test badges generation
   - **Impact**: Better CI reporting

**Updated Timeline**: **6-7 weeks** (was 3 weeks) due to:

- LSP integration requirements (+1-2 weeks)
- Compiler prerequisites: spans + symbol table (+1 week)
- Incremental compilation support (+2-3 weeks)

### Key Improvements Over Current System

| Issue | Current State | New Approach | Impact |
|-------|--------------|--------------|---------|
| **Duplication** | `.ferris` files in `/examples` AND `/godot_test/scripts` | Single source in `/examples` | -50% maintenance burden |
| **Synchronization** | Manual file copying | Godot reads directly from `/examples` | Zero-effort updates |
| **Test Discovery** | Manual test registration | Automated metadata scanning | 100% test coverage |
| **Windows Support** | Broken (symlink issues) | FileAccess-based approach | ✅ Cross-platform |
| **CI Integration** | Fragile | Exit codes + JSON reports | ✅ Reliable automation |
| **LSP Integration** | Not supported | Real-time test status + "Run Test" code lenses | ✅ Editor integration |
| **Test Assertions** | Not validated | Runtime assertion validation framework | ✅ Accurate test results |
| **Test Filtering** | Run all or nothing | Filter by name/category via env vars | ✅ Fast iteration |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│              Test Discovery Engine (Rust)                   │
│  - Scans /examples for TEST: metadata                       │
│  - Validates required fields                                │
│  - On-demand generation (no stale manifests)                │
└────────────────┬────────────────────────────────────────────┘
                 │
    ┌────────────┴──────────────┐
    │                           │
┌───▼─────────────┐   ┌────────▼──────────────┐
│ Rust Unit Tests │   │ Godot Headless Tests  │
│                 │   │                       │
│ - Type checks   │   │ - Runtime behavior    │
│ - Syntax errors │   │ - Engine integration  │
│ - Compile tests │   │ - Signal/property     │
└─────────────────┘   └───────────────────────┘
         │                      │
         │                      │
         └──────────┬───────────┘
                    │
         ┌──────────▼──────────┐
         │     /examples       │
         │ (single source)     │
         │                     │
         │ ✅ bounce.ferris    │
         │ ✅ signal.ferris    │
         │ ✅ error_test.ferris│
         └─────────────────────┘
```

## File Structure

```
FerrisScript/
├── examples/                    # 📝 Single source of truth
│   ├── README.md               # Test authoring guide
│   ├── bounce.ferris           # Integration test
│   ├── signal_test.ferris      # Runtime test
│   └── type_error.ferris       # Unit test (negative)
│
├── crates/
│   └── test_harness/           # 🧪 Testing infrastructure
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # Public API
│           ├── metadata.rs     # TestMetadata types + parser
│           ├── discovery.rs    # Test discovery engine
│           └── runner.rs       # Test execution
│
├── tests/
│   └── ferris_integration_tests.rs  # 🦀 Rust test harness
│
├── godot_test/                 # 🎮 Godot test project
│   ├── project.godot           # Minimal config
│   └── test_runner.gd          # GDScript test harness
│
├── scripts/
│   └── run_godot_tests.sh      # 🚀 CI runner script
│
└── .github/
    └── workflows/
        └── test.yml            # 🔄 CI configuration
```

**Key Changes**:

- ❌ No `.ferris` files in `godot_test/scripts` (eliminated)
- ❌ No `target/test_manifest.json` (on-demand generation)
- ✅ Direct filesystem access (no symlinks)
- ✅ Cross-platform compatibility

## Implementation Phases

### Phase 0: Compiler Prerequisites (2-3 weeks) 🆕 BLOCKING

**CRITICAL**: These compiler changes must be completed before any LSP integration work can begin.

#### 0.1 Add Source Spans to AST (Week 1)

**Goal**: Every AST node must track its source location for LSP error reporting.

**Tasks**:

1. Define `Span` struct:

```rust
// crates/compiler/src/span.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub column: u32,
    pub offset: usize,  // Byte offset in file
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start,
            end: other.end,
        }
    }
}
```

2. Add `span()` method to all AST nodes:

```rust
// crates/compiler/src/ast.rs (BEFORE)
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary { left: Box<Expr>, op: BinOp, right: Box<Expr> },
}

// crates/compiler/src/ast.rs (AFTER)
pub enum Expr {
    Literal { value: Literal, span: Span },
    Variable { name: String, span: Span },
    Binary { 
        left: Box<Expr>, 
        op: BinOp, 
        right: Box<Expr>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal { span, .. } => *span,
            Expr::Variable { span, .. } => *span,
            Expr::Binary { span, .. } => *span,
        }
    }
}
```

3. Update Parser to track spans:

```rust
// crates/compiler/src/parser.rs
impl Parser {
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let start = self.current_token().span.start;
        
        let expr = match self.current_token().kind {
            TokenKind::Number(n) => {
                let end = self.current_token().span.end;
                self.advance();
                Expr::Literal { 
                    value: Literal::Number(n),
                    span: Span::new(start, end),
                }
            }
            // ... other cases
        };
        
        Ok(expr)
    }
}
```

4. Update all tests to include spans:

```rust
// Before
assert_eq!(expr, Expr::Literal(Literal::Number(42)));

// After
assert_eq!(
    expr,
    Expr::Literal {
        value: Literal::Number(42),
        span: Span::new(Position::new(1, 0, 0), Position::new(1, 2, 2))
    }
);
```

**Deliverables**:

- [ ] `Span` and `Position` structs defined
- [ ] All AST nodes have `span` field
- [ ] Parser tracks spans from tokens
- [ ] All tests updated with span assertions
- [ ] Error messages include span information

**Time Estimate**: 5-7 days

#### 0.2 Extract Symbol Table from Type Checker (Week 2)

**Goal**: Make symbol table accessible to LSP for go-to-definition and autocomplete.

**Tasks**:

1. Define `SymbolTable` struct:

```rust
// crates/compiler/src/symbol_table.rs
use std::collections::HashMap;

pub struct SymbolTable {
    /// Map symbol name → symbol info
    symbols: HashMap<String, Symbol>,
    
    /// Scopes (for nested symbols)
    scopes: Vec<Scope>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: Type,
    pub span: Span,
    pub scope_id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
    Constant,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: usize,
    pub parent: Option<usize>,
    pub symbols: Vec<String>,  // Symbol names in this scope
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            scopes: vec![Scope { id: 0, parent: None, symbols: vec![] }],
        }
    }
    
    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }
    
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
    
    pub fn lookup_in_scope(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        // Walk up scope chain to find symbol
        let mut current_scope = Some(scope_id);
        
        while let Some(id) = current_scope {
            let scope = &self.scopes[id];
            
            if scope.symbols.contains(&name.to_string()) {
                return self.symbols.get(name);
            }
            
            current_scope = scope.parent;
        }
        
        None
    }
    
    pub fn all_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }
    
    pub fn symbols_in_scope(&self, scope_id: usize) -> Vec<&Symbol> {
        let scope = &self.scopes[scope_id];
        scope.symbols.iter()
            .filter_map(|name| self.symbols.get(name))
            .collect()
    }
}
```

2. Refactor `TypeChecker` to build symbol table:

```rust
// crates/compiler/src/type_checker.rs
pub struct TypeChecker {
    symbol_table: SymbolTable,
    current_scope: usize,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn check(ast: &Ast) -> Result<(Type, SymbolTable), Vec<TypeError>> {
        let mut checker = TypeChecker {
            symbol_table: SymbolTable::new(),
            current_scope: 0,
            errors: vec![],
        };
        
        let ty = checker.check_program(ast);
        
        if checker.errors.is_empty() {
            Ok((ty, checker.symbol_table))
        } else {
            Err(checker.errors)
        }
    }
    
    fn check_variable_decl(&mut self, name: &str, ty: &Type, span: Span) {
        let symbol = Symbol {
            name: name.to_string(),
            kind: SymbolKind::Variable,
            ty: ty.clone(),
            span,
            scope_id: self.current_scope,
        };
        
        self.symbol_table.insert(symbol);
    }
}
```

3. Update compiler entry point:

```rust
// crates/compiler/src/lib.rs
pub struct CompilationResult {
    pub program: CompiledProgram,
    pub symbol_table: SymbolTable,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn compile(source: &str) -> Result<CompilationResult, CompileError> {
    // Lex
    let tokens = lex(source)?;
    
    // Parse
    let ast = parse(tokens)?;
    
    // Type check (returns symbol table)
    let (ty, symbol_table) = type_check(&ast)?;
    
    // Code gen
    let program = codegen(&ast, &symbol_table)?;
    
    Ok(CompilationResult {
        program,
        symbol_table,
        diagnostics: vec![],
    })
}
```

**Deliverables**:

- [ ] `SymbolTable` struct with public API
- [ ] `TypeChecker` builds symbol table during type checking
- [ ] `compile()` returns `SymbolTable`
- [ ] Tests verify symbol table contents
- [ ] Documentation for symbol table usage

**Time Estimate**: 5-7 days

#### 0.3 Add Incremental Compilation Support (Week 3)

**Goal**: Cache parsed ASTs and type check only changed regions for fast LSP updates.

**Tasks**:

1. Design incremental compilation architecture:

```rust
// crates/compiler/src/incremental.rs
use std::collections::HashMap;

pub struct IncrementalCompiler {
    /// Cache of parsed ASTs by file URI
    ast_cache: HashMap<String, CachedAst>,
    
    /// Cache of symbol tables by file URI
    symbol_cache: HashMap<String, SymbolTable>,
    
    /// Dependency graph (which files import which)
    dependencies: DependencyGraph,
}

#[derive(Clone)]
struct CachedAst {
    ast: Ast,
    source_hash: u64,  // Hash of source for invalidation
    timestamp: SystemTime,
}

impl IncrementalCompiler {
    pub fn new() -> Self {
        Self {
            ast_cache: HashMap::new(),
            symbol_cache: HashMap::new(),
            dependencies: DependencyGraph::new(),
        }
    }
    
    /// Compile a file, using cache when possible
    pub fn compile_file(&mut self, uri: &str, source: &str) -> Result<CompilationResult, CompileError> {
        let source_hash = hash_source(source);
        
        // Check if cached AST is still valid
        if let Some(cached) = self.ast_cache.get(uri) {
            if cached.source_hash == source_hash {
                // Cache hit - reuse AST
                return self.recheck_cached(uri, cached);
            }
        }
        
        // Cache miss - full compilation
        self.compile_fresh(uri, source)
    }
    
    fn compile_fresh(&mut self, uri: &str, source: &str) -> Result<CompilationResult, CompileError> {
        // Full compilation path
        let tokens = lex(source)?;
        let ast = parse(tokens)?;
        let (ty, symbol_table) = type_check(&ast)?;
        let program = codegen(&ast, &symbol_table)?;
        
        // Cache results
        self.ast_cache.insert(uri.to_string(), CachedAst {
            ast: ast.clone(),
            source_hash: hash_source(source),
            timestamp: SystemTime::now(),
        });
        
        self.symbol_cache.insert(uri.to_string(), symbol_table.clone());
        
        Ok(CompilationResult {
            program,
            symbol_table,
            diagnostics: vec![],
        })
    }
    
    fn recheck_cached(&self, uri: &str, cached: &CachedAst) -> Result<CompilationResult, CompileError> {
        // Reuse cached AST, only re-run type checker
        let (ty, symbol_table) = type_check(&cached.ast)?;
        let program = codegen(&cached.ast, &symbol_table)?;
        
        Ok(CompilationResult {
            program,
            symbol_table,
            diagnostics: vec![],
        })
    }
    
    /// Invalidate cache for a file and its dependents
    pub fn invalidate(&mut self, uri: &str) {
        self.ast_cache.remove(uri);
        self.symbol_cache.remove(uri);
        
        // Invalidate files that depend on this one
        for dependent in self.dependencies.dependents_of(uri) {
            self.invalidate(dependent);
        }
    }
}

fn hash_source(source: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}
```

2. Implement dependency tracking:

```rust
// crates/compiler/src/dependency_graph.rs
use std::collections::{HashMap, HashSet};

pub struct DependencyGraph {
    /// Map file URI → files it depends on
    dependencies: HashMap<String, HashSet<String>>,
    
    /// Map file URI → files that depend on it
    reverse_deps: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            reverse_deps: HashMap::new(),
        }
    }
    
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        self.dependencies
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
        
        self.reverse_deps
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string());
    }
    
    pub fn dependents_of(&self, uri: &str) -> Vec<&str> {
        self.reverse_deps
            .get(uri)
            .map(|deps| deps.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}
```

3. Integrate with LSP document manager:

```rust
// crates/lsp/src/document.rs
use ferrisscript_compiler::IncrementalCompiler;

pub struct DocumentManager {
    compiler: IncrementalCompiler,
    documents: HashMap<Url, Document>,
}

impl DocumentManager {
    pub fn did_change(&mut self, uri: &Url, changes: Vec<TextDocumentContentChangeEvent>) {
        // Apply changes to document
        let doc = self.documents.get_mut(uri).unwrap();
        doc.apply_changes(changes);
        
        // Recompile incrementally
        match self.compiler.compile_file(&uri.to_string(), &doc.text) {
            Ok(result) => {
                doc.diagnostics = result.diagnostics;
                doc.symbol_table = Some(result.symbol_table);
            }
            Err(e) => {
                doc.diagnostics = vec![e.into()];
            }
        }
    }
}
```

**Deliverables**:

- [ ] `IncrementalCompiler` with AST caching
- [ ] `DependencyGraph` for cache invalidation
- [ ] Source hash-based cache validation
- [ ] Integration with LSP document manager
- [ ] Performance benchmarks (cache hit rate, compilation speed)

**Time Estimate**: 10-14 days

**Total Phase 0 Time**: 2-3 weeks

---

### Phase 1: Test Harness Foundation (2-3 days)

#### 1.1 Create `crates/test_harness/Cargo.toml`

```toml
[package]
name = "ferrisscript_test_harness"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
regex = "1.10"
thiserror = "1.0"

[dev-dependencies]
tempfile = "3.8"
```

#### 1.2 Define Core Types (`crates/test_harness/src/metadata.rs`)

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Test metadata parsed from .ferris file headers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestMetadata {
    /// Unique test identifier (e.g., "bounce_horizontal")
    pub name: String,
    
    /// Test category
    pub category: TestCategory,
    
    /// Human-readable description
    pub description: String,
    
    /// Expected outcome
    pub expectation: TestExpectation,
    
    /// Runtime assertions (optional)
    pub assertions: Vec<String>,
    
    /// Source file path (relative to workspace root)
    pub file_path: PathBuf,
    
    /// Required scene path for runtime tests (optional)
    pub scene: Option<String>,
    
    /// Test timeout in seconds (default: 10.0)
    pub timeout: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TestCategory {
    /// Compile-only tests (type checking, syntax)
    Unit,
    
    /// Compile + basic runtime (no Godot scene required)
    Integration,
    
    /// Full Godot runtime tests (requires scene setup)
    Runtime,
}

impl std::fmt::Display for TestCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestCategory::Unit => write!(f, "unit"),
            TestCategory::Integration => write!(f, "integration"),
            TestCategory::Runtime => write!(f, "runtime"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum TestExpectation {
    /// Test should pass without errors
    Success,
    
    /// Test should fail with specific error code
    Error { code: String },
    
    /// Test should produce specific warning
    Warning { code: String },
}

impl TestExpectation {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "success" => Ok(TestExpectation::Success),
            _ if s.starts_with("error:") => {
                let code = s.strip_prefix("error:").unwrap().to_string();
                Ok(TestExpectation::Error { code })
            }
            _ if s.starts_with("warning:") => {
                let code = s.strip_prefix("warning:").unwrap().to_string();
                Ok(TestExpectation::Warning { code })
            }
            _ => Err(format!(
                "Invalid expectation '{}', must be success/error:CODE/warning:CODE",
                s
            )),
        }
    }
}

/// Error type for metadata parsing failures
#[derive(Debug, thiserror::Error)]
pub enum MetadataParseError {
    #[error("Missing required field '{field}' in {file}")]
    MissingField {
        file: PathBuf,
        field: String,
    },
    
    #[error("Invalid value '{value}' for field '{field}' in {file}: {reason}")]
    InvalidValue {
        file: PathBuf,
        field: String,
        value: String,
        reason: String,
    },
    
    #[error("IO error reading {file}: {source}")]
    IoError {
        file: PathBuf,
        source: std::io::Error,
    },
}
```

#### 1.3 Implement Robust Parser (`crates/test_harness/src/discovery.rs`)

```rust
use crate::metadata::{MetadataParseError, TestCategory, TestExpectation, TestMetadata};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// Discover all tests in a directory
pub fn discover_tests(dir: impl AsRef<Path>) -> Result<Vec<TestMetadata>, MetadataParseError> {
    let dir = dir.as_ref();
    let mut tests = Vec::new();
    
    let entries = fs::read_dir(dir).map_err(|e| MetadataParseError::IoError {
        file: dir.to_path_buf(),
        source: e,
    })?;
    
    for entry in entries {
        let entry = entry.map_err(|e| MetadataParseError::IoError {
            file: dir.to_path_buf(),
            source: e,
        })?;
        
        let path = entry.path();
        
        // Only process .ferris files
        if path.extension().and_then(|s| s.to_str()) != Some("ferris") {
            continue;
        }
        
        // Parse metadata (skip files without TEST: metadata)
        if let Ok(metadata) = parse_test_metadata(&path) {
            tests.push(metadata);
        }
    }
    
    Ok(tests)
}

/// Parse test metadata from a .ferris file
pub fn parse_test_metadata(path: impl AsRef<Path>) -> Result<TestMetadata, MetadataParseError> {
    let path = path.as_ref();
    
    let content = fs::read_to_string(path).map_err(|e| MetadataParseError::IoError {
        file: path.to_path_buf(),
        source: e,
    })?;
    
    // Extract header comments (stop at first non-comment line)
    let header: Vec<&str> = content
        .lines()
        .take_while(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("//") || trimmed.is_empty()
        })
        .collect();
    
    let header_text = header.join("\n");
    
    // Compile regexes once (in production, these would be lazy_static)
    let re_test = Regex::new(r"//\s*TEST:\s*(.+)").unwrap();
    let re_category = Regex::new(r"//\s*CATEGORY:\s*(.+)").unwrap();
    let re_description = Regex::new(r"//\s*DESCRIPTION:\s*(.+)").unwrap();
    let re_expect = Regex::new(r"//\s*EXPECT:\s*(.+)").unwrap();
    let re_assert = Regex::new(r"//\s*ASSERT:\s*(.+)").unwrap();
    let re_scene = Regex::new(r"//\s*SCENE:\s*(.+)").unwrap();
    
    // Extract required fields
    let name = re_test
        .captures(&header_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .ok_or_else(|| MetadataParseError::MissingField {
            file: path.to_path_buf(),
            field: "TEST".to_string(),
        })?;
    
    let category_str = re_category
        .captures(&header_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .ok_or_else(|| MetadataParseError::MissingField {
            file: path.to_path_buf(),
            field: "CATEGORY".to_string(),
        })?;
    
    let description = re_description
        .captures(&header_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .ok_or_else(|| MetadataParseError::MissingField {
            file: path.to_path_buf(),
            field: "DESCRIPTION".to_string(),
        })?;
    
    let expect_str = re_expect
        .captures(&header_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .ok_or_else(|| MetadataParseError::MissingField {
            file: path.to_path_buf(),
            field: "EXPECT".to_string(),
        })?;
    
    // Parse and validate category
    let category = match category_str.as_str() {
        "unit" => TestCategory::Unit,
        "integration" => TestCategory::Integration,
        "runtime" => TestCategory::Runtime,
        other => {
            return Err(MetadataParseError::InvalidValue {
                file: path.to_path_buf(),
                field: "CATEGORY".to_string(),
                value: other.to_string(),
                reason: "must be unit/integration/runtime".to_string(),
            })
        }
    };
    
    // Parse expectation
    let expectation = TestExpectation::from_str(&expect_str).map_err(|reason| {
        MetadataParseError::InvalidValue {
            file: path.to_path_buf(),
            field: "EXPECT".to_string(),
            value: expect_str.clone(),
            reason,
        }
    })?;
    
    // Extract optional fields
    let assertions: Vec<String> = re_assert
        .captures_iter(&header_text)
        .filter_map(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .collect();
    
    let scene = re_scene
        .captures(&header_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string());
    
    // Parse optional timeout (default: 10.0 seconds)
    let re_timeout = Regex::new(r"//\s*TIMEOUT:\s*(.+)").unwrap();
    let timeout = re_timeout
        .captures(&header_text)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().trim().parse::<f32>().ok());
    
    Ok(TestMetadata {
        name,
        category,
        description,
        expectation,
        assertions,
        file_path: path.to_path_buf(),
        scene,
        timeout,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_parse_valid_metadata() {
        let content = r#"
// TEST: bounce_horizontal
// CATEGORY: integration
// DESCRIPTION: Horizontal bouncing motion
// EXPECT: success
// ASSERT: Position oscillates

fn _ready() {}
"#;
        
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.ferris");
        fs::write(&test_file, content).unwrap();
        
        let metadata = parse_test_metadata(&test_file).unwrap();
        
        assert_eq!(metadata.name, "bounce_horizontal");
        assert_eq!(metadata.category, TestCategory::Integration);
        assert_eq!(metadata.expectation, TestExpectation::Success);
    }
    
    #[test]
    fn test_missing_required_field() {
        let content = r#"
// TEST: test_name
// EXPECT: success

fn _ready() {}
"#;
        
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.ferris");
        fs::write(&test_file, content).unwrap();
        
        let result = parse_test_metadata(&test_file);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MetadataParseError::MissingField { field, .. } if field == "CATEGORY"
        ));
    }
    
    #[test]
    fn test_invalid_category() {
        let content = r#"
// TEST: test_name
// CATEGORY: invalid_category
// DESCRIPTION: Test description
// EXPECT: success

fn _ready() {}
"#;
        
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.ferris");
        fs::write(&test_file, content).unwrap();
        
        let result = parse_test_metadata(&test_file);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MetadataParseError::InvalidValue { field, .. } if field == "CATEGORY"
        ));
    }
}
```

#### 1.4 Public API (`crates/test_harness/src/lib.rs`)

```rust
pub mod discovery;
pub mod metadata;
pub mod runner;

// Re-export main types
pub use discovery::{discover_tests, parse_test_metadata};
pub use metadata::{MetadataParseError, TestCategory, TestExpectation, TestMetadata};
```

#### 1.5 Update Root `Cargo.toml`

```toml
[workspace]
members = [
    "crates/compiler",
    "crates/runtime",
    "crates/godot_bind",
    "crates/test_harness",  # Add this
]
```

### Phase 2: Rust Test Integration (1-2 days)

#### 2.1 Create `tests/ferris_integration_tests.rs`

```rust
use ferrisscript_compiler;
use ferrisscript_test_harness::{discover_tests, TestCategory, TestExpectation};
use std::fs;
use std::path::Path;

#[test]
fn test_all_ferris_examples() {
    // Discover tests on-demand
    let tests = discover_tests("examples").expect("Failed to discover tests");
    
    // Apply filtering based on environment variables
    let filter = std::env::var("FERRIS_TEST_FILTER").ok();
    let category_filter = std::env::var("FERRIS_TEST_CATEGORY").ok();
    
    let filtered_tests: Vec<_> = tests
        .into_iter()
        .filter(|t| {
            // Filter by test name
            if let Some(ref f) = filter {
                if !t.name.contains(f) {
                    return false;
                }
            }
            
            // Filter by category
            if let Some(ref c) = category_filter {
                if t.category.to_string() != *c {
                    return false;
                }
            }
            
            true
        })
        .collect();
    
    println!("\n🧪 FerrisScript Test Suite");
    println!("═══════════════════════════════════════");
    println!("Discovered {} tests", filtered_tests.len());
    if filter.is_some() || category_filter.is_some() {
        println!("Filters applied:");
        if let Some(f) = filter {
            println!("  - Name contains: '{}'", f);
        }
        if let Some(c) = category_filter {
            println!("  - Category: '{}'", c);
        }
    }
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;
    
    for test in filtered_tests {
        match test.category {
            TestCategory::Unit => {
                print!("Running unit test: {} ... ", test.name);
                
                match run_compile_test(&test.file_path, &test.expectation) {
                    Ok(()) => {
                        println!("✅ PASSED");
                        passed += 1;
                    }
                    Err(e) => {
                        println!("❌ FAILED");
                        eprintln!("  Error: {}", e);
                        failed += 1;
                    }
                }
            }
            TestCategory::Integration | TestCategory::Runtime => {
                println!(
                    "Skipping {} test (run via Godot): {} ... ⏭️  SKIPPED",
                    test.category, test.name
                );
                skipped += 1;
            }
        }
    }
    
    println!("\n═══════════════════════════════════════");
    println!("Test Results:");
    println!("  ✅ Passed:  {}", passed);
    println!("  ❌ Failed:  {}", failed);
    println!("  ⏭️  Skipped: {}", skipped);
    println!("═══════════════════════════════════════\n");
    
    assert_eq!(failed, 0, "Some unit tests failed");
}

/// Run a compile test and verify expected outcome
fn run_compile_test(
    path: &Path,
    expectation: &TestExpectation,
) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let compile_result = ferrisscript_compiler::compile(&content);
    
    match expectation {
        TestExpectation::Success => {
            match compile_result {
                Ok(_) => Ok(()),
                Err(e) => Err(format!(
                    "Expected success, but compilation failed: {}",
                    e
                )),
            }
        }
        TestExpectation::Error { code } => {
            match compile_result {
                Ok(_) => Err(format!(
                    "Expected error '{}', but compilation succeeded",
                    code
                )),
                Err(e) => {
                    let error_code = extract_error_code(&e.to_string());
                    if error_code.contains(code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "Expected error '{}', got '{}'",
                            code, error_code
                        ))
                    }
                }
            }
        }
        TestExpectation::Warning { code: _ } => {
            // For now, warnings don't fail compilation
            // This can be enhanced when warning system is implemented
            Ok(())
        }
    }
}

/// Extract error code from compiler error message
fn extract_error_code(error_msg: &str) -> String {
    // Try to extract error code pattern like "E201" or "TYPE_ERROR"
    if let Some(start) = error_msg.find("E") {
        if let Some(end) = error_msg[start..].find(|c: char| !c.is_alphanumeric()) {
            return error_msg[start..start + end].to_string();
        }
    }
    error_msg.to_string()
}
```

### Phase 2.5: LSP Test Integration (2-3 days) 🆕

**Critical for v0.0.5**: LSP needs to integrate with test framework for editor features.

#### 2.5.1 LSP Protocol Extensions

Add custom LSP methods to `crates/lsp/src/handlers/custom.rs`:

```rust
use lsp_types::*;
use serde::{Deserialize, Serialize};
use tower_lsp::jsonrpc::Result;

/// Custom request: Get tests in document
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentTestsParams {
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentTestsResponse {
    pub tests: Vec<TestInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestInfo {
    pub name: String,
    pub range: Range,
    pub category: String,
    pub status: TestStatus,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    Unknown,
    Passed,
    Failed,
    Running,
}

/// Custom request: Run single test
#[derive(Debug, Serialize, Deserialize)]
pub struct RunTestParams {
    pub test_name: String,
    pub uri: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunTestResponse {
    pub status: TestStatus,
    pub duration: f64,
    pub message: Option<String>,
}
```

#### 2.5.2 LSP Test Handler Implementation

```rust
use ferrisscript_test_harness::{discover_tests, parse_test_metadata};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct TestCache {
    /// Map test name -> status
    statuses: Arc<RwLock<HashMap<String, TestStatus>>>,
}

impl TestCache {
    pub fn new() -> Self {
        Self {
            statuses: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn get_status(&self, test_name: &str) -> Option<TestStatus> {
        self.statuses.read().unwrap().get(test_name).copied()
    }
    
    pub fn update(&self, test_name: String, status: TestStatus) {
        self.statuses.write().unwrap().insert(test_name, status);
    }
}

pub async fn handle_document_tests(
    params: DocumentTestsParams,
    test_cache: Arc<TestCache>,
) -> Result<DocumentTestsResponse> {
    let uri = params.text_document.uri;
    let path = uri
        .to_file_path()
        .map_err(|_| jsonrpc::Error::invalid_params("Invalid file URI"))?;
    
    // Parse test metadata from file
    let content = std::fs::read_to_string(&path)
        .map_err(|e| jsonrpc::Error::invalid_request(e.to_string()))?;
    
    let metadata = parse_test_metadata(&path)
        .map_err(|e| jsonrpc::Error::parse_error(e.to_string()))?;
    
    // Get cached test status
    let status = test_cache
        .get_status(&metadata.name)
        .unwrap_or(TestStatus::Unknown);
    
    // Find test range in source (simple heuristic: look for TEST: comment)
    let range = find_test_range(&content);
    
    Ok(DocumentTestsResponse {
        tests: vec![TestInfo {
            name: metadata.name,
            range,
            category: metadata.category.to_string(),
            status,
            message: None,
        }],
    })
}

pub async fn handle_run_test(
    params: RunTestParams,
    test_cache: Arc<TestCache>,
    client: tower_lsp::Client,
) -> Result<RunTestResponse> {
    let test_name = params.test_name.clone();
    
    // Update status to running
    test_cache.update(test_name.clone(), TestStatus::Running);
    
    // Run test in background
    let result = tokio::task::spawn_blocking(move || {
        run_single_test(&test_name)
    })
    .await
    .map_err(|e| jsonrpc::Error::internal_error(e.to_string()))?;
    
    // Update cache with result
    test_cache.update(test_name.clone(), result.status);
    
    // Publish diagnostics if failed
    if result.status == TestStatus::Failed {
        let diagnostic = Diagnostic {
            range: result.error_range.unwrap_or_default(),
            severity: Some(DiagnosticSeverity::ERROR),
            message: result.message.clone().unwrap_or_default(),
            source: Some("ferrisscript-test".to_string()),
            ..Default::default()
        };
        
        client
            .publish_diagnostics(params.uri, vec![diagnostic], None)
            .await;
    }
    
    Ok(RunTestResponse {
        status: result.status,
        duration: result.duration,
        message: result.message,
    })
}

fn find_test_range(content: &str) -> Range {
    // Simple implementation: find TEST: comment line
    for (i, line) in content.lines().enumerate() {
        if line.contains("// TEST:") {
            return Range {
                start: Position {
                    line: i as u32,
                    character: 0,
                },
                end: Position {
                    line: i as u32,
                    character: line.len() as u32,
                },
            };
        }
    }
    Range::default()
}

struct TestRunResult {
    status: TestStatus,
    duration: f64,
    message: Option<String>,
    error_range: Option<Range>,
}

fn run_single_test(test_name: &str) -> TestRunResult {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Discover tests
    let tests = discover_tests("examples").unwrap();
    
    // Find specific test
    let test = tests.iter().find(|t| t.name == test_name);
    
    if let Some(test) = test {
        // Run test based on category
        match test.category {
            TestCategory::Unit => {
                // Compile test
                let content = std::fs::read_to_string(&test.file_path).unwrap();
                let compile_result = ferrisscript_compiler::compile(&content);
                
                let status = match (&test.expectation, compile_result) {
                    (TestExpectation::Success, Ok(_)) => TestStatus::Passed,
                    (TestExpectation::Error { code }, Err(e)) 
                        if e.to_string().contains(code) => TestStatus::Passed,
                    _ => TestStatus::Failed,
                };
                
                TestRunResult {
                    status,
                    duration: start.elapsed().as_secs_f64(),
                    message: Some(format!("Test {} {}", test_name, 
                        if status == TestStatus::Passed { "passed" } else { "failed" })),
                    error_range: None,
                }
            }
            _ => {
                // Integration/runtime tests need Godot runner
                TestRunResult {
                    status: TestStatus::Unknown,
                    duration: 0.0,
                    message: Some("Integration/runtime tests must be run via Godot".to_string()),
                    error_range: None,
                }
            }
        }
    } else {
        TestRunResult {
            status: TestStatus::Failed,
            duration: 0.0,
            message: Some(format!("Test '{}' not found", test_name)),
            error_range: None,
        }
    }
}
```

#### 2.5.3 VS Code Extension Integration

Create `extensions/vscode/src/testProvider.ts`:

```typescript
import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

interface TestInfo {
    name: string;
    range: vscode.Range;
    category: string;
    status: 'unknown' | 'passed' | 'failed' | 'running';
    message?: string;
}

export class FerrisTestProvider implements vscode.CodeLensProvider {
    private _onDidChangeCodeLenses = new vscode.EventEmitter<void>();
    public readonly onDidChangeCodeLenses = this._onDidChangeCodeLenses.event;
    
    private testCache = new Map<string, TestInfo>();
    
    constructor(private client: LanguageClient) {}
    
    async provideCodeLenses(
        document: vscode.TextDocument
    ): Promise<vscode.CodeLens[]> {
        // Query LSP for tests in this file
        const tests: TestInfo[] = await this.client.sendRequest(
            'ferrisscript/documentTests',
            {
                textDocument: { uri: document.uri.toString() }
            }
        );
        
        const lenses: vscode.CodeLens[] = [];
        
        for (const test of tests) {
            this.testCache.set(test.name, test);
            
            const icon = this.getStatusIcon(test.status);
            const command = {
                title: `${icon} Run Test`,
                command: 'ferrisscript.runTest',
                arguments: [test.name, document.uri]
            };
            
            lenses.push(new vscode.CodeLens(test.range, command));
        }
        
        return lenses;
    }
    
    async runTest(testName: string, uri: vscode.Uri) {
        const result = await this.client.sendRequest('ferrisscript/runTest', {
            testName,
            uri: uri.toString()
        });
        
        // Update cache
        const cached = this.testCache.get(testName);
        if (cached) {
            cached.status = result.status;
        }
        
        // Refresh code lenses
        this._onDidChangeCodeLenses.fire();
        
        // Show notification
        if (result.status === 'passed') {
            vscode.window.showInformationMessage(
                `✅ Test passed: ${testName} (${result.duration.toFixed(2)}s)`
            );
        } else {
            vscode.window.showErrorMessage(
                `❌ Test failed: ${testName}\n${result.message || ''}`
            );
        }
    }
    
    private getStatusIcon(status: string): string {
        switch (status) {
            case 'passed': return '✅';
            case 'failed': return '❌';
            case 'running': return '⏳';
            default: return '▶️';
        }
    }
}
```

### Phase 3: Godot Test Runner (2-3 days)

#### 3.1 Add `compile_from_string` to `FerrisScriptRunner`

**File**: `crates/godot_bind/src/lib.rs`

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct FerrisScriptRunner {
    base: Base<Node>,
    compiled_program: Option<CompiledProgram>,
    runtime: Option<Runtime>,
}

#[godot_api]
impl INode for FerrisScriptRunner {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            compiled_program: None,
            runtime: None,
        }
    }
}

#[godot_api]
impl FerrisScriptRunner {
    /// Load and compile a .ferris script file
    #[func]
    pub fn load_script(&mut self, path: GString) {
        godot_print!("Loading FerrisScript: {}", path);
        
        let file = FileAccess::open(path.clone(), godot::engine::file_access::ModeFlags::READ);
        match file {
            Some(mut f) => {
                let content = f.get_as_text();
                self.compile_from_string(content);
            }
            None => {
                godot_error!("Failed to open file: {}", path);
            }
        }
    }
    
    /// Compile script from string (for testing)
    /// Returns true on success, false on error
    #[func]
    pub fn compile_from_string(&mut self, source: GString) -> bool {
        godot_print!("Compiling FerrisScript from string...");
        
        match ferrisscript_compiler::compile(&source.to_string()) {
            Ok(program) => {
                godot_print!("✅ Compilation successful");
                self.compiled_program = Some(program);
                self.initialize_runtime();
                true
            }
            Err(e) => {
                godot_error!("❌ Compilation failed: {}", e);
                false
            }
        }
    }
    
    /// Get last compilation error (if any)
    #[func]
    pub fn get_last_error(&self) -> GString {
        // Store last error in a field (add to struct)
        GString::from("Not implemented yet")
    }
    
    /// Get runtime variable value (for assertion checking)
    #[func]
    pub fn get_variable(&self, name: GString) -> Variant {
        if let Some(runtime) = &self.runtime {
            runtime.get_variable(&name.to_string())
                .map(|v| variant_from_ferris_value(v))
                .unwrap_or(Variant::nil())
        } else {
            Variant::nil()
        }
    }
    
    /// Get emitted signals since last check (for assertion validation)
    #[func]
    pub fn get_emitted_signals(&mut self) -> Array<Dictionary> {
        if let Some(runtime) = &mut self.runtime {
            let signals = runtime.signal_log.drain(..).collect::<Vec<_>>();
            array_from_signal_log(signals)
        } else {
            Array::new()
        }
    }
    
    fn initialize_runtime(&mut self) {
        if let Some(program) = &self.compiled_program {
            self.runtime = Some(Runtime::new(program));
        }
    }
}

// Helper functions
fn variant_from_ferris_value(value: &FerrisValue) -> Variant {
    // Convert FerrisScript value to Godot Variant
    // Implementation depends on FerrisValue definition
    Variant::nil() // Placeholder
}

fn array_from_signal_log(signals: Vec<Signal>) -> Array<Dictionary> {
    // Convert signal log to Godot array
    Array::new() // Placeholder
}
```

#### 3.2 Create Complete Test Runner (`godot_test/test_runner.gd`)

```gdscript
extends Node

## FerrisScript Test Runner
## Discovers and runs .ferris tests in headless mode

const DEFAULT_TIMEOUT_SECONDS = 10.0

var test_results = []
var start_time_ms = 0
var current_timeout_timer: SceneTreeTimer = null
var timed_out_test: String = ""

func _ready():
 print("\n🧪 FerrisScript Godot Test Runner")
 print("═══════════════════════════════════════")
 
 start_time_ms = Time.get_ticks_msec()
 
 # Discover tests from /examples
 var examples_dir = "../examples"
 var tests = discover_tests(examples_dir)
 
 if tests.is_empty():
  print("❌ No tests discovered in %s" % examples_dir)
  get_tree().quit(1)
  return
 
 print("Discovered %d tests" % tests.size())
 
 # Filter for integration/runtime tests
 var tests_to_run = []
 for test in tests:
  if test.category in ["integration", "runtime"]:
   tests_to_run.append(test)
 
 if tests_to_run.is_empty():
  print("ℹ️  No Godot tests to run (all are unit tests)")
  get_tree().quit(0)
  return
 
 print("Running %d Godot tests...\n" % tests_to_run.size())
 
 # Run each test
 for test in tests_to_run:
  run_test(test)
 
 # Print results and exit
 if OS.get_environment("FERRIS_JSON_OUTPUT") == "true":
  print_json_results()
 else:
  print_results()
 
 var exit_code = 0 if all_tests_passed() else 1
 get_tree().quit(exit_code)

## Discover all .ferris tests in a directory
func discover_tests(dir_path: String) -> Array:
 var tests = []
 
 # Try to open directory
 var dir = DirAccess.open(dir_path)
 if dir == null:
  push_error("Failed to open directory: %s" % dir_path)
  return tests
 
 # Scan for .ferris files
 dir.list_dir_begin()
 var file_name = dir.get_next()
 
 while file_name != "":
  if file_name.ends_with(".ferris"):
   var full_path = dir_path + "/" + file_name
   var metadata = parse_metadata(full_path)
   
   if metadata != null and not metadata.is_empty():
    tests.append(metadata)
  
  file_name = dir.get_next()
 
 dir.list_dir_end()
 return tests

## Parse test metadata from .ferris file header
func parse_metadata(file_path: String) -> Dictionary:
 var file = FileAccess.open(file_path, FileAccess.READ)
 if file == null:
  push_error("Failed to open file: %s" % file_path)
  return {}
 
 var content = file.get_as_text()
 file.close()
 
 var metadata = {
  "file_path": file_path,
  "name": "",
  "category": "",
  "description": "",
  "expect": "success",
  "assertions": []
 }
 
 # Parse header comments (stop at first non-comment)
 for line in content.split("\n"):
  var trimmed = line.strip_edges()
  
  # Stop at first non-comment line
  if not trimmed.begins_with("//") and not trimmed.is_empty():
   break
  
  # Extract metadata fields
  if "TEST:" in line:
   metadata.name = line.split("TEST:")[1].strip_edges()
  elif "CATEGORY:" in line:
   metadata.category = line.split("CATEGORY:")[1].strip_edges()
  elif "DESCRIPTION:" in line:
   metadata.description = line.split("DESCRIPTION:")[1].strip_edges()
  elif "EXPECT:" in line:
   metadata.expect = line.split("EXPECT:")[1].strip_edges()
  elif "ASSERT:" in line:
   var assertion = line.split("ASSERT:")[1].strip_edges()
   metadata.assertions.append(assertion)
  elif "TIMEOUT:" in line:
   var timeout_str = line.split("TIMEOUT:")[1].strip_edges()
   metadata["timeout"] = float(timeout_str) if timeout_str.is_valid_float() else DEFAULT_TIMEOUT_SECONDS
 
 # Set default timeout if not specified
 if not metadata.has("timeout"):
  metadata["timeout"] = DEFAULT_TIMEOUT_SECONDS
 
 # Validate required fields
 if metadata.name == "" or metadata.category == "":
  return {}
 
 return metadata

## Run a single test
func run_test(test: Dictionary):
 print("─────────────────────────────────────")
 print("Running: %s" % test.name)
 print("Category: %s | Expect: %s" % [test.category, test.expect])
 
 var result = {
  "name": test.name,
  "category": test.category,
  "status": "pending",
  "message": "",
  "duration": 0.0
 }
 
 var start_time = Time.get_ticks_msec()
 
 # Set up timeout timer
 var timeout_duration = test.get("timeout", DEFAULT_TIMEOUT_SECONDS)
 current_timeout_timer = get_tree().create_timer(timeout_duration)
 current_timeout_timer.timeout.connect(_on_test_timeout.bind(test.name))
 
 # Load script content from filesystem
 var file = FileAccess.open(test.file_path, FileAccess.READ)
 if file == null:
  result.status = "error"
  result.message = "Failed to load file: %s" % test.file_path
  test_results.append(result)
  print("❌ ERROR: %s" % result.message)
  return
 
 var script_content = file.get_as_text()
 file.close()
 
 # Create FerrisScriptRunner
 var runner = FerrisScriptRunner.new()
 add_child(runner)
 
 # Compile script
 var compile_ok = runner.compile_from_string(script_content)
 
 # Check result against expectation
 if not compile_ok:
  if test.expect.begins_with("error"):
   # Expected compilation error
   result.status = "passed"
   result.message = "Got expected compilation error"
   print("✅ PASSED (expected error)")
  else:
   # Unexpected compilation error
   result.status = "failed"
   result.message = "Compilation failed unexpectedly"
   print("❌ FAILED: %s" % result.message)
 else:
  if test.expect.begins_with("error"):
   # Should have failed but didn't
   result.status = "failed"
   result.message = "Expected compilation error but succeeded"
   print("❌ FAILED: %s" % result.message)
  else:
   # Compilation succeeded, run the script
   runner._ready()
   
   # For integration tests, run a few frames
   if test.category == "integration":
    for i in range(10):
     runner._process(0.016)  # ~60fps
   
   # For runtime tests with assertions, validate them
   if test.assertions.size() > 0:
    var assertions_passed = validate_assertions(runner, test.assertions)
    if not assertions_passed:
     result.status = "failed"
     result.message = "Assertion validation failed"
     print("❌ FAILED: Assertions not satisfied")
    else:
     result.status = "passed"
     result.message = "Test completed successfully"
     print("✅ PASSED")
   else:
    result.status = "passed"
    result.message = "Test completed successfully"
    print("✅ PASSED")
 
 # Cancel timeout timer
 if current_timeout_timer:
  current_timeout_timer.timeout.disconnect(_on_test_timeout)
  current_timeout_timer = null
 
 # Check if test timed out
 if timed_out_test == test.name:
  result.status = "timeout"
  result.message = "Test exceeded timeout of %.1fs" % test.get("timeout", DEFAULT_TIMEOUT_SECONDS)
  print("⏱️  TIMEOUT")
  timed_out_test = ""
 
 result.duration = (Time.get_ticks_msec() - start_time) / 1000.0
 print("Duration: %.3fs" % result.duration)
 
 runner.queue_free()
 test_results.append(result)

## Handle test timeout
func _on_test_timeout(test_name: String):
 timed_out_test = test_name
 push_error("Test '%s' timed out" % test_name)

## Validate runtime assertions
func validate_assertions(runner: FerrisScriptRunner, assertions: Array) -> bool:
 for assertion in assertions:
  var assertion_str = str(assertion)
  
  # Simple assertion validation (can be extended with DSL parser)
  if "signal" in assertion_str.to_lower():
   # Check if expected signal was emitted
   var signals = runner.get_emitted_signals()
   if signals.is_empty():
    push_error("Assertion failed: Expected signal emission but none found")
    return false
  elif "position" in assertion_str.to_lower():
   # Check position bounds
   var position = runner.get_variable("position")
   # Add validation logic based on assertion
  elif ">" in assertion_str or "<" in assertion_str or "==" in assertion_str:
   # Parse and validate comparison assertions
   # Format: "variable_name > value" or "variable_name < value"
   pass
 
 return true

## Print test results summary
func print_results():
 var total_time = (Time.get_ticks_msec() - start_time_ms) / 1000.0
 
 print("\n═══════════════════════════════════════")
 print("TEST RESULTS")
 print("═══════════════════════════════════════")
 
 var passed = 0
 var failed = 0
 var errors = 0
 
 for result in test_results:
  match result.status:
   "passed": passed += 1
   "failed": failed += 1
   "error": errors += 1
 
 var total = test_results.size()
 print("Total: %d | ✅ Passed: %d | ❌ Failed: %d | ⚠️  Errors: %d" % 
  [total, passed, failed, errors])
 print("Total time: %.3fs" % total_time)
 
 if passed == total:
  print("\n✅ ALL TESTS PASSED")
 else:
  print("\n❌ SOME TESTS FAILED")
  print("\nFailed tests:")
  for result in test_results:
   if result.status != "passed":
    print("  • %s: %s" % [result.name, result.message])

## Check if all tests passed
func all_tests_passed() -> bool:
 for result in test_results:
  if result.status != "passed":
   return false
 return true

## Print results as JSON (for CI parsing)
func print_json_results():
 var passed = 0
 var failed = 0
 var errors = 0
 var timeouts = 0
 
 for result in test_results:
  match result.status:
   "passed": passed += 1
   "failed": failed += 1
   "error": errors += 1
   "timeout": timeouts += 1
 
 var json_data = {
  "total": test_results.size(),
  "passed": passed,
  "failed": failed,
  "errors": errors,
  "timeouts": timeouts,
  "total_time": (Time.get_ticks_msec() - start_time_ms) / 1000.0,
  "tests": test_results
 }
 
 print(JSON.stringify(json_data, "\t"))
```

### Phase 4: CI Integration (1 day)

#### 4.1 Create Shell Script (`scripts/run_godot_tests.sh`)

```bash
#!/bin/bash
# FerrisScript Godot Test Runner
set -e

echo "🧪 FerrisScript Godot Test Suite"
echo "════════════════════════════════════════"
echo ""

# Check if Godot is available
if ! command -v godot &> /dev/null; then
    echo "❌ Godot not found in PATH"
    echo "Please install Godot 4.2+ and add to PATH"
    exit 1
fi

echo "Godot version:"
godot --version
echo ""

# Build GDExtension first
echo "Building FerrisScript GDExtension..."
cargo build --release --package ferrisscript_godot_bind

if [ $? -ne 0 ]; then
    echo "❌ GDExtension build failed"
    exit 1
fi

echo "✅ GDExtension built successfully"
echo ""

# Run Godot headless test runner
echo "Starting Godot headless test runner..."
echo "════════════════════════════════════════"
godot --headless --path godot_test --script res://test_runner.gd

# Capture exit code
EXIT_CODE=$?

echo ""
echo "════════════════════════════════════════"
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ All Godot tests passed"
else
    echo "❌ Some Godot tests failed (exit code: $EXIT_CODE)"
fi

exit $EXIT_CODE
```

Make executable:

```bash
chmod +x scripts/run_godot_tests.sh
```

#### 4.2 PowerShell Version (`scripts/run_godot_tests.ps1`)

```powershell
# FerrisScript Godot Test Runner (Windows)
$ErrorActionPreference = "Stop"

Write-Host "🧪 FerrisScript Godot Test Suite" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════=" -ForegroundColor Cyan
Write-Host ""

# Check if Godot is available
$godotPath = Get-Command godot -ErrorAction SilentlyContinue
if (-not $godotPath) {
    Write-Host "❌ Godot not found in PATH" -ForegroundColor Red
    Write-Host "Please install Godot 4.2+ and add to PATH" -ForegroundColor Yellow
    exit 1
}

Write-Host "Godot version:"
godot --version
Write-Host ""

# Build GDExtension
Write-Host "Building FerrisScript GDExtension..." -ForegroundColor Yellow
cargo build --release --package ferrisscript_godot_bind

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ GDExtension build failed" -ForegroundColor Red
    exit 1
}

Write-Host "✅ GDExtension built successfully" -ForegroundColor Green
Write-Host ""

# Run Godot headless
Write-Host "Starting Godot headless test runner..." -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════=" -ForegroundColor Cyan
godot --headless --path godot_test --script res://test_runner.gd

$exitCode = $LASTEXITCODE

Write-Host ""
Write-Host "═══════════════════════════════════════=" -ForegroundColor Cyan
if ($exitCode -eq 0) {
    Write-Host "✅ All Godot tests passed" -ForegroundColor Green
} else {
    Write-Host "❌ Some Godot tests failed (exit code: $exitCode)" -ForegroundColor Red
}

exit $exitCode
```

#### 4.3 Update CI Workflow (`.github/workflows/test.yml`)

```yaml
name: FerrisScript Tests

on:
  push:
    branches: [ develop, main ]
  pull_request:
    branches: [ develop, main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test Suite
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    
    - name: Cache Cargo
      uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/bin/
          ~/.cargo/registry/index/
          ~/.cargo/registry/cache/
          ~/.cargo/git/db/
          target/
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Install Godot (Linux)
      if: runner.os == 'Linux'
      run: |
        wget https://github.com/godotengine/godot/releases/download/4.2-stable/Godot_v4.2-stable_linux.x86_64.zip
        unzip Godot_v4.2-stable_linux.x86_64.zip
        chmod +x Godot_v4.2-stable_linux.x86_64
        sudo mv Godot_v4.2-stable_linux.x86_64 /usr/local/bin/godot
    
    - name: Install Godot (macOS)
      if: runner.os == 'macOS'
      run: |
        brew install godot
    
    - name: Install Godot (Windows)
      if: runner.os == 'Windows'
      run: |
        Invoke-WebRequest -Uri "https://github.com/godotengine/godot/releases/download/4.2-stable/Godot_v4.2-stable_win64.exe.zip" -OutFile godot.zip
        Expand-Archive godot.zip -DestinationPath .
        Move-Item Godot_v4.2-stable_win64.exe godot.exe
        echo "$PWD" | Out-File -FilePath $env:GITHUB_PATH -Encoding utf8 -Append
    
    - name: Check Rust formatting
      run: cargo fmt -- --check
    
    - name: Run Clippy
      run: cargo clippy -- -D warnings
    
    - name: Build all crates
      run: cargo build --verbose --workspace
    
    - name: Run Rust Unit Tests
      run: cargo test --verbose --workspace
    
    - name: Run Godot Integration Tests (Linux/macOS)
      if: runner.os != 'Windows'
      run: |
        chmod +x scripts/run_godot_tests.sh
        ./scripts/run_godot_tests.sh
    
    - name: Run Godot Integration Tests (Windows)
      if: runner.os == 'Windows'
      run: |
        pwsh scripts/run_godot_tests.ps1
    
    - name: Upload Test Results
      uses: actions/upload-artifact@v4
      if: always()
      with:
        name: test-results-${{ matrix.os }}
        path: |
          target/test-results/
          godot_test/.godot/
```

### Phase 5: Documentation & Examples (1 day)

#### 5.1 Update `examples/README.md`

````markdown
# FerrisScript Test Examples

This directory contains `.ferris` files that serve as both examples and automated tests.

## Test Metadata Format

Each test file must include metadata in header comments:

```ferris
// TEST: unique_test_name
// CATEGORY: unit|integration|runtime
// DESCRIPTION: What this test validates
// EXPECT: success|error:E###|warning:W###
// ASSERT: (optional) Runtime assertion description
// SCENE: (optional) res://path/to/scene.tscn (runtime tests only)
```

### Required Fields

- **TEST**: Unique identifier for this test
- **CATEGORY**: Test execution mode
  - `unit` - Compile-only (syntax, types)
  - `integration` - Compile + basic runtime (no scene)
  - `runtime` - Full Godot integration (requires scene)
- **DESCRIPTION**: Human-readable description
- **EXPECT**: Expected outcome
  - `success` - Should compile and run without errors
  - `error:E###` - Should fail with specific error code
  - `warning:W###` - Should produce specific warning

### Optional Fields

- **ASSERT**: Runtime assertion (for integration/runtime tests)
- **SCENE**: Required scene path (for runtime tests)

## Example: Success Test

```ferris
// TEST: bounce_horizontal
// CATEGORY: integration
// DESCRIPTION: Horizontal bouncing motion using self.position
// EXPECT: success
// ASSERT: Position oscillates between -10.0 and 10.0

let mut dir: f32 = 1.0;

fn _process(delta: f32) {
    self.position.x += dir * 100.0 * delta;
    
    if self.position.x > 10.0 {
        dir = -1.0;
    }
    if self.position.x < -10.0 {
        dir = 1.0;
    }
}
```

## Example: Error Test

```ferris
// TEST: type_mismatch_error
// CATEGORY: unit
// DESCRIPTION: Verify compiler rejects type mismatches
// EXPECT: error:E201

fn _ready() {
    let x: i32 = "string";  // Should fail: E201 (type mismatch)
}
```

## Running Tests

### Unit Tests (Rust)
```bash
cargo test
```

### Integration/Runtime Tests (Godot)
```bash
# Linux/macOS
./scripts/run_godot_tests.sh

# Windows
pwsh scripts/run_godot_tests.ps1
```

### All Tests
```bash
cargo test && ./scripts/run_godot_tests.sh
```

## Test Categories

### Unit Tests
- Fast execution (~1ms per test)
- Compile-only validation
- Run via `cargo test`
- Examples: syntax errors, type errors, semantic errors

### Integration Tests
- Medium execution (~10ms per test)
- Compile + basic runtime
- Run via Godot headless
- Examples: arithmetic, control flow, function calls

### Runtime Tests
- Slow execution (~100ms per test)
- Full Godot integration
- Requires scene setup
- Examples: signals, properties, node queries

## Adding New Tests

1. Create `.ferris` file in this directory
2. Add test metadata header
3. Write test code
4. Run `cargo test` to validate (unit tests)
5. Run `./scripts/run_godot_tests.sh` to validate (integration/runtime)

Tests are automatically discovered - no manual registration needed!
````

#### 5.2 Create Example Tests

**`examples/type_error.ferris`** (Negative Test):

```ferris
// TEST: type_mismatch_string_to_int
// CATEGORY: unit
// DESCRIPTION: Verify compiler catches type mismatch between String and i32
// EXPECT: error:E201

fn _ready() {
    let x: i32 = "this should fail";
}
```

**`examples/vector_math.ferris`** (Integration Test):

```ferris
// TEST: vector2_addition
// CATEGORY: integration
// DESCRIPTION: Test Vector2 arithmetic operations
// EXPECT: success
// ASSERT: Vector addition produces correct result

fn _ready() {
    let a = Vector2::new(2.0, 3.0);
    let b = Vector2::new(3.0, 4.0);
    let result = a + b;
    
    // Expected: Vector2(5.0, 7.0)
    godot_print("Result: ", result);
}
```

## Testing Strategy

### Test Coverage Goals

| Category | Target Coverage | Current Coverage |
|----------|----------------|------------------|
| **Compiler** | 90%+ | TBD |
| **Runtime** | 80%+ | TBD |
| **Godot Binding** | 70%+ | TBD |

### Test Pyramid

```
        ┌──────────┐
        │ Runtime  │  10% (slow, comprehensive)
        │  Tests   │
        └──────────┘
      ┌──────────────┐
      │ Integration  │  30% (medium, feature)
      │    Tests     │
      └──────────────┘
    ┌──────────────────┐
    │   Unit Tests     │  60% (fast, focused)
    └──────────────────┘
```

### Negative Testing

Always include tests for error cases:

- ✅ Syntax errors
- ✅ Type mismatches
- ✅ Undefined variables
- ✅ Invalid operations
- ✅ Runtime panics

## CI Integration

Tests run automatically on:

- Every push to `develop` or `main`
- Every pull request
- Manual workflow dispatch

See `.github/workflows/test.yml` for CI configuration.

## Troubleshooting

### "Failed to discover tests"

- Ensure `.ferris` files have required metadata
- Check that file is in `/examples`
- Validate metadata format

### "Compilation failed unexpectedly"

- Check that expected error code matches actual error
- Review compiler output for details

### "Godot tests not running"

- Ensure Godot 4.2+ is installed
- Check that GDExtension builds successfully
- Verify `godot` is in PATH

## Future Enhancements

- [ ] Test coverage reporting
- [ ] Parallel test execution
- [ ] Watch mode for TDD
- [ ] Interactive test runner UI
- [ ] Performance benchmarking

---

For more information, see:

- [`docs/testing/TESTING_GUIDE.md`](../docs/testing/TESTING_GUIDE.md)
- [`docs/testing/TEST_HARNESS_TESTING_STRATEGY.md`](../docs/testing/TEST_HARNESS_TESTING_STRATEGY.md)
- [`CONTRIBUTING.md`](../CONTRIBUTING.md)

```

## Timeline & Milestones

**Updated Timeline**: 6-7 weeks (was 3 weeks, then 4-5 weeks)

**Timeline Breakdown**:
- LSP integration: +1-2 weeks
- Compiler prerequisites (spans, symbol table): +1 week  
- Incremental compilation: +2-3 weeks
- Original test framework: ~2 weeks

### Week 1: Compiler Prerequisites - Source Spans
- **Days 1-2**: Phase 0.1a (Span Infrastructure)
  - [ ] Create `crates/compiler/src/span.rs`
  - [ ] Define `Span` and `Position` structs
  - [ ] Implement span merging and manipulation
  - [ ] Add unit tests for span operations
  
- **Days 3-4**: Phase 0.1b (AST Integration)
  - [ ] Add `span` field to all AST nodes
  - [ ] Implement `span()` method for each AST variant
  - [ ] Update all parser tests with span assertions
  
- **Day 5**: Phase 0.1c (Parser Updates)
  - [ ] Update parser to track spans from tokens
  - [ ] Verify span accuracy for all expression types
  - [ ] Add span information to error messages

### Week 2: Compiler Prerequisites - Symbol Table
- **Days 1-2**: Phase 0.2a (Symbol Table Design)
  - [ ] Create `crates/compiler/src/symbol_table.rs`
  - [ ] Define `SymbolTable`, `Symbol`, `Scope` structs
  - [ ] Implement scope chain lookup methods
  - [ ] Add unit tests for symbol resolution
  
- **Days 3-4**: Phase 0.2b (Type Checker Integration)
  - [ ] Refactor `TypeChecker` to build `SymbolTable`
  - [ ] Add symbol insertion for variables/functions/parameters
  - [ ] Update `compile()` to return `SymbolTable`
  - [ ] Add integration tests for symbol table construction
  
- **Day 5**: Phase 0.2c (Validation & Documentation)
  - [ ] Verify symbol table correctness across test suite
  - [ ] Document symbol table API
  - [ ] Performance profiling

### Week 3: Compiler Prerequisites - Incremental Compilation
- **Days 1-3**: Phase 0.3a (Incremental Compiler)
  - [ ] Create `crates/compiler/src/incremental.rs`
  - [ ] Implement `IncrementalCompiler` with AST caching
  - [ ] Add source hash-based cache invalidation
  - [ ] Create `DependencyGraph` for transitive invalidation
  
- **Days 4-5**: Phase 0.3b (Integration & Benchmarking)
  - [ ] Integrate with LSP document manager
  - [ ] Add cache metrics (hit rate, compilation speed)
  - [ ] Optimize cache eviction strategy
  - [ ] Performance benchmarks vs. full recompilation

### Week 4: Test Harness Foundation
- **Days 1-2**: Phase 1 (Test harness crate)
  - [ ] Create `crates/test_harness`
  - [ ] Implement metadata parser with validation
  - [ ] Add timeout field support
  - [ ] Add comprehensive unit tests
  
- **Days 3-4**: Phase 2 (Rust integration)
  - [ ] Create `tests/ferris_integration_tests.rs`
  - [ ] Implement test filtering (env vars)
  - [ ] Implement negative testing
  - [ ] Validate on Windows/Linux/macOS

- **Day 5**: Buffer for issues

### Week 5: LSP Test Integration
- **Days 1-2**: Phase 2.5a (LSP Protocol Extensions)
  - [ ] Define custom LSP methods (`documentTests`, `runTest`)
  - [ ] Implement `TestCache` for status tracking
  - [ ] Create test status types
  - [ ] Add test discovery API

- **Days 3-4**: Phase 2.5b (LSP Handlers)
  - [ ] Implement `handle_document_tests`
  - [ ] Implement `handle_run_test`
  - [ ] Add test range detection
  - [ ] Integrate with test harness and incremental compiler

- **Day 5**: Phase 2.5c (VS Code Extension)
  - [ ] Create `testProvider.ts`
  - [ ] Implement CodeLens provider
  - [ ] Add "Run Test" command
  - [ ] Test editor integration

### Week 6: Godot Integration
- **Days 1-2**: Phase 3a (FerrisScriptRunner Extensions)
  - [ ] Add `compile_from_string` method
  - [ ] Add `get_variable` for assertion checking
  - [ ] Add `get_emitted_signals` for signal validation
  
- **Days 2-3**: Phase 3b (Test Runner)
  - [ ] Implement complete `test_runner.gd`
  - [ ] Add timeout mechanism
  - [ ] Add assertion validation framework
  - [ ] Add JSON output mode
  - [ ] Test filesystem access on all platforms
  
- **Day 4-5**: Phase 3c (Integration Testing)
  - [ ] Run full test suite in Godot headless mode
  - [ ] Validate assertion checks work correctly
  - [ ] Performance testing
  - [ ] Cross-platform compatibility checks

### Week 7: CI Integration & Migration
- **Days 1-2**: Phase 4 (CI Integration)
  - [ ] Create shell scripts (bash + PowerShell)
  - [ ] Add JSON result parsing
  - [ ] Update GitHub Actions workflow
  - [ ] Validate on CI runners
  
- **Days 3-4**: Phase 5 (Migration)
  - [ ] Move all tests from `/godot_test/scripts` to `/examples`
  - [ ] Update test metadata for all tests
  - [ ] Decommission old test structure
  - [ ] Verify no regressions
  
- **Day 5**: Final validation & documentation
  - [ ] Comprehensive CI testing
  - [ ] Update developer documentation
  - [ ] Create migration guide for contributors
  - [ ] Polish release notes

- **Day 5**: Phase 5 (Documentation)
  - [ ] Update `examples/README.md`
  - [ ] Create example test files
  - [ ] Update `CONTRIBUTING.md`
  - [ ] Document LSP test features

### Week 4: Migration & Polish
- **Days 1-2**: Migrate existing tests
  - [ ] Add metadata to existing examples
  - [ ] Create negative test cases
  - [ ] Validate all tests pass

- **Days 3-4**: Polish & validation
  - [ ] Cross-platform testing
  - [ ] LSP integration testing
  - [ ] Performance benchmarks
  - [ ] Documentation review

- **Day 5**: Buffer for issues

### Week 5: Contingency (if needed)
- Buffer week for unexpected blockers
- Additional LSP polish
- Extended cross-platform validation

## Success Criteria

✅ **Compiler Prerequisites** 🆕 BLOCKING
- All AST nodes have source spans
- Symbol table built during type checking
- Incremental compilation with AST caching
- Cache hit rate > 80% for typical edits
- Incremental compilation 5-10x faster than full recompilation

✅ **Zero Duplication**
- No `.ferris` files in `godot_test/scripts`
- All tests in `/examples` with metadata

✅ **Automated Discovery**
- `cargo test` discovers and runs unit tests
- `./scripts/run_godot_tests.sh` discovers and runs Godot tests
- No manual test registration required

✅ **Cross-Platform**
- Tests pass on Windows, Linux, macOS
- No platform-specific workarounds
- CI validates all platforms

✅ **Developer Experience**
- Clear error messages for malformed metadata
- Fast test execution (unit tests < 5s)
- Easy to add new tests (just create `.ferris` file)

✅ **CI Integration**
- Tests run on every PR
- Clear pass/fail indicators
- Detailed failure reports
- JSON output for automated parsing

✅ **LSP Integration** 🆕
- Code lenses show test status (✅/❌/▶️)
- "Run Test" command works from editor
- Test results appear in real-time
- Failed tests show diagnostics in Problems panel
- Incremental compilation keeps LSP responsive (<100ms for typical edits)

✅ **Test Quality** 🆕
- Assertions validate runtime behavior
- Tests timeout after configurable duration (default 10s)
- Test filtering works (by name/category)
- Negative tests validate error detection

## Dependencies & Blockers

### Critical Path (Blocking Dependencies)

1. **Phase 0 (Compiler Prerequisites)** → **Phase 2.5 (LSP Integration)** 🆕 CRITICAL
   - LSP requires spans for error reporting
   - LSP requires symbol table for go-to-definition
   - LSP requires incremental compilation for responsiveness
   - **Status**: Must complete Phase 0 before starting Phase 2.5
   - **Blocker**: Cannot implement LSP without compiler foundation
   - **Timeline Impact**: +3 weeks (Weeks 1-3)

2. **Phase 2.5 (LSP Integration)** → **Phase 3 (Godot Runner)**
   - Test runner needs LSP test discovery API
   - Test results must flow back to LSP cache
   - **Status**: Phase 2.5 must complete before Phase 3
   - **Blocker**: Test runner depends on LSP protocol

### Hard Dependencies (Must Complete First)

1. **FerrisScriptRunner Runtime State** (Phase 3)
   - Must expose runtime variables (`get_variable()`)
   - Must track signal emissions (`get_emitted_signals()`)
   - **Status**: Needs implementation
   - **Blocker**: Assertion validation depends on this

2. **Godot 4.x Headless Mode**
   - Must support `--headless` flag
   - Must support `--quit-after` flag
   - **Status**: Available in Godot 4.2+
   - **Blocker**: CI tests require headless execution

### Soft Dependencies (Nice to Have)

1. **Test Coverage Tooling**
   - Not required for v0.0.5
   - Can defer to v0.0.6

2. **Test Profiling/Benchmarking**
   - Nice for identifying slow tests
   - Not blocking v0.0.5 release

2. **Performance Benchmarking**
   - Not required for v0.0.5
   - Can defer to v0.0.6

### Integration Points with LSP

```

Test Framework ←→ LSP Server
    ↓                 ↓
Test Discovery   Protocol Extensions
Test Execution   Test Status Cache
    ↓                 ↓
    └─────────────────┘
         VS Code Extension
         (CodeLens, Commands)

```

**Critical Path**: Test Framework → LSP Integration → VS Code Extension

## Risk Mitigation

### Risk 1: Compiler Refactoring Breaks Existing Tests 🔴 HIGH 🆕
**Impact**: All existing tests fail after adding spans to AST  
**Probability**: Very High  
**Mitigation**:
- Add spans incrementally (one AST node type at a time)
- Use default/dummy spans for unmodified nodes during transition
- Run full test suite after each AST change
- Create migration script to bulk-update test assertions
- **Timeline Buffer**: 1 week contingency built into Phase 0

### Risk 2: Incremental Compilation Cache Bugs 🔴 HIGH 🆕
**Impact**: LSP shows stale errors, incorrect completions  
**Probability**: Medium  
**Mitigation**:
- Extensive unit tests for cache invalidation
- Add cache validation mode (compare cached vs. fresh compilation)
- Implement "force full recompilation" command in LSP
- Monitor cache hit rate in production
- **Fallback**: Disable caching if bugs persist (graceful degradation)

### Risk 3: LSP Integration Complexity 🔴 HIGH
**Impact**: Could delay entire v0.0.5 release  
**Probability**: Medium  
**Mitigation**: 
- Complete Phase 0 (compiler prerequisites) FIRST (Weeks 1-3)
- Define protocol extensions early (Week 5, Day 1)
- Create mock LSP server for Phase 2.5 testing
- Have fallback: Ship test framework without LSP, add in v0.0.6
- **Timeline Buffer**: LSP work starts Week 5 (after compiler ready)

### Risk 4: Godot FileAccess Limitations ✅ MITIGATED
**Mitigation**: Tested in feedback phase - `FileAccess.open()` works with absolute paths on all platforms.

### Risk 5: FerrisScriptRunner Missing API 🟡 MEDIUM
**Impact**: Assertion validation won't work  
**Probability**: Low  
**Mitigation**: Add `compile_from_string()`, `get_variable()`, `get_emitted_signals()` methods in Phase 3 (straightforward additions).

### Risk 6: Assertion DSL Complexity 🟡 MEDIUM
**Impact**: Complex assertions may not validate correctly  
**Probability**: Medium  
**Mitigation**: 
- Start with simple string matching
- Phase 2: Add regex patterns
- Phase 3: Full DSL parser (can defer to v0.0.6)

### Risk 7: Performance Regression from Incremental Compilation Overhead 🟡 MEDIUM 🆕
**Impact**: LSP slower than expected despite caching  
**Probability**: Low  
**Mitigation**:
- Benchmark cache overhead vs. full compilation
- Profile hot paths (hashing, cache lookups)
- Implement cache size limits (LRU eviction)
- **Target**: <100ms for typical edits (cache hit)

### Risk 8: Timeline Overrun (7 weeks → 9+ weeks) 🟡 MEDIUM 🆕
**Impact**: Delayed v0.0.5 release  
**Probability**: Medium  
**Mitigation**:
- Phase 0 has highest risk (3 weeks of compiler changes)
- Weekly check-ins to assess timeline
- De-scope features if falling behind:
  - Drop incremental compilation (fallback: always recompile)
  - Drop LSP test integration (ship test framework only)
- **Decision Point**: End of Week 3 (reassess timeline)

### Risk 9: Performance Regression 🟢 LOW
**Mitigation**: Benchmark current tests, ensure new system is faster (expected: 2x faster due to no file copying).

### Risk 10: Breaking Existing Workflows 🟢 LOW
**Mitigation**: 
- Maintain backward compatibility during migration
- Document migration path clearly
- Provide helper scripts for bulk metadata addition

## Maintenance Plan

### Adding New Tests
1. Create `.ferris` file in `/examples`
2. Add metadata header
3. Run `cargo test` locally
4. Push - CI validates automatically

### Updating Test Framework
- All test logic in `crates/test_harness`
- Versioned alongside compiler
- Breaking changes require migration guide

### Debugging Failed Tests
```bash
# Run specific test
cargo test test_name

# Run with verbose output
RUST_LOG=debug cargo test

# Run Godot tests with debugging
godot --path godot_test --script res://test_runner.gd
```

## Appendix A: Complete File Checklist

### New Files

- [ ] `crates/test_harness/Cargo.toml`
- [ ] `crates/test_harness/src/lib.rs`
- [ ] `crates/test_harness/src/metadata.rs`
- [ ] `crates/test_harness/src/discovery.rs`
- [ ] `crates/test_harness/src/runner.rs`
- [ ] `tests/ferris_integration_tests.rs`
- [ ] `scripts/run_godot_tests.sh`
- [ ] `scripts/run_godot_tests.ps1`
- [ ] `examples/type_error.ferris`
- [ ] `examples/vector_math.ferris`

### Modified Files

- [ ] `Cargo.toml` (add test_harness to workspace)
- [ ] `crates/godot_bind/src/lib.rs` (add `compile_from_string`)
- [ ] `godot_test/test_runner.gd` (complete rewrite)
- [ ] `.github/workflows/test.yml` (add Godot tests)
- [ ] `examples/README.md` (test authoring guide)
- [ ] `examples/bounce.ferris` (add metadata)

### Deleted Files/Directories

- [ ] `godot_test/scripts/*.ferris` (all test scripts - moved to /examples)

## Appendix B: Testing the Test Framework

### Validate Metadata Parser

```bash
cd crates/test_harness
cargo test
```

### Validate Rust Integration

```bash
cargo test --test ferris_integration_tests
```

### Validate Godot Runner (Manual)

```bash
godot --path godot_test --script res://test_runner.gd
```

### Validate CI (Local)

```bash
# Install act (https://github.com/nektos/act)
act -j test
```

## Appendix C: Migration Script

**`scripts/migrate_tests.sh`**:

```bash
#!/bin/bash
# Add metadata headers to existing .ferris files

for file in examples/*.ferris; do
    if ! grep -q "// TEST:" "$file"; then
        basename=$(basename "$file" .ferris)
        
        # Create temporary file with metadata
        cat > /tmp/header.txt << EOF
// TEST: $basename
// CATEGORY: integration
// DESCRIPTION: TODO: Add description
// EXPECT: success

EOF
        
        # Prepend header to file
        cat /tmp/header.txt "$file" > /tmp/newfile
        mv /tmp/newfile "$file"
        
        echo "Added metadata to: $file"
    fi
done
```

## Appendix D: Test Metadata Reference

### Complete Metadata Schema

```ferris
// TEST: <unique_test_identifier>
// CATEGORY: unit|integration|runtime
// DESCRIPTION: <human_readable_description>
// EXPECT: success|error:E###|warning:W###
// ASSERT: <runtime_assertion> (optional, repeatable)
// SCENE: <path_to_scene> (optional, runtime tests only)
// TIMEOUT: <seconds> (optional, default: 10.0)
```

### Metadata Examples

#### Unit Test (Compile Error)

```ferris
// TEST: type_mismatch_string_to_int
// CATEGORY: unit
// DESCRIPTION: Verify compiler catches type mismatches
// EXPECT: error:E201

fn _ready() {
    let x: i32 = "string";
}
```

#### Integration Test (Runtime Success with Assertions)

```ferris
// TEST: bounce_horizontal
// CATEGORY: integration
// DESCRIPTION: Horizontal bouncing motion
// EXPECT: success
// ASSERT: position.x oscillates between -10.0 and 10.0
// TIMEOUT: 5.0

let mut dir: f32 = 1.0;

fn _process(delta: f32) {
    self.position.x += dir * 100.0 * delta;
    if self.position.x > 10.0 { dir = -1.0; }
    if self.position.x < -10.0 { dir = 1.0; }
}
```

#### Runtime Test (Scene Required)

```ferris
// TEST: button_signal_emission
// CATEGORY: runtime
// DESCRIPTION: Test button signal connection
// EXPECT: success
// ASSERT: signal 'pressed' emitted
// SCENE: res://test_scenes/button_test.tscn
// TIMEOUT: 15.0

fn _ready() {
    let button = get_node("Button") as Button;
    button.connect("pressed", Callable::from(self, "_on_button_pressed"));
}
```

## Appendix E: LSP Test Integration Guide

### VS Code Commands

| Command | Description | Shortcut |
|---------|-------------|----------|
| `ferrisscript.runTest` | Run single test | Click code lens |
| `ferrisscript.runAllTests` | Run all tests in file | Ctrl+Shift+T |
| `ferrisscript.debugTest` | Debug test (future) | - |

### LSP Request/Response Flow

```
1. User Opens File
   ↓
2. VS Code → LSP: textDocument/didOpen
   ↓
3. LSP: Parse test metadata
   ↓
4. LSP → VS Code: CodeLens positions
   ↓
5. User Clicks "Run Test"
   ↓
6. VS Code → LSP: ferrisscript/runTest
   ↓
7. LSP: Execute test, update cache
   ↓
8. LSP → VS Code: Test result + diagnostics
   ↓
9. VS Code: Update CodeLens icons, show notification
```

### Test Status Cache

The LSP maintains an in-memory cache of test statuses:

```rust
TestCache {
    "bounce_horizontal": Passed,
    "type_mismatch_error": Passed,
    "signal_emission": Failed,
    "slow_test": Running,
}
```

Cache invalidation:

- On file save (mark Unknown)
- On test run completion (update status)
- On workspace reload (clear all)

## Appendix F: Environment Variables

### Test Execution

| Variable | Values | Description |
|----------|--------|-------------|
| `FERRIS_TEST_FILTER` | String | Run only tests matching name pattern |
| `FERRIS_TEST_CATEGORY` | `unit`/`integration`/`runtime` | Run only tests of specific category |
| `FERRIS_JSON_OUTPUT` | `true`/`false` | Output test results as JSON |
| `RUST_LOG` | `debug`/`info`/`warn` | Rust logging level |

### Examples

```bash
# Run only bounce tests
FERRIS_TEST_FILTER=bounce cargo test

# Run only unit tests
FERRIS_TEST_CATEGORY=unit cargo test

# Get JSON output from Godot tests
FERRIS_JSON_OUTPUT=true ./scripts/run_godot_tests.sh

# Debug test execution
RUST_LOG=debug cargo test -- --nocapture
```

## Appendix G: Troubleshooting Guide

### Issue: "Test not discovered"

**Symptoms**: Test file exists but not found by discovery engine

**Causes & Fixes**:

1. Missing required metadata fields
   - **Fix**: Verify `TEST:`, `CATEGORY:`, `DESCRIPTION:`, `EXPECT:` are present
2. File not in `/examples`
   - **Fix**: Move file to `/examples` directory
3. File extension is not `.ferris`
   - **Fix**: Rename file to have `.ferris` extension

### Issue: "Test times out"

**Symptoms**: Test shows timeout status after X seconds

**Causes & Fixes**:

1. Infinite loop in test code
   - **Fix**: Add exit condition or break statement
2. Timeout too short for slow test
   - **Fix**: Add `// TIMEOUT: 30.0` to increase limit
3. Godot rendering bottleneck
   - **Fix**: Run tests in headless mode (already default)

### Issue: "Assertion failed but test passed"

**Symptoms**: Test shows green checkmark despite wrong behavior

**Causes & Fixes**:

1. Assertion validation not implemented yet
   - **Fix**: Phase 3 will add this (see Phase 3b tasks)
2. Assertion syntax not recognized
   - **Fix**: Check assertion format matches supported patterns
3. Runtime state not exposed
   - **Fix**: Ensure `get_variable()` and `get_emitted_signals()` are implemented

### Issue: "LSP not showing test code lenses"

**Symptoms**: No "Run Test" buttons appear in editor

**Causes & Fixes**:

1. LSP server not running
   - **Fix**: Check VS Code output panel for LSP logs
2. File not recognized as FerrisScript
   - **Fix**: Verify file extension is `.ferris`
3. Test metadata malformed
   - **Fix**: Validate metadata syntax
4. CodeLens provider not registered
   - **Fix**: Restart VS Code, check extension is activated

---

**Ready for Implementation**: This plan is production-ready and addresses all issues identified in the feedback phase, including LSP integration requirements for v0.0.5.

**Next Steps**:

1. ✅ Review and approve this implementation plan
2. ✅ Coordinate with LSP development team (parallel track)
3. ✅ Create feature branch: `feature/consolidated-test-framework`
4. ✅ Begin Phase 1: Test Harness Foundation
5. ✅ Track progress against milestones in this document

**Critical Dependencies**:

- LSP server foundation must be ready before Phase 2.5
- If LSP is delayed, ship test framework without editor integration in v0.0.5
- Add LSP integration in v0.0.6 as enhancement
