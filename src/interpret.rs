use crate::{parser::Stmt, token::LiteralValue};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, Debug, Clone)]
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
            None => {
                println!("Var: {} not found in this scope", key);
                &LiteralValue::Nil
            }
        }
    }
}

pub struct Interpret;

impl Interpret {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, stmts: &[Stmt], env: Rc<RefCell<Environment>>) {
        for stmt in stmts {
            self.execute(stmt, env.clone());
        }
    }

    fn execute(&mut self, stmt: &Stmt, env: Rc<RefCell<Environment>>) {
        match stmt {
            Stmt::Expression(expr) => {
                expr.evaluate(env);
            }
            Stmt::Print(expr) => {
                println!("{}", expr.evaluate(env.clone()).to_string());
            }
            Stmt::Variable { token, expression } => {
                let value = expression.evaluate(env.clone());
                let name = token.lexeme.to_owned();

                let mut t = env.borrow_mut();

                t.define(name, value);
            }
            Stmt::Block(stmts) => {
                let inner_env = Rc::new(RefCell::new(env.clone().borrow().clone()));

                for stmt in stmts {
                    self.execute(stmt, inner_env.clone());
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if condition.evaluate(env.clone()) == LiteralValue::True {
                    self.execute(then_branch, env)
                } else {
                    if let Some(stmt) = else_branch {
                        self.execute(stmt, env.clone())
                    }
                }
            }
        }
    }
}
