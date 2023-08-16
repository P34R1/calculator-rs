mod calculator;
use calculator::calculate;

use std::io::{stdin, stdout, Write};

fn main() {
    print!("Enter Your Equation\n> ");
    Write::flush(&mut stdout()).expect("Failed to flush stdout");

    let mut equation = String::new();
    stdin()
        .read_line(&mut equation)
        .expect("Failed to read line");

    println!("{}", calculate(equation.trim()));
}
