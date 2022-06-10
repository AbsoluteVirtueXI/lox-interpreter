use crate::lexer::*;
use core::cmp::Ordering;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn main(&mut self) {
        let args = env::args().collect::<Vec<String>>();
        match args.len().cmp(&2) {
            Ordering::Greater => {
                eprintln!("Usage: lox [script]");
                process::exit(64);
            }
            Ordering::Equal => {
                self.run_file(&args[1]);
            }
            _ => {
                self.run_prompt();
            }
        }
    }

    fn run_file(&self, file_name: &str) {
        let source = fs::read_to_string(file_name).expect("Can't read file");
        self.run(&source);
        if self.had_error {
            process::exit(65);
        }
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().expect("Error while flushing stdout");
            let mut source = String::new();
            io::stdin().read_line(&mut source).expect("Can't read line");
            let source = source.trim();
            self.run(source);
            self.had_error = false;
        }
    }

    fn run(&self, source: &str) {
        let lexer = Lexer::new();
        let tokens = lexer.scan_tokens(source);
        for token in tokens {
            println!("{:?}", token);
        }
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, "", &message);
    }

    fn report(&mut self, line: usize, at: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, at, message);
        self.had_error = true;
    }
}
