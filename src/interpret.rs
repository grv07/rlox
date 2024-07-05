use crate::{parser::Stmt, token::LiteralValue, token_type::TokenType};
use std::{cell::RefCell, collections::HashMap, ops::RangeInclusive, rc::Rc};

#[derive(Default, Debug, Clone)]
pub struct Environment {
    define: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Rc<RefCell<Environment>>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Rc<RefCell<Environment>>>>) -> Self {
        Self {
            enclosing,
            ..Default::default()
        }
    }

    pub fn define(&mut self, key: String, value: LiteralValue) {
        self.define.insert(key, value);
    }

    pub fn assign(&mut self, key: &str, value: LiteralValue) -> LiteralValue {
        if self.define.contains_key(key) {
            let inserted = self.define.insert(key.to_string(), value);
            inserted.unwrap()
        } else {
            if let Some(enclosing) = self.enclosing.as_ref() {
                return enclosing.borrow_mut().assign(key, value);
            };

            println!("Var: {} not found in this scope", key);
            LiteralValue::Nil
        }
    }

    pub fn get(&self, key: &str) -> LiteralValue {
        let value = self
            .define
            .get(key)
            .unwrap_or(&LiteralValue::Nil)
            .to_owned();

        match value {
            LiteralValue::Nil => {
                // if eclosing is None
                let Some(enclosing) = self.enclosing.as_ref() else {
                    return LiteralValue::Nil;
                };

                return enclosing.borrow().get(key);
            }
            _ => value,
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
                let env = Rc::new(RefCell::new(Environment::new(Some(Box::new(env.clone())))));

                for stmt in stmts {
                    self.execute(stmt, env.clone());
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

            Stmt::While { expr, stmt } => {
                while expr.evaluate(env.clone()) == LiteralValue::True {
                    self.execute(&stmt, env.clone());
                }
            }

            _ => todo!(),
        }
    }
}
