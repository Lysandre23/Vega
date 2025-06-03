#[derive(Debug, Clone)]
pub enum Expr {
    Number(f32),
    Symbol(String),
    String(String),
    List(Vec<Expr>),
    Array(Vec<Expr>),
}