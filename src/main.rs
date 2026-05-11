use std::fs;
use std::io::Write;
use std::io::{self, stdin};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{self, Parser};
use dhakal_lang::eval::Eval;
use dhakal_lang::lexer::Lexer;
use dhakal_lang::object::Object;

#[derive(clap::Parser)]
#[command(name = "Dhakal Lang")]
#[command(about = "A simple interpreted language")]
struct Args {
    /// Path to the file
    path: Option<PathBuf>,
}

fn run(source: String, evaluator: &mut Eval) -> Vec<Object> {
    let mut lexer = Lexer::new(source);
    let mut parser = dhakal_lang::parser::Parser::new(&mut lexer);
    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        for error in &parser.errors {
            eprintln!("parse error: {error}");
        }
        return Vec::new();
    }

    evaluator.eval_program(program)
}

fn print_results(results: Vec<Object>) {
    for object in results {
        if !matches!(object, Object::Null) {
            println!("{object}");
        }
    }
}

fn repl() {
    println!("Welcome to dhakal-lang");

    let mut evaluator = Eval::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                return;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("read error: {error}");
                return;
            }
        }

        if input.trim().is_empty() {
            continue;
        }

        print_results(run(input, &mut evaluator));
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    let Some(path) = args.path else {
        repl();
        return ExitCode::SUCCESS;
    };

    if !path.exists() {
        eprintln!("file not found: {}", path.display());
        return ExitCode::FAILURE;
    }

    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("failed to read {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    };

    let mut evaluator = Eval::new();
    print_results(run(contents, &mut evaluator));

    ExitCode::SUCCESS
}
