use std::io::Write;
use std::io::{self, stdin};

use dhakal_lang::parser::Parser;
use dhakal_lang::{lexer::Lexer, token::TokenType};

fn main() {
    println!("Welcome to dhakal-lang");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input.to_string());

        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        println!("{:#?}", program.statements);
        println!("{:#?}", parser.errors);
    }
}
