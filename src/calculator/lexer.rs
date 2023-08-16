use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    Eof,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Self {
        // Remove spaces
        let input_no_spaces: &'a str = Box::leak(input.replace(' ', "").into_boxed_str());

        Lexer {
            input: input_no_spaces.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        match self.input.next() {
            Some(c) => match c {
                '0'..='9' => {
                    let mut num_str = c.to_string();
                    while let Some(&next_char) = self.input.peek() {
                        if next_char.is_ascii_digit() || next_char == '.' {
                            num_str.push(self.input.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    Token::Number(num_str.parse::<f64>().unwrap())
                }
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                _ => panic!("Invalid character: {}", c),
            },
            None => Token::Eof,
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            if token != Token::Eof {
                tokens.push(token);
            } else {
                break;
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::lexer::{Lexer, Token};

    #[test]
    fn test_lexer() {
        use Token::*;

        let input = "3.2 + 5 * (10 - 2) / 2";

        let expected_tokens = vec![
            Number(3.2),
            Plus,
            Number(5.0),
            Multiply,
            LeftParen,
            Number(10.0),
            Minus,
            Number(2.0),
            RightParen,
            Divide,
            Number(2.0),
        ];

        let mut lexer = Lexer::new(input);

        let actual_tokens = lexer.get_tokens();

        assert_eq!(actual_tokens, expected_tokens);
    }
}
