use std::fs;
use std::path::PathBuf;
use clap::{arg, Command, Parser, Subcommand};
use vega::core::expr::Expr;
use vega::core::interpreter::Interpreter;
use vega::core::lexer::{Lexer, Token};

#[derive(Parser)]
#[command(name = "vega")]
#[command(about = "Vega interpreter")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file } => {
            match fs::read_to_string(&file) {
                Ok(content) => evaluate(content.as_str()),
                Err(e) => eprintln!("Error reading file {:?}: {}", file, e),
            }
        }
    }
}

fn evaluate(input: &str) {
    let tokens: Vec<Token> = Lexer::tokenize(input);
    let exprs: Vec<Expr> = vega::core::parser::Parser::parse(&mut tokens.into_iter().peekable());
    let mut interpreter = Interpreter::new();
    interpreter.compute(&mut exprs.iter().peekable(), interpreter.env.clone());
}