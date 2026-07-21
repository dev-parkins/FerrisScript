# Struct Literal Syntax - Implementation Analysis

**Date**: October 10, 2025  
**Status**: Research & Planning  
**Purpose**: Enable struct literal construction for Phase 4 types (Color, Rect2, Transform2D)

---

## 🎯 Executive Summary

**Problem**: 30 Phase 4 tests commented out due to missing struct literal syntax support.  
**Solution**: Implement parser, type checker, and runtime support for struct literals.  
**Complexity**: MEDIUM - Parser extension + validation logic, no runtime changes needed.  
**Estimated Effort**: **4-6 hours** (focused session)

---

## 📊 Current State Assessment

### ✅ What Exists (Phase 4 Complete)

1. **Type System**: Color, Rect2, Transform2D in Type enum
2. **Field Access**: Validation for r/g/b/a, position/size, position/rotation/scale
3. **Runtime Support**: Value enum with Color, Rect2, Transform2D variants
4. **Field Execution**: Runtime field get/set implemented
5. **Error Codes**: E701-E710 reserved and defined

### ❌ What's Missing (Struct Literal Syntax)

1. **Lexer**: No new tokens needed (uses existing `{`, `}`, `:`, `,`, identifiers)
2. **Parser**: No struct literal expression parsing
3. **AST**: No struct literal node type
4. **Type Checker**: No struct construction validation
5. **Runtime**: No struct literal evaluation (but Value construction exists)

---

## 🏗️ Implementation Requirements

### 1. AST Extension (ast.rs)

**Add new expression variant**:

```rust
pub enum Expr {
    // ... existing variants ...
    StructLiteral {
        type_name: String,
        fields: Vec<(String, Expr)>, // field_name: value_expr
        span: Span,
    },
}
```

**Effort**: 30 minutes  
**Risk**: LOW - Simple enum addition  
**Dependencies**: None

---

### 2. Parser Extension (parser.rs)

**Add struct literal parsing**:

```rust
fn parse_struct_literal(&mut self, type_name: String, span: Span) -> Result<Expr, String> {
    // Expect: TypeName { field1: expr1, field2: expr2 }
    // Already consumed TypeName, now expect '{'
    self.expect(Token::LBrace)?;
    
    let mut fields = Vec::new();
    loop {
        if self.peek() == Token::RBrace { break; }
        
        let field_name = self.expect_identifier()?;
        self.expect(Token::Colon)?;
        let value = self.parse_expression()?;
        fields.push((field_name, value));
        
        if self.peek() == Token::Comma {
            self.advance();
        } else {
            break;
        }
    }
    self.expect(Token::RBrace)?;
    
    Ok(Expr::StructLiteral { type_name, fields, span })
}
```

**Call site**: In `parse_primary()` after parsing an identifier:

```rust
fn parse_primary(&mut self) -> Result<Expr, String> {
    match self.peek() {
        Token::Identifier(name) => {
            let span = self.current_span();
            let name_str = name.clone();
            self.advance();
            
            // Check for struct literal: Identifier '{'
            if self.peek() == Token::LBrace {
                return self.parse_struct_literal(name_str, span);
            }
            
            // Check for function call: Identifier '('
            if self.peek() == Token::LParen {
                return self.parse_function_call(name_str, span);
            }
            
            // Otherwise it's a variable
            Ok(Expr::Variable(name_str, span))
        }
        // ... rest of parsing ...
    }
}
```

**Effort**: 2 hours  
**Risk**: MEDIUM - Parsing logic needs careful error handling  
**Dependencies**: AST extension  
**Edge Cases**:

- Nested struct literals: `Rect2 { position: Vector2 { x: 0.0, y: 0.0 }, ... }`
- Missing comma between fields
- Missing field values
- Extra commas (trailing comma support)

---

### 3. Type Checker Extension (type_checker.rs)

**Add struct literal validation** in `infer_type()`:

```rust
fn infer_type(&mut self, expr: &Expr, expected: Option<Type>) -> Result<Type, String> {
    match expr {
        // ... existing cases ...
        
        Expr::StructLiteral { type_name, fields, span } => {
            self.validate_struct_literal(type_name, fields, *span)
        }
    }
}

fn validate_struct_literal(
    &mut self,
    type_name: &str,
    fields: &[(String, Expr)],
    span: Span,
) -> Result<Type, String> {
    // Parse type from string
    let struct_type = Type::from_string(type_name).ok_or_else(|| {
        self.error(ErrorCode::E704, // Invalid Color construction
            format!("Unknown type '{}'", type_name), span)
    })?;
    
    match struct_type {
        Type::Color => validate_color_literal(self, fields, span),
        Type::Rect2 => validate_rect2_literal(self, fields, span),
        Type::Transform2D => validate_transform2d_literal(self, fields, span),
        _ => Err(self.error(ErrorCode::E704,
            format!("Type '{}' does not support struct literal syntax", type_name), span))
    }
}

fn validate_color_literal(
    checker: &mut TypeChecker,
    fields: &[(String, Expr)],
    span: Span,
) -> Result<Type, String> {
    let required_fields = ["r", "g", "b", "a"];
    
    // Check all required fields present
    for req in &required_fields {
        if !fields.iter().any(|(name, _)| name == req) {
            return Err(checker.error(ErrorCode::E704,
                format!("Missing required field '{}' in Color literal", req), span));
        }
    }
    
    // Check no unknown fields
    for (field_name, _) in fields {
        if !required_fields.contains(&field_name.as_str()) {
            return Err(checker.error(ErrorCode::E701,
                format!("Unknown field '{}' on Color", field_name), span));
        }
    }
    
    // Validate field types
    for (field_name, field_expr) in fields {
        let field_type = checker.infer_type(field_expr, Some(Type::Float))?;
        if field_type != Type::Float && field_type != Type::Int {
            return Err(checker.error(ErrorCode::E707,
                format!("Color field '{}' must be f32, found {:?}", field_name, field_type), 
                field_expr.span()));
        }
    }
    
    Ok(Type::Color)
}

// Similar for validate_rect2_literal and validate_transform2d_literal
```

**Effort**: 2-3 hours  
**Risk**: MEDIUM - Complex validation matrix  
**Dependencies**: Parser extension  
**Tests Needed**:

- Valid literals for all 3 types
- Missing required fields
- Unknown fields
- Wrong field types
- Nested literals (Rect2 with Vector2)

---

### 4. Runtime Extension (runtime/src/lib.rs)

**Add struct literal evaluation** in `eval_expr()`:

```rust
fn eval_expr(&mut self, expr: &Expr, env: &mut Env) -> Result<Value, String> {
    match expr {
        // ... existing cases ...
        
        Expr::StructLiteral { type_name, fields, span } => {
            self.eval_struct_literal(type_name, fields, *span, env)
        }
    }
}

fn eval_struct_literal(
    &mut self,
    type_name: &str,
    fields: &[(String, Expr)],
    span: Span,
    env: &mut Env,
) -> Result<Value, String> {
    match type_name {
        "Color" => {
            let mut r = None;
            let mut g = None;
            let mut b = None;
            let mut a = None;
            
            for (field_name, field_expr) in fields {
                let value = self.eval_expr(field_expr, env)?;
                let float_val = value.to_float().ok_or_else(|| {
                    format!("Color field '{}' must be numeric at {}", field_name, span)
                })?;
                
                match field_name.as_str() {
                    "r" => r = Some(float_val),
                    "g" => g = Some(float_val),
                    "b" => b = Some(float_val),
                    "a" => a = Some(float_val),
                    _ => return Err(format!("Unknown field '{}' at {}", field_name, span)),
                }
            }
            
            Ok(Value::Color {
                r: r.ok_or("Missing field 'r'")?,
                g: g.ok_or("Missing field 'g'")?,
                b: b.ok_or("Missing field 'b'")?,
                a: a.ok_or("Missing field 'a'")?,
            })
        }
        
        "Rect2" => {
            // Similar pattern
            let mut position = None;
            let mut size = None;
            
            for (field_name, field_expr) in fields {
                let value = self.eval_expr(field_expr, env)?;
                match field_name.as_str() {
                    "position" => {
                        if let Value::Vector2 { .. } = value {
                            position = Some(Box::new(value));
                        } else {
                            return Err(format!("Rect2 'position' must be Vector2 at {}", span));
                        }
                    }
                    "size" => {
                        if let Value::Vector2 { .. } = value {
                            size = Some(Box::new(value));
                        } else {
                            return Err(format!("Rect2 'size' must be Vector2 at {}", span));
                        }
                    }
                    _ => return Err(format!("Unknown field '{}' at {}", field_name, span)),
                }
            }
            
            Ok(Value::Rect2 {
                position: position.ok_or("Missing field 'position'")?,
                size: size.ok_or("Missing field 'size'")?,
            })
        }
        
        "Transform2D" => {
            // Similar pattern with mixed types
            // ...
        }
        
        _ => Err(format!("Unknown struct type '{}' at {}", type_name, span)),
    }
}
```

**Effort**: 2 hours  
**Risk**: LOW - Straightforward value construction  
**Dependencies**: Type checker validation ensures correctness

---

## 🧪 Testing Requirements

### Test Coverage Needed

**Type Checker Tests** (restore 30 commented tests):

```rust
// Color tests (8)
#[test]
fn test_color_literal_valid() {
    let input = "fn test() { let c = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }; }";
    assert!(check_ok(input));
}

#[test]
fn test_color_literal_missing_field() {
    let input = "fn test() { let c = Color { r: 1.0, g: 0.5, b: 0.0 }; }"; // missing 'a'
    assert!(check_err(input).contains("E704") || check_err(input).contains("Missing"));
}

#[test]
fn test_color_literal_unknown_field() {
    let input = "fn test() { let c = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0, x: 2.0 }; }";
    assert!(check_err(input).contains("E701"));
}

#[test]
fn test_color_literal_wrong_type() {
    let input = r#"fn test() { let c = Color { r: "red", g: 0.5, b: 0.0, a: 1.0 }; }"#;
    assert!(check_err(input).contains("E707") || check_err(input).contains("type"));
}

// Rect2 tests (10)
#[test]
fn test_rect2_literal_nested_vector2() {
    let input = "fn test() { let r = Rect2 { position: Vector2 { x: 0.0, y: 0.0 }, size: Vector2 { x: 100.0, y: 50.0 } }; }";
    assert!(check_ok(input));
}

// Transform2D tests (12)
#[test]
fn test_transform2d_literal_mixed_types() {
    let input = "fn test() { let t = Transform2D { position: Vector2 { x: 100.0, y: 200.0 }, rotation: 1.57, scale: Vector2 { x: 2.0, y: 2.0 } }; }";
    assert!(check_ok(input));
}
```

**Runtime Tests** (integration):

```rust
#[test]
fn test_color_literal_runtime() {
    let program = compile("fn test() -> f32 { let c = Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 }; return c.r; }").unwrap();
    let mut env = Env::new();
    execute(&program, &mut env).unwrap();
    let result = call_function("test", &[], &mut env).unwrap();
    assert_eq!(result, Value::Float(1.0));
}
```

**Effort**: 1 hour (tests already written, just uncomment and verify)

---

## 📈 Effort Breakdown

| Component | Task | Effort | Risk | Priority |
|-----------|------|--------|------|----------|
| AST | Add StructLiteral variant | 30 min | LOW | P0 |
| Parser | Add struct literal parsing | 2 hrs | MEDIUM | P0 |
| Type Checker | Add validation logic | 2-3 hrs | MEDIUM | P0 |
| Runtime | Add evaluation logic | 2 hrs | LOW | P0 |
| Tests | Uncomment & verify tests | 1 hr | LOW | P0 |
| **TOTAL** | **End-to-end implementation** | **7.5-8.5 hrs** | **MEDIUM** | **P0** |

**Realistic Estimate with Buffer**: **4-6 hours in focused session**

---

## 🎯 Quick Win Opportunities

### Option A: Minimal Viable Implementation (2-3 hours)

**Scope**: Implement just enough to enable 10-15 key tests

**Approach**:

1. Add AST node (30 min)
2. Add basic parser (no nested literals) (1 hr)
3. Add type checker validation (basic, no complex nested cases) (1 hr)
4. Add runtime evaluation (basic) (30 min)
5. Verify 10-15 tests pass (30 min)

**Benefits**:

- Quick validation of approach
- Demonstrates feasibility
- Enables core functionality

**Limitations**:

- May not handle nested literals (Rect2 with Vector2)
- May not catch all edge cases
- Some tests may remain commented

---

### Option B: Full Implementation (4-6 hours)

**Scope**: Complete struct literal support for all 3 types

**Approach**:

1. Follow full implementation plan above
2. Handle all edge cases (nested literals, error recovery)
3. Enable all 30 tests
4. Comprehensive validation

**Benefits**:

- Complete feature
- All tests enabled
- Robust error handling
- Ready for production

**Limitations**:

- Requires dedicated session
- More complex testing

---

## 💡 Recommended Approach

### Phase 1: Quick Win (Next Session - 2-3 hours)

**Goal**: Enable basic struct literals without nested construction

**Implementation**:

1. ✅ Add AST StructLiteral node
2. ✅ Add parser support (basic, no nested Vector2 in Rect2/Transform2D)
3. ✅ Add type checker validation (all fields, types)
4. ✅ Add runtime evaluation
5. ✅ Enable 8 Color tests + 5 simple Rect2/Transform2D tests

**Tests Enabled**: ~15-20 tests

**Workaround for nested**:

```rust
// Instead of:
let r = Rect2 { position: Vector2 { x: 0.0, y: 0.0 }, size: Vector2 { x: 100.0, y: 50.0 } };

// Use two-step construction:
fn test(pos: Vector2, sz: Vector2) {
    let r = Rect2 { position: pos, size: sz };
}
```

---

### Phase 2: Full Support (Follow-up Session - 2-3 hours)

**Goal**: Add nested struct literal support

**Implementation**:

1. ✅ Extend parser for nested struct literals
2. ✅ Update type checker to handle recursive validation
3. ✅ Update runtime for nested evaluation
4. ✅ Enable remaining 10-15 tests

**Tests Enabled**: All 30 tests

---

## 🚨 Risk Assessment

### LOW RISK ✅

- **AST Addition**: Simple enum extension
- **Runtime Evaluation**: Straightforward value construction
- **Basic Parsing**: Standard recursive descent pattern

### MEDIUM RISK ⚠️

- **Parser Complexity**: Nested struct literals add recursion
- **Type Checking**: Validation matrix has many edge cases
- **Error Messages**: Need clear diagnostics for each failure mode

### HIGH RISK ❌

- **None identified** - This is a well-scoped feature with clear boundaries

---

## 📝 Implementation Checklist

### Pre-Implementation

- [ ] Read existing Vector2 usage patterns for reference
- [ ] Review parser recovery mechanisms
- [ ] Review error code E704-E710 descriptions

### Implementation

- [ ] Add StructLiteral to Expr enum in ast.rs
- [ ] Update Display impl for new AST node
- [ ] Add parse_struct_literal() to parser.rs
- [ ] Update parse_primary() to detect struct literals
- [ ] Add validate_struct_literal() to type_checker.rs
- [ ] Add validate_color_literal() helper
- [ ] Add validate_rect2_literal() helper
- [ ] Add validate_transform2d_literal() helper
- [ ] Add eval_struct_literal() to runtime
- [ ] Uncomment Phase 4 tests
- [ ] Run cargo test --all
- [ ] Fix any failing tests

### Validation

- [ ] All 30 Phase 4 tests passing
- [ ] Error messages are clear
- [ ] Nested literals work (Rect2 with Vector2)
- [ ] Type coercion works (int → float in Color fields)
- [ ] Parser recovery works on syntax errors

---

## 🎓 Lessons from Phase 4

### What Worked Well ✅

1. **Incremental Testing**: Field access logic validated before full implementation
2. **Error Code Pre-allocation**: E701-E710 already defined and ready
3. **Pattern Following**: Vector2 provided excellent reference implementation

### What to Improve 🔧

1. **Test Strategy**: Should have implemented struct literals before writing tests
2. **Dependency Planning**: Tests had hidden dependency on parser feature
3. **Documentation**: Should document "prerequisites" for tests

### Apply to Struct Literals

1. **Start Small**: Implement basic literals first, then nested
2. **Test Early**: Add 1-2 tests immediately after parser working
3. **Clear Errors**: Focus on error message quality from start

---

## 🎯 Success Criteria

### Minimum Viable Product (MVP)

- [ ] Parser accepts struct literal syntax
- [ ] Type checker validates field presence and types
- [ ] Runtime constructs Value from literals
- [ ] 15+ Phase 4 tests enabled and passing

### Complete Feature

- [ ] All 30 Phase 4 tests enabled and passing
- [ ] Nested struct literals work (Rect2 with Vector2)
- [ ] Clear error messages for all failure modes
- [ ] No clippy warnings or compiler errors
- [ ] Documentation updated

---

## 📅 Next Steps

### Immediate (This PR)

1. ✅ Commit Phase 4 work with commented tests
2. ✅ Document struct literal requirements (this file)
3. ✅ Document @export complexity (separate file)
4. ✅ Leave in uncommitted state for review

### Next Session (Struct Literals - MVP)

1. Implement basic struct literal support (2-3 hours)
2. Enable 15-20 tests
3. Validate approach works

### Future Session (Struct Literals - Complete)

1. Add nested literal support (2-3 hours)
2. Enable remaining 10-15 tests
3. Complete Phase 4

### Later (Phase 5 - @export)

1. Review EXPORT_ANNOTATION_RESEARCH.md
2. Plan incremental approach (parser → type checker → runtime → godot_bind)
3. Implement in 3-4 focused sessions

---

## 📚 References

- **STRUCT_LITERAL_SYNTAX_RESEARCH.md**: Original requirements analysis
- **EXPORT_ANNOTATION_RESEARCH.md**: Phase 5 complexity breakdown
- **PHASE_4_5_EXECUTION_PLAN.md**: Original Phase 4-5 plan
- **AST Reference**: crates/compiler/src/ast.rs lines 407-420 (Expr enum)
- **Parser Reference**: crates/compiler/src/parser.rs parse_primary()
- **Type Checker Reference**: crates/compiler/src/type_checker.rs lines 1176-1228 (field access)

---

**Status**: ✅ Ready for implementation  
**Next Action**: User reviews, then implement struct literals in dedicated session
