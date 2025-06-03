use std::iter::Peekable;
pub(crate) use crate::core::expr::Expr;
use crate::core::lexer::Token;

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Vec<Expr> {
        let mut exprs: Vec<Expr> = Vec::new();
        while let Some(token) = tokens.next() {
            match token {
                Token::Identifier(i) => exprs.push(Expr::Symbol(i.to_string())),
                Token::Number(n) => exprs.push(Expr::Number(n)),
                Token::String(s) => exprs.push(Expr::String(s)),
                Token::LeftParen => {
                    let inner_expr = Self::parse(tokens);
                    exprs.push(Expr::List(inner_expr));
                },
                Token::RightParen => break,
                Token::LeftBracket => {
                    let inner_expr = Self::parse(tokens);
                    exprs.push(Expr::Array(inner_expr));
                },
                Token::RightBracket => break,
            }
        }
        exprs
    }
}