use vega::core::interpreter::Interpreter;
use vega::core::lexer::{Lexer, Token};
use vega::core::parser::{Expr, Parser};

fn main() {
    evaluate("\
    (if (== 5 5)
    (do
        (print 1)
        (print 2)    
    )
    (do
        (print 3)
    )
    "); // -> Returns 9
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
    interpreter.compute(&mut exprs.iter().peekable(), interpreter.env.clone());
    /*println!("--------------------");
    for a in interpreter.env.variables {
        println!("{:?}", a);
    }*/
}