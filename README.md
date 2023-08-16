# Calculator-rs

This project is designed to help you become familiar with the Rust programming language by implementing a calculator with a lexer and parser. The calculator supports basic arithmetic operations and serves as a practical way to understand lexer and parser concepts using object-oriented programming (OOP) logic.

## Features

- Arithmetic operations: addition, subtraction, multiplication, division.
- Use of brackets.
- Order of Operations (BEDMAS, PEMDAS).
- Lexer to tokenize the input equation.
- Parser to interpret the tokens and calculate the result.

## Getting Started

To run this project on your local machine, follow these steps:

1. **Clone the Repository:**

    Begin by cloning this repository to your local machine using the following command:

    ```bash
    git clone P34R1/calculator-rs
    ```

2. **Navigate to Project Directory:**

    Change your current directory to the project directory:

    ```bash
    cd calculator-rs
    ```

3. **Run**:

    To run the project, use the following command:

    ```bash
    cargo run
    ```

    This will execute the calculator program, allowing you to input an equation and receive calculated results.

## Usage

The project consists of the following main components:

- lexer: This module defines the `Lexer` struct and its associated methods to tokenize the input equation.
- parser: This module defines the `Parser` struct and its associated methods to interpret the tokens and calculate the result.
- main.rs: The `main` function in the `src/main.rs` file demonstrates how to use the `Lexer` and `Parser` to calculate equations.

By default it

- Asks for your equation.
- Attempts to Lex and Parse your equation, handling errors if any.
- Prints the answer to your equation.

Feel free to modify the test statments, perform multiple equations, or experiment with the code to further practice Rust programming concepts.

## Contribution

While this project is designed as a learning exercise, contributions are
welcome. If you have improvements, bug fixes, or additional features to suggest,
feel free to fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE). You're free to use,
modify, and distribute the code as per the terms of the license.

Have fun exploring Rust and learning about basic object-oriented programming
concepts with this simple account bank system project! Happy coding!
