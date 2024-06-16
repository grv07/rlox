use std::boxed::Box;

use crate::token::{LiteralValue, Token};

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
        oprator: Token,
        expression: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
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
                oprator,
                expression,
            } => format!("({} {})", oprator.lexeme, expression.to_string()),

            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
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
                oprator: minus_token,
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
