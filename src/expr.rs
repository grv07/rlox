use std::{boxed::Box, cell::RefCell, rc::Rc};

use crate::{
    interpret::Environment,
    token::{LiteralValue, Token},
    token_type::TokenType,
};

// enum LiteralType {
//     Number,
//     String,
//     True,
//     False,
//     Nil,
// }

// impl Into<String> for LiteralType {
//     fn into(self) -> String {
//         match self {
//             LiteralType::Number => todo!(),
//             LiteralType::String => todo!(),
//             LiteralType::True => todo!(),
//             LiteralType::False => todo!(),
//             LiteralType::Nil => todo!(),
//         }
//     }
// }

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Unary {
        operator: Token,
        expression: Box<Expr>,
    },

    Grouping {
        expression: Box<Expr>,
    },

    Literal {
        value: LiteralValue,
    },

    Variable {
        name: Token,
    },

    Assign {
        name: Token,
        value: Box<Expr>,
    },
}

impl Expr {
    pub fn evaluate(&self, env: Rc<RefCell<Environment>>) -> LiteralValue {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let op = |condition: bool| {
                    if condition {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    }
                };
                match operator.token_type {
                    TokenType::Minus => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => {
                                LiteralValue::IntValue(a - b)
                            }
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => {
                                LiteralValue::FValue(a - b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::Slash => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => {
                                LiteralValue::IntValue(a / b)
                            }
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => {
                                LiteralValue::FValue(a / b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::Star => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => {
                                LiteralValue::IntValue(a * b)
                            }
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => {
                                LiteralValue::FValue(a * b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::Plus => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => {
                                LiteralValue::IntValue(a + b)
                            }
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => {
                                LiteralValue::FValue(a + b)
                            }
                            (LiteralValue::StringValue(a), LiteralValue::StringValue(b)) => {
                                LiteralValue::StringValue(format!("{}{}", a, b))
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::Greater => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => op(a > b),
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => op(a > b),
                            (LiteralValue::StringValue(a), LiteralValue::StringValue(b)) => {
                                op(a > b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::Less => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => op(a < b),
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => op(a < b),
                            (LiteralValue::StringValue(a), LiteralValue::StringValue(b)) => {
                                op(a < b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::GreaterEqual => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => op(a >= b),
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => op(a >= b),
                            (LiteralValue::StringValue(a), LiteralValue::StringValue(b)) => {
                                op(a >= b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::LessEqual => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => op(a <= b),
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => op(a <= b),
                            (LiteralValue::StringValue(a), LiteralValue::StringValue(b)) => {
                                op(a <= b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::EqualEqual => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => op(a == b),
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => op(a == b),
                            (LiteralValue::StringValue(a), LiteralValue::StringValue(b)) => {
                                op(a == b)
                            }
                            _ => todo!(),
                        }
                    }

                    TokenType::BangEqual => {
                        match (left.evaluate(env.clone()), right.evaluate(env.clone())) {
                            (LiteralValue::IntValue(a), LiteralValue::IntValue(b)) => op(a != b),
                            (LiteralValue::FValue(a), LiteralValue::FValue(b)) => op(a != b),
                            (LiteralValue::StringValue(a), LiteralValue::StringValue(b)) => {
                                op(a != b)
                            }
                            _ => todo!(),
                        }
                    }

                    _ => {
                        println!("Defsault case");
                        todo!()
                    }
                }
            }
            Expr::Unary {
                operator,
                expression,
            } => match (&operator.token_type, expression.evaluate(env.clone())) {
                (TokenType::Minus, LiteralValue::IntValue(x)) => LiteralValue::IntValue(-x),
                (TokenType::Minus, LiteralValue::FValue(x)) => LiteralValue::FValue(-x),
                (TokenType::Bang, LiteralValue::True) => LiteralValue::False,
                (TokenType::Bang, LiteralValue::False) => LiteralValue::True,
                _ => {
                    println!("Not yet handled");
                    todo!()
                }
            },
            Expr::Variable { name } => {
                let mut env = env.borrow_mut();
                let value = env.get(&name.lexeme);
                value.to_owned()
            }

            Expr::Assign { name, value } => {
                let value = value.evaluate(env.clone());
                let mut env = env.borrow_mut();
                env.assign(&name.lexeme, value)
            }

            Expr::Grouping { expression } => expression.evaluate(env),
            Expr::Literal { value } => value.clone(),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),

            Expr::Unary {
                operator,
                expression,
            } => format!("({} {})", operator.lexeme, expression.to_string()),

            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Variable { .. } => todo!("---------"),
            Expr::Assign { .. } => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pp_ast() {
        let minus_token = Token::new(
            crate::token_type::TokenType::Minus,
            None,
            "-".to_string(),
            0,
        );

        let one_two_three = LiteralValue::IntValue(123);

        let group = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::FValue(45.67),
            }),
        };

        let multi = Token::new(crate::token_type::TokenType::Star, None, "*".to_string(), 0);
        let ast = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: minus_token,
                expression: Box::new(Expr::Literal {
                    value: one_two_three,
                }),
            }),
            operator: multi,
            right: Box::new(group),
        };

        assert_eq!("(* (- 123) (group 45.67))", ast.to_string());
    }
}
