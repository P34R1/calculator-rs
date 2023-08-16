use crate::calculator::lexer::Token;
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
        let mut result = self.parse_term();

        while let Some(&token) = self.tokens_iter.peek() {
            match token {
                Token::Plus => {
                    self.tokens_iter.next();
                    result += self.parse_term();
                }
                Token::Minus => {
                    self.tokens_iter.next();
                    result -= self.parse_term();
                }
                _ => break,
            }
        }

        result
    }

    fn parse_term(&mut self) -> f64 {
        let mut result = self.parse_factor();

        while let Some(&token) = self.tokens_iter.peek() {
            match token {
                Token::Multiply => {
                    self.tokens_iter.next();
                    result *= self.parse_factor();
                }
                Token::Divide => {
                    self.tokens_iter.next();
                    result /= self.parse_factor();
                }
                _ => break,
            }
        }

        result
    }

    fn parse_factor(&mut self) -> f64 {
        match self.tokens_iter.next() {
            Some(&Token::Number(value)) => value,
            Some(&Token::LeftParen) => {
                let expression_value = self.parse_expression();
                match self.tokens_iter.next() {
                    Some(&Token::RightParen) => expression_value,
                    _ => panic!("Expected ')' after expression"),
                }
            }
            _ => panic!("Unexpected token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::{lexer::Lexer, parser::Parser};

    fn parse(input: &str) -> f64 {
        let mut lexer = Lexer::new(input);
        let binding = lexer.get_tokens();

        let mut parser = Parser::new(&binding);

        parser.parse_expression()
    }

    #[test]
    fn test_parser() {
        let inputs = [
            "3.2 + 5 * (10 - (2 + 3))",
            "32 - 7 * (104 / 2 + 3)",
            "6 * 4 - (10 - 2) / 2",
            "((312) + 32 * 0.5) - 14 / 2",
        ];

        let expected_answers = [28.2, -353.0, 20.0, 321.0];

        for (input, expected_answer) in inputs.iter().zip(expected_answers.iter()) {
            assert_eq!(expected_answer, &parse(input));
        }
    }
}
