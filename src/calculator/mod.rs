mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

pub fn calculate(equation: &str) -> f64 {
    let mut lexer = Lexer::new(equation);
    let binding = lexer.get_tokens();

    let mut parser = Parser::new(&binding);
    parser.parse_expression()
}

#[cfg(test)]
mod tests {
    use crate::calculator::calculate;

    #[test]
    fn test_calculate() {
        let inputs = [
            "3.2 + 5 * (10 - (2 + 3))",
            "32 - 7 * (104 / 2 + 3)",
            "6 * 4 - (10 - 2) / 2",
            "((312) + 32 * 0.5) - 14 / 2",
        ];

        let expected_answers = [28.2, -353.0, 20.0, 321.0];

        for (input, expected_answer) in inputs.iter().zip(expected_answers.iter()) {
            assert_eq!(expected_answer, &calculate(input));
        }
    }
}
