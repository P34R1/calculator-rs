use crate::calculator::lexer::{Lexer, Token};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    tokens_iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let tokens_iter: Peekable<Iter<'a, Token>> = tokens.iter().peekable();

        Parser { tokens_iter }
    }
    pub fn parse_expression(&mut self) -> f64 {
        self.parse_expression_recursive()
    }

    fn parse_expression_recursive(&mut self) -> f64 {
        let mut result = self.parse_term_recursive();

        while let Some(&token) = self.tokens_iter.peek() {
            match token {
                Token::Plus => {
                    self.tokens_iter.next();
                    let term = self.parse_term_recursive();
                    result += term;
                }
                Token::Minus => {
                    self.tokens_iter.next();
                    let term = self.parse_term_recursive();
                    result -= term;
                }
                _ => break,
            }
        }

        result
    }

    fn parse_term_recursive(&mut self) -> f64 {
        let mut result = self.parse_factor_recursive();

        while let Some(&token) = self.tokens_iter.peek() {
            match token {
                Token::Multiply => {
                    self.tokens_iter.next();
                    let factor = self.parse_factor_recursive();
                    result *= factor;
                }
                Token::Divide => {
                    self.tokens_iter.next();
                    let factor = self.parse_factor_recursive();
                    result /= factor;
                }
                _ => break,
            }
        }

        result
    }

    fn parse_factor_recursive(&mut self) -> f64 {
        match self.tokens_iter.next() {
            Some(&Token::Number(value)) => value,
            Some(&Token::LeftParen) => {
                let expression_value = self.parse_expression_recursive();
                if let Some(&Token::RightParen) = self.tokens_iter.next() {
                    expression_value
                } else {
                    panic!("Expected ')' after expression");
                }
            }
            _ => panic!("Unexpected token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_parser() {
        let input = "3.2 + 5 * (10 - 2)";
        let expected_answer = 43.2;

        let mut lexer = Lexer::new(input);
        let binding = lexer.get_tokens();

        let mut parser = Parser::new(&binding);

        let actual_answer = parser.parse_expression();

        assert_eq!(expected_answer, actual_answer);
    }
}
