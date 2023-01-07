#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negation,
    BitComp,
    LogNeg
}

// Enum to represent the different types of tokens
#[derive(Debug, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Keyword (String),
    Identifier (String),
    IntLiteral (u32),
    UnaryOp(UnaryOp),
}

// Enums to represent nodes in an Abstract Syntax Tree (AST)
#[derive(Debug)]
pub enum Expr {
    IntLiteral(u32),
    UnaryOp(UnaryOp, Box<Expr>)
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug)]
pub enum Func {
    Func(String, Stmt),
}

#[derive(Debug)]
pub enum Prog {
    Prog(Func),
}