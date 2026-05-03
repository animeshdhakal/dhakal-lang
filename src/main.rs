use std::io::Write;
use std::io::{self, stdin};

use dhakal_lang::{lexer::Lexer, token::TokenType};

fn main() {
    println!("Welcome to dhakal-lang");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input.to_string());

        'token_loop: loop {
            let token = lexer.next_token();
            println!("{:#?}", token);
            if token.token_type == TokenType::Eof {
                break 'token_loop;
            }
        }
    }
}
