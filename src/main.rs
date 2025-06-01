use vega::core::interpreter::Interpreter;
use vega::core::lexer::{Lexer, Token};
use vega::core::parser::{Expr, Parser};

fn main() {
    evaluate("\
    (fn fact (n)
    (do
      (print n)
      (if (== n 1)
          1
          (* n (fact (- n 1))))
      )
    )
    (print (fact 5))
    ");
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