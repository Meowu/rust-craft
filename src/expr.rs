#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Symbol, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Call(Box<Expr>, SourceLocation, Vec<Expr>),
    Get(Box<Expr>, Symbol),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    Set(Box<Expr>, Symbol, Box<Expr>),
    Super(SourceLocation, Symbol),
    This(SourceLocation),
    Variable(Symbol),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    line: usize,
    col: i64,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    line: usize,
    col: i64,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpType {
    Minus,
    Bang,
}

#[derive(Debug, Clone, Copy)]
pub struct UnaryOp {
    pub op_type: UnaryOpType,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpType {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star,
}

#[derive(Debug, Clone, Copy)]
pub struct BinaryOp {
    pub op_type: BinaryOpType,
    pub line: usize,
    pub col: i64,
}
