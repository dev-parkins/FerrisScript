Excellent question! LSP is **the missing piece** that transforms FerrisScript from "static types for safety" to "static types for **developer experience**". Let me break down how LSP amplifies your type safety philosophy.

## 📝 Decision Log (v0.0.5 Planning)

**Date**: October 13, 2025

### Critical Scope Decisions (All Option A Chosen)

#### Decision 1: Compiler Prerequisites Timing

**Question**: When to implement compiler prerequisites (spans, symbol table)?

**Options**:

- **A**: Ship v0.0.5 with compiler prerequisites (+1 week)
- B: Defer to v0.1.0 (ship test framework in v0.0.5, add compiler later)

**Chosen**: **Option A**

**Rationale**:

- LSP fundamentally requires spans for error reporting
- Symbol table is blocking for any LSP navigation features
- Better to pay the cost once upfront than refactor later
- Establishes solid foundation for v0.1.0 LSP expansion

---

#### Decision 2: LSP Test Integration Scope

**Question**: How much LSP integration in v0.0.5?

**Options**:

- **A**: Full LSP test integration (CodeLens, run test, status cache)
- B: Defer LSP test integration to v0.1.0

**Chosen**: **Option A**

**Rationale**:

- Validates LSP infrastructure works end-to-end
- Provides immediate developer value (run tests from editor)
- Lower risk than full diagnostics (smaller, well-defined scope)
- Tests are self-contained and easier to implement

---

#### Decision 3: Incremental Compilation Timing

**Question**: When to implement incremental compilation?

**Options**:

- **A**: Add incremental compilation to v0.0.5 (+2-3 weeks)
- B: Defer to v0.1.0+ (ship without caching, add later)

**Chosen**: **Option A**

**Rationale**:

- LSP requires fast recompilation (<100ms) for good UX
- Full recompilation on every keystroke is too slow (150ms+)
- Incremental compilation is architectural - hard to bolt on later
- 5-10x speedup is worth the upfront investment

---

### Timeline Impact

**Original Plan**: 3-4 weeks

- Test framework: 2 weeks
- Basic LSP diagnostics: 1-2 weeks

**Updated Plan**: 6-7 weeks

- Compiler prerequisites: +1 week (Decision 1)
- LSP test integration: already planned (Decision 2)
- Incremental compilation: +2-3 weeks (Decision 3)

**Total Impact**: +3-4 weeks

---

### De-Scoping Strategy (If Needed)

**Decision Point**: End of Week 3 (after compiler prerequisites)

**Priority 1 (Must Keep)**:

- Source spans in AST
- Symbol table extraction
- Basic LSP diagnostics
- Test framework foundation

**Priority 2 (Defer to v0.0.6 if behind schedule)**:

- Incremental compilation (fallback: always recompile)
- LSP test integration (ship test framework standalone)

**Priority 3 (Defer to v0.1.0)**:

- Advanced caching strategies
- Dependency graph optimizations

---

## 🎯 LSP: The Type Safety UX Layer

### The Current Gap

**Right now**:

```
FerrisScript Editor → Save → Compile → See Errors
     (VS Code)              (cargo)     (terminal)
```

**With LSP**:

```
FerrisScript Editor → Real-time feedback as you type
     (VS Code)        ← (LSP shows errors, completions, hints)
```

LSP makes your **compile-time type safety visible instantly** while coding.

## 🔥 How LSP Supercharges Type Safety

### 1. **Instant Error Feedback** (Red Squiggles)

**Without LSP** (current):

```rust
// player.ferris
let mut health: i32 = 100;
health = "low";  // Looks fine until you compile
```

Save → Compile → "Error on line 2" → Fix → Repeat

**With LSP**:

```rust
// player.ferris
let mut health: i32 = 100;
health = "low";  // Red squiggle appears instantly
         ~~~~~ 
         E201: Type mismatch - expected i32, found String
```

No compilation needed. Error appears **as you type**.

### 2. **Type-Aware Autocomplete** (IntelliSense)

**Without LSP**:

```rust
fn _process(delta: f32) {
    self.pos|  // You type "pos", hit Ctrl+Space
               // VS Code shows generic completions (position, possible, etc.)
}
```

**With LSP**:

```rust
fn _process(delta: f32) {
    self.pos|  // You type "pos", LSP shows:
               // • position: Vector2  ← Type known from Node2D
               // • (filtered to relevant properties only)
}
```

LSP **knows** `self` is a `Node2D`, so it only shows valid properties.

### 3. **Go-to-Definition** (Navigation)

**Current**:

```rust
// enemy.ferris
let player_health = get_health();  // Where is get_health defined?
                    ~~~~~~~~~~~
                    // Manual search through files
```

**With LSP**:

```rust
// enemy.ferris
let player_health = get_health();  // Ctrl+Click or F12
                    ~~~~~~~~~~~
                    // Jump to: player.ferris, line 23
```

LSP **tracks** function definitions across files.

### 4. **Hover Documentation** (Type Inspection)

**Current**:

```rust
fn take_damage(amount: f32) {
    health -= amount;  // What type is health? Open another file to check
}
```

**With LSP**:

```rust
fn take_damage(amount: f32) {
    health -= amount;  // Hover over "health"
    ~~~~~~
    Tooltip: let mut health: i32 = 100
             Declared in: line 5
             Mutable: yes
}
```

LSP shows **type information** without leaving the editor.

### 5. **Rename Refactoring** (Safe Changes)

**Current**:

```rust
// Rename "health" to "hp"
// 1. Find all files manually
// 2. Replace text (hope you don't miss any)
// 3. Compile to check if you broke anything
```

**With LSP**:

```rust
// Right-click "health" → Rename Symbol
// LSP finds ALL references across ALL files
// Renames safely with preview
// Type checker validates no errors
```

LSP makes **type-safe refactoring** effortless.

### 6. **Parameter Hints** (Function Signatures)

**Current**:

```rust
move_to(100.0, 200.0);  // Which parameter is X? Which is Y?
        ~~~~~ ~~~~~
```

**With LSP**:

```rust
move_to(100.0, 200.0);
        ↓      ↓
        x: f32 y: f32  // Parameter names shown inline
```

LSP shows **function signatures** as you type.

## 🏗️ LSP Architecture for FerrisScript

### High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                        VS Code / Editor                      │
├─────────────────────────────────────────────────────────────┤
│              Language Server Protocol (JSON-RPC)            │
├─────────────────────────────────────────────────────────────┤
│                    FerrisScript LSP Server                   │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Document Manager (tracks open files)                  │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ Incremental Compiler (lexer → parser → type checker) │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ Symbol Index (functions, vars, types)                │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │ Diagnostics Engine (errors, warnings)                │  │
│  └───────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│            Your Existing Compiler (reused!)                  │
│     • lexer.rs                                              │
│     • parser.rs                                             │
│     • type_checker.rs                                       │
│     • ast.rs                                                │
└─────────────────────────────────────────────────────────────┘
```

### Key Components

#### 1. **LSP Server** (`crates/lsp/src/lib.rs`)

```rust
use tower_lsp::*;

#[tower_lsp::async_trait]
impl LanguageServer for FerrisScriptServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL
                )),
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                // ...
            },
            // ...
        })
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // Incrementally recompile on each keystroke
        let uri = params.text_document.uri;
        let changes = params.content_changes;
        
        self.update_document(&uri, changes).await;
        self.publish_diagnostics(&uri).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        // Use type checker to find valid completions at cursor
        let completions = self.get_completions_at_position(uri, position).await;
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // Use symbol index to find definition location
        // ...
    }

    // ... more handlers
}
```

#### 2. **Document Manager** (Tracks Open Files)

```rust
struct DocumentManager {
    // Cache of open documents
    documents: HashMap<Url, Document>,
}

struct Document {
    text: String,
    version: i32,
    ast: Option<ast::Program>,  // Parsed AST
    type_info: Option<TypeInfo>, // Type checker output
    symbols: Vec<Symbol>,        // Function/var definitions
}

impl DocumentManager {
    fn update(&mut self, uri: &Url, changes: Vec<TextDocumentContentChangeEvent>) {
        let doc = self.documents.get_mut(uri).unwrap();
        
        // Apply incremental changes
        for change in changes {
            apply_change(&mut doc.text, change);
        }
        
        // Recompile incrementally
        doc.ast = Some(compile(&doc.text).ok()?);
        doc.type_info = Some(check_types(&doc.ast?)?);
        doc.symbols = extract_symbols(&doc.ast?);
    }
}
```

#### 3. **Symbol Index** (Find Definitions)

```rust
struct SymbolIndex {
    // Map symbol name → location
    symbols: HashMap<String, Vec<Location>>,
}

impl SymbolIndex {
    fn find_definition(&self, symbol: &str) -> Option<Location> {
        self.symbols.get(symbol)?.first().cloned()
    }
    
    fn find_references(&self, symbol: &str) -> Vec<Location> {
        // Find all uses of symbol across files
        // ...
    }
}

#[derive(Debug, Clone)]
struct Location {
    uri: Url,
    range: Range,  // Line/column span
}
```

#### 4. **Diagnostics Engine** (Show Errors)

```rust
async fn publish_diagnostics(&self, uri: &Url) {
    let doc = self.documents.get(uri).unwrap();
    
    let diagnostics = match &doc.type_info {
        Some(type_info) => {
            type_info.errors.iter().map(|err| Diagnostic {
                range: err.span.to_lsp_range(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String(err.code.clone())),
                message: err.message.clone(),
                // ...
            }).collect()
        }
        None => vec![],
    };
    
    self.client.publish_diagnostics(uri.clone(), diagnostics, None).await;
}
```

## 🎯 LSP Features Priority for FerrisScript

### Phase 0: Test-Specific LSP (v0.0.5) 🆕 UPDATED

**Goal**: LSP integration for test framework

**Features**:

- ✅ Test discovery via LSP (`ferrisscript/documentTests`)
- ✅ Run single test via CodeLens ("Run Test" button)
- ✅ Test status indicators (✅/❌/⏱️)
- ✅ Test result caching
- ✅ Real-time test status updates

**Implementation**: ~1-2 weeks (Week 5 of v0.0.5)

**Scope**: LSP features are **test-specific only** in v0.0.5. General LSP features (diagnostics for all code, autocomplete, etc.) come in v0.1.0.

**Why Test-Specific First?**:

- Validates LSP infrastructure works end-to-end
- Provides immediate developer value (run tests from editor)
- Lower risk than full diagnostics (smaller scope)
- Tests are well-defined and self-contained

---

### Phase 1: Core Diagnostics (v0.1.0) - UPDATED

**Goal**: Red squiggles for type errors in all code

**Features**:

- ✅ Real-time compilation on file changes (using Phase 0.3 incremental compiler)
- ✅ Publish diagnostics (errors/warnings)
- ✅ Error code links (E201 → docs)
- ✅ Performance: <100ms for typical edits (cache hit)

**Implementation**: ~2 weeks

**Dependencies**: Requires v0.0.5 compiler prerequisites (spans, symbol table, incremental compilation)

**Note**: This expands the test-specific diagnostics from v0.0.5 to all FerrisScript code.

---

### Phase 2: Navigation (v0.1.1)

**Goal**: Jump to definitions

**Features**:

- ✅ Go-to-definition (Ctrl+Click) - uses Phase 0.2 symbol table
- ✅ Find references (Shift+F12) - uses Phase 0.2 symbol table
- ✅ Hover tooltips (type information) - uses Phase 0.1 spans + Phase 0.2 symbol table

**Implementation**: ~2 weeks

**Dependencies**: v0.0.5 Phase 0.1 (spans) and Phase 0.2 (symbol table)

---

### Phase 3: Refactoring (v0.1.2)

**Goal**: Safe symbol renaming

**Features**:

- ✅ Rename symbol (F2)
- ✅ Document symbols (Ctrl+Shift+O)
- ✅ Workspace symbols (Ctrl+T)

**Implementation**: ~1 week

**Dependencies**: v0.1.1 (navigation, symbol index)

---

### Phase 4: IntelliSense (v0.1.3)

**Goal**: Smart autocomplete

**Features**:

- ✅ Context-aware completions (uses Phase 0.2 symbol table)
- ✅ Parameter hints (Ctrl+Shift+Space)
- ✅ Signature help

**Implementation**: ~2 weeks

**Dependencies**: v0.1.1 (navigation), v0.0.5 Phase 0.2 (symbol table)

---

### Phase 5: Advanced (v0.2.0+)

**Features**:

- ✅ Code actions (quick fixes)
- ✅ Inlay hints (inline type annotations)
- ✅ Semantic highlighting
- ✅ Code folding

**Dependencies**: v0.1.3 (full IntelliSense)

## � Incremental Compilation Architecture (v0.0.5 Phase 0.3)

### Why Incremental Compilation is Critical for LSP

**Problem**: Full recompilation on every keystroke is too slow for LSP.

**Example**:

```rust
// player.ferris (500 lines)
let mut health: i32 = 100;
health = "low";  // User types this
                 // ← LSP needs to show error in <100ms
```

**Without Incremental Compilation**:

- Parse entire 500-line file (~50ms)
- Type-check entire file (~100ms)
- **Total**: 150ms+ per keystroke ❌

**With Incremental Compilation**:

- Hash unchanged regions (cached) (~5ms)
- Parse only changed function (~10ms)
- Type-check only affected scope (~20ms)
- **Total**: 35ms per keystroke ✅

---

### Implementation Strategy

#### 1. **AST Caching**

```rust
// crates/compiler/src/incremental.rs
use std::collections::HashMap;
use std::time::SystemTime;

pub struct IncrementalCompiler {
    /// Cache compiled AST nodes by source hash
    ast_cache: HashMap<String, CachedAst>,
    
    /// Track dependencies between files
    dependency_graph: DependencyGraph,
}

#[derive(Clone)]
struct CachedAst {
    ast: ast::Program,
    source_hash: u64,
    timestamp: SystemTime,
}

impl IncrementalCompiler {
    pub fn new() -> Self {
        Self {
            ast_cache: HashMap::new(),
            dependency_graph: DependencyGraph::new(),
        }
    }
    
    /// Compile a file, using cache when possible
    pub fn compile_file(&mut self, uri: &str, source: &str) 
        -> Result<CompilationResult, CompileError> 
    {
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
    
    fn compile_fresh(&mut self, uri: &str, source: &str) 
        -> Result<CompilationResult, CompileError> 
    {
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
        
        Ok(CompilationResult {
            program,
            symbol_table,
            diagnostics: vec![],
        })
    }
    
    fn recheck_cached(&self, uri: &str, cached: &CachedAst) 
        -> Result<CompilationResult, CompileError> 
    {
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
        
        // Invalidate files that depend on this one
        for dependent in self.dependency_graph.dependents_of(uri) {
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

---

#### 2. **Dependency Tracking**

```rust
// crates/compiler/src/dependency_graph.rs
use std::collections::{HashMap, HashSet};

pub struct DependencyGraph {
    /// Map file URI → files it depends on
    dependencies: HashMap<String, HashSet<String>>,
    
    /// Map file URI → files that depend on it (reverse index)
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
    
    pub fn remove_file(&mut self, uri: &str) {
        // Remove from both indices
        self.dependencies.remove(uri);
        self.reverse_deps.remove(uri);
        
        // Remove from other files' dependency lists
        for deps in self.dependencies.values_mut() {
            deps.remove(uri);
        }
        
        for deps in self.reverse_deps.values_mut() {
            deps.remove(uri);
        }
    }
}
```

---

#### 3. **LSP Integration**

```rust
// crates/lsp/src/server.rs
use tower_lsp::lsp_types::*;
use ferrisscript_compiler::IncrementalCompiler;

pub struct DocumentManager {
    compiler: IncrementalCompiler,
    documents: HashMap<Url, Document>,
}

struct Document {
    text: String,
    version: i32,
    diagnostics: Vec<Diagnostic>,
    symbol_table: Option<SymbolTable>,
}

impl DocumentManager {
    pub fn new() -> Self {
        Self {
            compiler: IncrementalCompiler::new(),
            documents: HashMap::new(),
        }
    }
    
    pub async fn did_open(&mut self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        
        // Initial compilation
        self.compile_document(&uri, &text).await;
    }
    
    pub async fn did_change(&mut self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let changes = params.content_changes;
        
        // Apply changes to document
        let doc = self.documents.get_mut(&uri).unwrap();
        for change in changes {
            apply_change(&mut doc.text, change);
        }
        
        // Recompile incrementally
        self.compile_document(&uri, &doc.text).await;
    }
    
    async fn compile_document(&mut self, uri: &Url, text: &str) {
        let result = self.compiler.compile_file(&uri.to_string(), text);
        
        let doc = self.documents.entry(uri.clone()).or_insert_with(|| Document {
            text: text.to_string(),
            version: 0,
            diagnostics: vec![],
            symbol_table: None,
        });
        
        match result {
            Ok(compilation) => {
                doc.diagnostics = compilation.diagnostics;
                doc.symbol_table = Some(compilation.symbol_table);
            }
            Err(e) => {
                // Show compile error
                doc.diagnostics = vec![Diagnostic {
                    range: e.span.to_lsp_range(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: e.message,
                    ..Default::default()
                }];
            }
        }
    }
}

fn apply_change(text: &mut String, change: TextDocumentContentChangeEvent) {
    if let Some(range) = change.range {
        // Incremental change
        let start = position_to_offset(text, range.start);
        let end = position_to_offset(text, range.end);
        text.replace_range(start..end, &change.text);
    } else {
        // Full document change
        *text = change.text;
    }
}

fn position_to_offset(text: &str, position: Position) -> usize {
    let mut offset = 0;
    for (line_num, line) in text.lines().enumerate() {
        if line_num < position.line as usize {
            offset += line.len() + 1; // +1 for newline
        } else {
            offset += position.character as usize;
            break;
        }
    }
    offset
}
```

---

### Performance Targets

| Scenario | Target Latency | Acceptable Latency | Cache Hit Rate |
|----------|----------------|-------------------|----------------|
| Single-line edit | <50ms | <100ms | >95% |
| Function-level change | <100ms | <200ms | >80% |
| Multi-file refactor | <500ms | <1000ms | >60% |
| Full workspace recompile | <2000ms | <5000ms | N/A |

---

### Cache Invalidation Strategy

**When to invalidate**:

1. **File content changes** (hash mismatch)
   - Hash source on every edit
   - Compare with cached hash
   - Invalidate if different

2. **Imported file changes** (dependency invalidation)
   - Track import statements in AST
   - Build dependency graph
   - Invalidate transitively when upstream changes

3. **Type definitions change** (transitive invalidation)
   - If function signature changes, invalidate callers
   - If struct fields change, invalidate accessors
   - Uses symbol table from Phase 0.2

4. **Compiler version changes** (force full recompile)
   - Store compiler version in cache
   - Invalidate all caches on version mismatch

**Cache size limits**:

- **Memory cache**: Max 100MB (≈200 cached ASTs for typical files)
- **Disk cache**: Max 1GB (for persistent caching across restarts)
- **Eviction policy**: LRU (Least Recently Used)

---

### Metrics & Monitoring

```rust
// crates/compiler/src/incremental.rs
pub struct CacheMetrics {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub total_compilations: usize,
    pub avg_hit_latency: Duration,
    pub avg_miss_latency: Duration,
}

impl IncrementalCompiler {
    pub fn metrics(&self) -> CacheMetrics {
        // Track compilation statistics
        CacheMetrics {
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            total_compilations: self.cache_hits + self.cache_misses,
            avg_hit_latency: self.hit_latencies.iter().sum::<Duration>() 
                / self.cache_hits.max(1),
            avg_miss_latency: self.miss_latencies.iter().sum::<Duration>() 
                / self.cache_misses.max(1),
        }
    }
    
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}
```

**Success Criteria**:

- Cache hit rate >80% for typical editing sessions
- <100ms latency for cache hits
- 5-10x speedup vs. full recompilation

---

## �🚀 Why LSP is Critical for FerrisScript's Philosophy

### 1. **Closes the Feedback Loop**

```
Without LSP:  Write → Save → Compile → Read Error → Fix  (30+ seconds)
With LSP:     Write → See Error Instantly → Fix           (<1 second)
```

LSP makes **type safety feel instant**, not batched.

### 2. **Makes Types Visible**

Types are **hidden** in the code without LSP:

```rust
fn calculate(a, b) -> ? {  // What types are these?
    return a + b;
}
```

LSP makes types **explicit** via hover/hints:

```rust
fn calculate(a: f32, b: f32) -> f32 {
    //         ↑ Hover: "a: f32"
    return a + b;
}
```

### 3. **Enables "Type-Driven Development"**

With LSP, you can:

1. Write function signature first
2. Let LSP show required types
3. Fill in implementation guided by type errors
4. Types guide your coding flow

**Example**:

```rust
// Step 1: Write signature
fn move_player(direction: Vector2) {
    // Step 2: Start typing, LSP suggests:
    self.position += direction;  // LSP knows += works on Vector2
}
```

### 4. **Marketing Advantage**

**GDScript's advantage**: "Great IDE support built-in"

**FerrisScript's answer with LSP**: "Better IDE support **because of** static types"

| Feature | FerrisScript + LSP | GDScript |
|---------|-------------------|----------|
| Error Detection | Instant (type-aware) ✅ | After running ⚠️ |
| Autocomplete | Type-filtered ✅ | Generic ⚠️ |
| Refactoring | Safe (type-checked) ✅ | Text-based ⚠️ |
| Navigation | Precise (symbol-based) ✅ | Search-based ⚠️ |

## 📋 Recommended Roadmap

### v0.0.5 (Current Release) - UPDATED

**Focus**: LSP Foundation + Test Framework + Incremental Compilation

**Timeline**: 6-7 weeks (was 2-3 weeks)

**Critical Decisions** (Option A for all):

1. ✅ Ship v0.0.5 with compiler prerequisites (spans + symbol table) - adds 1 week
2. ✅ Full LSP test integration in v0.0.5 - no deferral
3. ✅ Add incremental compilation to v0.0.5 - adds 2-3 weeks

#### Phase 0: Compiler Prerequisites (Weeks 1-3) 🆕 BLOCKING

**Phase 0.1: Source Spans (Week 1)**

- [ ] Add `Span` and `Position` structs to all AST nodes
- [ ] Update parser to track spans from tokens
- [ ] Update all tests with span assertions
- [ ] Add span information to error messages
- **Deliverable**: All AST nodes have accurate source location tracking

**Phase 0.2: Symbol Table (Week 2)**

- [ ] Extract symbol table from type checker
- [ ] Implement `SymbolTable` with scope chain lookups
- [ ] Update `compile()` to return `SymbolTable`
- [ ] Add integration tests for symbol resolution
- **Deliverable**: Public API for LSP (go-to-definition, autocomplete foundation)

**Phase 0.3: Incremental Compilation (Week 3)**

- [ ] Implement `IncrementalCompiler` with AST caching
- [ ] Add source hash-based cache invalidation
- [ ] Create `DependencyGraph` for transitive invalidation
- [ ] Performance benchmarks (target: 5-10x speedup)
- **Deliverable**: <100ms compilation for typical edits (cache hit)

#### Phase 1: Test Framework Foundation (Week 4)

- [ ] Create `crates/test_harness` with metadata parsing
- [ ] Test discovery engine (scan `/examples`)
- [ ] Rust test harness with filtering support
- [ ] Cross-platform validation (Windows/Linux/macOS)
- **Deliverable**: Consolidated test framework (single source of truth)

#### Phase 2: LSP Server Foundation (Week 5a)

- [ ] Create `ferrisscript_lsp` crate with tower-lsp
- [ ] Document manager with incremental compiler integration
- [ ] Basic LSP protocol (initialize, shutdown, capabilities)
- [ ] Text document synchronization (incremental updates)
- **Deliverable**: Working LSP server with document tracking

#### Phase 2.5: LSP Test Integration (Week 5b) 🆕

- [ ] Custom LSP protocol (`ferrisscript/documentTests`, `ferrisscript/runTest`)
- [ ] Test discovery API in LSP
- [ ] Test result cache for status tracking
- [ ] VS Code CodeLens provider ("Run Test" buttons)
- **Deliverable**: Real-time test status in editor (✅/❌/⏱️)

#### Phase 3: Real-Time Diagnostics (Week 6a)

- [ ] Integrate compiler with LSP (using Phase 0 infrastructure)
- [ ] Incremental compilation on each keystroke
- [ ] Publish diagnostics (errors, warnings) to client
- [ ] Error recovery for partial syntax
- **Deliverable**: Red squiggles for type errors (<100ms latency)

#### Phase 4: Godot Test Runner (Week 6b)

- [ ] Implement `test_runner.gd` with assertion validation
- [ ] Timeout mechanism and signal monitoring
- [ ] JSON output for CI integration
- [ ] Cross-platform filesystem access
- **Deliverable**: Headless Godot test execution

#### Phase 5: CI Integration & Migration (Week 7)

- [ ] GitHub Actions workflow updates
- [ ] JSON result parsing
- [ ] Migrate tests from `/godot_test/scripts` to `/examples`
- [ ] Decommission old test structure
- **Deliverable**: Fully automated CI with test framework

**Estimated Premium Requests**: 20-25 (was 11-16)

**Decision Rationale**:

- Chose Option A: Ship v0.0.5 with full compiler prerequisites
- Chose Option A: Full LSP test integration (not deferred)
- Chose Option A: Incremental compilation in v0.0.5 (not v0.1.0+)

---

### v0.1.0 (LSP Core Features) - UPDATED

**Focus**: Expand LSP beyond test integration to general coding features

**Timeline**: 3-4 weeks

**Features**:

- [ ] Real-time diagnostics for all code (not just tests)
- [ ] Hover tooltips (type information, documentation)
- [ ] Document symbols (outline view, Ctrl+Shift+O)
- [ ] Basic autocomplete (context-aware completions)

**Note**: v0.0.5 already includes LSP test integration + compiler prerequisites, so v0.1.0 expands LSP to general coding features.

**Dependencies**: v0.0.5 compiler prerequisites (spans, symbol table, incremental compilation)

---

### v0.1.x (LSP Navigation & Refactoring)

**Focus**: Advanced navigation and safe refactoring

**Timeline**: 4-6 weeks

**Features**:

- [ ] Go-to-definition (Ctrl+Click, F12)
- [ ] Find references (Shift+F12)
- [ ] Rename symbol (F2)
- [ ] Workspace symbols (Ctrl+T)

**Dependencies**: v0.1.0 (hover, autocomplete, symbols)

## 🎯 TL;DR: LSP Makes Type Safety **Feel Good**

**Without LSP**: Type safety is a **compile-time batch process**

- Write code → Save → Compile → Read errors → Fix → Repeat

**With LSP**: Type safety is a **real-time conversation**

- Type character → See error → Fix immediately → Move on

**LSP transforms**:

- ❌ "Type errors are annoying compiler messages"
- ✅ "Type errors are helpful real-time guidance"

**For FerrisScript specifically**:

- Your static types are **already the foundation** for great LSP
- LSP makes your type safety **visible and interactive**
- This is where FerrisScript **surpasses GDScript** in DX

---

## 📊 v0.0.5 Implementation Summary

**Timeline**: 6-7 weeks (was 2-3 weeks)

**Scope** (Option A decisions):

- ✅ Compiler prerequisites (spans, symbol table, incremental compilation) - Weeks 1-3
- ✅ Test framework with LSP integration - Weeks 4-5
- ✅ Godot test runner and CI - Weeks 6-7

**v0.0.5 Delivers**:

- LSP test integration (CodeLens, run test, status indicators)
- Consolidated test framework (single source of truth in `/examples`)
- Incremental compilation foundation (<100ms for typical edits)
- Red squiggles for type errors (test-specific diagnostics)

**v0.1.0 Expands**:

- General LSP diagnostics (all code, not just tests)
- Autocomplete and navigation
- Hover tooltips and refactoring tools

**Decision**: We chose Option A for all three decisions, accelerating LSP foundation into v0.0.5 to establish compiler infrastructure early.

**See Also**:

- `docs/planning/v0.0.5/CONSOLIDATED_TEST_FRAMEWORK_IMPLEMENTATION.md` - Detailed implementation guide
- `docs/planning/ROADMAP_MASTER.md` - Version roadmap and consistency checklist
