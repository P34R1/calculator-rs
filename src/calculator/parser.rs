use crate::calculator::lexer::{Lexer, Token};
use std::{error::Error, iter::Peekable};

pub fn parse_expression(tokens: &[Token]) -> f64 {
    let mut tokens_iter = tokens.iter().peekable();
    parse_expression_recursive(&mut tokens_iter)
}

fn parse_expression_recursive<'a>(
    tokens_iter: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> f64 {
    let mut result = parse_term_recursive(tokens_iter);

    while let Some(&token) = tokens_iter.peek() {
        match token {
            Token::Plus => {
                tokens_iter.next();
                let term = parse_term_recursive(tokens_iter);
                result += term;
            }
            Token::Minus => {
                tokens_iter.next();
                let term = parse_term_recursive(tokens_iter);
                result -= term;
            }
            _ => break,
        }
    }

    result
}

fn parse_term_recursive<'a>(tokens_iter: &mut Peekable<impl Iterator<Item = &'a Token>>) -> f64 {
    let mut result = parse_factor_recursive(tokens_iter);

    while let Some(&token) = tokens_iter.peek() {
        match token {
            Token::Multiply => {
                tokens_iter.next();
                let factor = parse_factor_recursive(tokens_iter);
                result *= factor;
            }
            Token::Divide => {
                tokens_iter.next();
                let factor = parse_factor_recursive(tokens_iter);
                result /= factor;
            }
            _ => break,
        }
    }

    result
}

fn parse_factor_recursive<'a>(tokens_iter: &mut Peekable<impl Iterator<Item = &'a Token>>) -> f64 {
    match tokens_iter.next() {
        Some(&Token::Number(value)) => value,
        Some(&Token::LeftParen) => {
            let expression_value = parse_expression_recursive(tokens_iter);
            if let Some(&Token::RightParen) = tokens_iter.next() {
                expression_value
            } else {
                panic!("Expected ')' after expression");
            }
        }
        _ => panic!("Unexpected token"),
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::{lexer::Lexer, parser::parse_expression};

    #[test]
    fn test_parser() {
        let input = "3.2 + 5 * (10 - 2)";
        let expected_answer = 43.2;

        let mut lexer = Lexer::new(input);

        let actual_answer = parse_expression(&lexer.get_tokens());

        assert_eq!(expected_answer, actual_answer);
    }
}
