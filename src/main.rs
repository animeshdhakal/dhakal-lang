use std::fs;
use std::io::Write;
use std::io::{self, stdin};
use std::path::PathBuf;

use clap::{self, Parser};
use dhakal_lang::eval::Eval;
use dhakal_lang::lexer::Lexer;

#[derive(clap::Parser)]
#[command(name = "Dhakal Lang")]
#[command(about = "A simple interpreted language")]
struct Args {
    /// Path to the file
    path: Option<PathBuf>,
}

fn repl() {
    println!("Welcome to dhakal-lang");

    let mut evaluator = Eval::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input.to_string());

        let mut parser = dhakal_lang::parser::Parser::new(&mut lexer);
        let program = parser.parse_program();

        evaluator.eval_program(program);
    }
}

fn main() {
    let args = Args::parse();

    if args.path.is_none() {
        repl();
        return;
    }

    let path = args.path.unwrap();

    if !path.exists() {
        println!("The file doesn't exists");
    }

    let contents = fs::read_to_string(path).unwrap();

    println!("Contents: {contents}");
}
