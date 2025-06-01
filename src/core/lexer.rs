use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Number(f32),
    Identifier(String),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    String(String)
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '"' {
                let mut string_lit = String::new();
                while let Some(next_c) = chars.next() {
                    if next_c == '"' {
                        break;
                    } else {
                        string_lit.push(next_c);
                    }
                }
                tokens.push(Token::String(string_lit));
            } else if "()[] \n".contains(c) {
                if !current.is_empty() {
                    if current.chars().all(|a| a.is_ascii_digit() || a == '.') {
                        tokens.push(Token::Number(f32::from_str(&current).unwrap()));
                    } else {
                        tokens.push(Token::Identifier(current.clone()));
                    }
                    current.clear();
                }
                match c {
                    '(' => tokens.push(Token::LeftParen),
                    ')' => tokens.push(Token::RightParen),
                    '[' => tokens.push(Token::LeftBracket),
                    ']' => tokens.push(Token::RightBracket),
                    _ => {}
                }
            } else {
                current.push(c);
            }
        }
        tokens
    }
}