use crate::{
    expr::Expr,
    token::{LiteralValue, Token},
    token_type::TokenType,
    ErrorMsg,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        todo!()
    }

    fn comparison() -> Expr {
        todo!()
    }

    fn term() -> Expr {
        todo!()
    }

    fn factor() -> Expr {
        todo!()
    }

    /// (! | -) unary | primary
    fn unary(&mut self) -> Expr {
        if self.match_token([TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous().clone();
            let right = self.unary();

            return Expr::Unary {
                operator: operator.clone(),
                expression: Box::new(right),
            };
        }

        self.primary().unwrap()
    }

    /// primary = Number | String | True | False | Nil | "(" expression ")"
    fn primary(&mut self) -> Option<Expr> {
        if self.match_token([TokenType::INumber]) {
            let lit = self.previous().clone().literal.unwrap();
            return Some(Expr::Literal { value: lit });
        }

        if self.match_token([TokenType::FNumber]) {
            let lit = self.previous().clone().literal.unwrap();
            return Some(Expr::Literal { value: lit });
        }

        if self.match_token([TokenType::String]) {
            let lit = self.previous().clone().literal.unwrap();
            return Some(Expr::Literal { value: lit });
        }

        if self.match_token([TokenType::True]) {
            return Some(Expr::Literal {
                value: LiteralValue::True,
            });
        }

        if self.match_token([TokenType::False]) {
            return Some(Expr::Literal {
                value: LiteralValue::True,
            });
        }

        if self.match_token([TokenType::Nil]) {
            return Some(Expr::Literal {
                value: LiteralValue::Nil,
            });
        }

        if self.match_token([TokenType::LeftParen]) {
            let expr = self.expression();
            // advance and check if next is right paren
            if self.advance().token_type == TokenType::RightParen {
                return Some(Expr::Grouping {
                    expression: Box::new(expr),
                });
            }

            return None;
        }
        None
    }

    // fn consume(&mut self, token_types: impl IntoIterator<Item = TokenType>, msg: &str) {
    //     if !self.match_token(token_types) {
    //         ErrorMsg::report(, , )
    //     }
    // }

    fn match_token(&mut self, token_types: impl IntoIterator<Item = TokenType>) -> bool {
        if token_types.into_iter().any(|tt| self.check(tt)) {
            self.advance();
            return true;
        }
        false
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::token::Token;

    #[test]
    fn primary_test() {
        let mut parser = Parser::new(vec![Token::new(
            crate::token_type::TokenType::String,
            Some(crate::token::LiteralValue::StringValue("data".to_string())),
            "data".to_string(),
            1,
        )]);

        println!(" ==== {:?}", parser.primary());
    }
}
