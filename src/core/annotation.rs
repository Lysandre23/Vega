use crate::core::expr::Expr;

#[derive(Debug, Clone)]
pub enum Annotation {
    Require(Expr),
    Test {args: Vec<Expr>, expected: Expr}
}