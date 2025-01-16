pub enum Expr {
    Assign(Symbol, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Call(Box<Expr>, SourceLocation, Vec<Expr>),
    Get(Box<Expr>, Symbol),
    Group(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    Set(Box<Expr>, Symbol, Box<Expr>),
    Super(SourceLocation, Symbol),
    This(SourceLocation),
    Variable(Symbol),
}

pub enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

pub struct SourceLocation {
    line: usize,
    col: i64,
}

pub struct Symbol {
    pub name: String,
    line: usize,
    col: i64,
}

pub enum LogicalOp {
    And,
    Or,
}

pub enum UnaryOpType {
    Minus,
    Bang,
}

pub struct UnaryOp {
    op_type: UnaryOpType,
    line: usize,
    col: i64,
}

pub enum BinaryOpType {
    Equal,
    EqualEqual,
}

pub struct BinaryOp {
    pub op_type: BinaryOpType,
    pub line: usize,
    pub col: i64,
}
