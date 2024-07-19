mod scanner;
mod parser;
mod ast;
mod interpreter;

use std::io::{self,Write};
use scanner::Scanner;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();

    interpreter.set_variable("x".to_string(), 10.0);
    
    loop {
        print!("Enter expression (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let scanner = Scanner::new(input);
        let mut parser = Parser::new(scanner);
        let ast = parser.parse();

        let result = interpreter.interpret(ast);

        println!("Result: {}", result);
    }
}