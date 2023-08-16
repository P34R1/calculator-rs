#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Example,
}

pub struct Lexer {}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {}
    }

    pub fn next_token(&mut self) -> Token {
        Token::Example
    }
}
