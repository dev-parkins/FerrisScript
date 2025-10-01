use std::fmt;

/// Position in source code (line, column)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Self {
        Span { line, column }
    }

    pub fn unknown() -> Self {
        Span { line: 0, column: 0 }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub global_vars: Vec<GlobalVar>,
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            global_vars: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for var in &self.global_vars {
            writeln!(f, "{}", var)?;
        }
        for func in &self.functions {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalVar {
    pub name: String,
    pub mutable: bool,
    pub ty: Option<String>,
    pub value: Expr,
    pub span: Span,
}

impl fmt::Display for GlobalVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let ")?;
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}", self.name)?;
        if let Some(ty) = &self.ty {
            write!(f, ": {}", ty)?;
        }
        write!(f, " = {};", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Vec<Stmt>,
    pub span: Span,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ")")?;
        if let Some(ret) = &self.return_type {
            write!(f, " -> {}", ret)?;
        }
        writeln!(f, " {{")?;
        for stmt in &self.body {
            writeln!(f, "    {}", stmt)?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: String,
    pub span: Span,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        mutable: bool,
        ty: Option<String>,
        value: Expr,
        span: Span,
    },
    Assign {
        target: Expr,
        value: Expr,
        span: Span,
    },
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
        span: Span,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{};", expr),
            Stmt::Let { name, mutable, ty, value, .. } => {
                write!(f, "let ")?;
                if *mutable {
                    write!(f, "mut ")?;
                }
                write!(f, "{}", name)?;
                if let Some(t) = ty {
                    write!(f, ": {}", t)?;
                }
                write!(f, " = {};", value)
            }
            Stmt::Assign { target, value, .. } => {
                write!(f, "{} = {};", target, value)
            }
            Stmt::If { cond, then_branch, else_branch, .. } => {
                write!(f, "if {} {{ ", cond)?;
                for stmt in then_branch {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")?;
                if !else_branch.is_empty() {
                    write!(f, " else {{ ")?;
                    for stmt in else_branch {
                        write!(f, "{} ", stmt)?;
                    }
                    write!(f, "}}")?;
                }
                Ok(())
            }
            Stmt::While { cond, body, .. } => {
                write!(f, "while {} {{ ", cond)?;
                for stmt in body {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::Return { value, .. } => {
                write!(f, "return")?;
                if let Some(v) = value {
                    write!(f, " {}", v)?;
                }
                write!(f, ";")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal, Span),
    Variable(String, Span),
    Binary(Box<Expr>, BinaryOp, Box<Expr>, Span),
    Unary(UnaryOp, Box<Expr>, Span),
    Call(String, Vec<Expr>, Span),
    FieldAccess(Box<Expr>, String, Span),
    Assign(Box<Expr>, Box<Expr>, Span),
    CompoundAssign(Box<Expr>, CompoundOp, Box<Expr>, Span),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(_, s) => *s,
            Expr::Variable(_, s) => *s,
            Expr::Binary(_, _, _, s) => *s,
            Expr::Unary(_, _, s) => *s,
            Expr::Call(_, _, s) => *s,
            Expr::FieldAccess(_, _, s) => *s,
            Expr::Assign(_, _, s) => *s,
            Expr::CompoundAssign(_, _, _, s) => *s,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(lit, _) => write!(f, "{}", lit),
            Expr::Variable(name, _) => write!(f, "{}", name),
            Expr::Binary(left, op, right, _) => write!(f, "({} {} {})", left, op, right),
            Expr::Unary(op, expr, _) => write!(f, "({}{})", op, expr),
            Expr::Call(name, args, _) => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::FieldAccess(obj, field, _) => write!(f, "{}.{}", obj, field),
            Expr::Assign(target, value, _) => write!(f, "{} = {}", target, value),
            Expr::CompoundAssign(target, op, value, _) => write!(f, "{} {} {}", target, op, value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundOp {
    AddAssign,
    SubAssign,
}

impl fmt::Display for CompoundOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CompoundOp::AddAssign => "+=",
            CompoundOp::SubAssign => "-=",
        };
        write!(f, "{}", s)
    }
}

// Backwards compatibility alias
pub type Op = BinaryOp;
