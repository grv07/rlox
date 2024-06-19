use crate::{parser::Stmt, token::LiteralValue};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Environment {
    define: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn define(&mut self, key: String, value: LiteralValue) {
        self.define.insert(key, value);
    }

    pub fn get(&mut self, key: &str) -> &LiteralValue {
        match self.define.get(key) {
            Some(v) => v,
            None => todo!(""),
        }
    }
}

// #[derive(Default)]
pub struct Interpret<'a> {
    environment: &'a mut Environment,
}

impl<'a> Interpret<'a> {
    pub fn new(environment: &'a mut Environment) -> Self {
        Self { environment }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                expr.evaluate(&mut self.environment);
            }
            Stmt::Print(expr) => {
                println!("{}", expr.evaluate(&mut self.environment).to_string());
            }
            Stmt::Variable { token, expression } => {
                let value = expression.evaluate(&mut self.environment);
                let name = token.lexeme.to_owned();

                self.environment.define(name, value);
            }
        }
    }
}
