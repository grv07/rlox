mod expr;
mod interpret;
mod parser;
mod scanner;
mod token;
mod token_type;

use anyhow::Context;
use interpret::{Environment, Interpret};
use parser::Parser;
use scanner::Scanner;
use std::io::Write;
use std::{env, fs::File, io::Read, path::Path};
use token::Token;
use token_type::TokenType;
// use token::Token;
// use token_type::TokenType;

#[derive(Default)]
struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_file(&self, path: &Path, env: &mut Environment) -> anyhow::Result<()> {
        let mut content = String::default();
        Read::read_to_string(
            &mut File::open(path).with_context(|| format!("Path: {:?}", path))?,
            &mut content,
        )?;

        run(content, env);

        if self.had_error {
            std::process::exit(64);
        }

        Ok(())
    }

    fn run_prompt(&mut self, env: &mut Environment) {
        loop {
            let _ = std::io::stdout().write(b"> ");
            let _ = std::io::stdout().flush();

            let mut line = String::default();
            let _ = std::io::stdin().read_line(&mut line);

            // println!("{}", line);
            run(line, env);
            self.had_error = false;
        }
    }
}

pub struct ErrorMsg;

impl ErrorMsg {
    pub fn error(token: &Token, msg: &str) {
        if token.token_type == TokenType::Eof {
            Self::report(token.line, " at end ", msg);
        }
        Self::report(
            token.line,
            &format!(" at '{lexme}' ", lexme = &token.lexeme),
            msg,
        );
    }

    pub fn report(line: usize, wh: &str, msg: &str) {
        eprintln!("[line {line}] Error {wh}: {msg}");
    }
}

fn run(source: String, env: &mut Environment) {
    let mut scanner = Scanner::new(&source);

    let parser = Parser::new(scanner.scan_tokens().to_vec());
    let stmts = parser.parse();

    let mut interpret = Interpret::new(env);
    interpret.interpret(&stmts);

    // for stmt in stmts {
    //     println!("ECHO: {}", stmt.evaluate().to_string());
    // }
    // println!("ECHO: {}", exprs.evaluate().to_string());
}

fn main() -> anyhow::Result<()> {
    let mut env = Environment::new();
    let mut lox = Lox::default();
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        std::process::exit(64)
    } else if args.len() == 2 {
        lox.run_file(&Path::new(args.get(1).unwrap()), &mut env)?;
    } else {
        let _ = lox.run_prompt(&mut env);
    }

    Ok(())
}
