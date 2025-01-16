use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, BufReader, Read, Write};
use std::{env, process};

mod error_format;
mod expr;
mod parser;
mod scanner;

const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> io::Result<()> {
    println!(
        "===================================================\n\
        Welcome to lox {}!\n\
        Inspired by the book \x1B[3mCrafting Interpreters\x1B[0m\nAuthors: {}\n\
        ===================================================\n",
        VERSION, AUTHORS
    );

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        loop {
            print!("> ");
            stdout().flush()?;
            let mut chars = String::new();
            stdin().read_line(&mut chars).expect("Failed to read input");
            let content = chars.trim();
            if content.len() == 0 {
                println!("No input.");
                process::exit(1);
            }
            if content.to_lowercase() == "bye" {
                println!("Exit REPL.");
                std::process::exit(1);
            }
            println!("input: {}", content);
        }
    } else if args.len() != 2 {
        eprintln!("Usage: lox [script]");
        process::exit(1);
    }

    let source_path = args.get(1).unwrap();
    println!("source file: {}", source_path);
    let mut file = File::open(source_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut scanner = crate::scanner::Scanner::new(content);
    scanner.scan_tokens();

    Ok(())
}
