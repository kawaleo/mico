use super::lexer::Token;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    BinaryExpr(BinaryExpr),
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub operator: Token,
}
