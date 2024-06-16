mod expr;
mod scanner;
mod token;
mod token_type;

use anyhow::Context;
use scanner::Scanner;
use std::io::Write;
use std::{env, fs::File, io::Read, path::Path};
// use token::Token;
// use token_type::TokenType;

#[derive(Default)]
struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_file(&self, path: &Path) -> anyhow::Result<()> {
        let mut content = String::default();
        Read::read_to_string(
            &mut File::open(path).with_context(|| format!("Path: {:?}", path))?,
            &mut content,
        )?;

        run(content);

        if self.had_error {
            std::process::exit(64);
        }

        Ok(())
    }

    fn run_prompt(&mut self) {
        loop {
            let _ = std::io::stdout().write(b"> ");
            let _ = std::io::stdout().flush();

            let mut line = String::default();
            let _ = std::io::stdin().read_line(&mut line);

            println!("{}", line);
            run(line);
            self.had_error = false;
        }
    }
}

struct ErrorMsg;

impl ErrorMsg {
    fn error(line: usize, msg: &str) {
        Self::report(line, "", msg);
    }

    fn report(line: usize, wh: &str, msg: &str) {
        eprintln!("[line {line}] Error {wh}: {msg}");
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(&source);
    for token in scanner.scan_tokens() {
        println!("{token:?}");
    }
}

fn main() -> anyhow::Result<()> {
    let mut lox = Lox::default();
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        std::process::exit(64)
    } else if args.len() == 2 {
        lox.run_file(&Path::new(args.get(1).unwrap()))?;
    } else {
        let _ = lox.run_prompt();
    }

    Ok(())
}
