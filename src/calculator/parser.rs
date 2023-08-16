use crate::calculator::lexer::{Lexer, Token};
pub struct Parser {}
//
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {}
    }

    pub fn eat(&mut self, expected_token: Token) {}

    pub fn factor(&mut self) -> f64 {
        0.0
    }

    pub fn term(&mut self) -> f64 {
        0.0
    }

    pub fn expression(&mut self) -> f64 {
        0.0
    }

    pub fn parse(&mut self) -> f64 {
        0.0
    }
}
