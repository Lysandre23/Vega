use vega::core::interpreter::Interpreter;
use vega::core::lexer::{Lexer, Token};
use vega::core::parser::{Expr, Parser};

fn main() {
    evaluate("\
    (let a [1 2 3])\
    (let size (len a))\
    (print (* size size))"); // -> Returns 9
}

fn evaluate(input: &str) {
    let tokens: Vec<Token> = Lexer::tokenize(input);
    /*for t in &tokens {
        println!("{:?}", t);
    }*/
    let exprs: Vec<Expr> = Parser::parse(&mut tokens.into_iter().peekable());
    /*for expr in &exprs {
        println!("{:?}", expr);
    }*/
    let mut interpreter = Interpreter::new();
    interpreter.compute(&mut exprs.iter().peekable());
    /*println!("--------------------");
    for a in interpreter.env.variables {
        println!("{:?}", a);
    }*/
}