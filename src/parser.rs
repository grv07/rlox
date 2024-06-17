use std::fmt::Binary;

use crate::{
    expr::{self, Expr},
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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let expr = self.comparison();
        while self.match_token([TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().to_owned();

            let right = Box::new(self.comparison());

            return Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        expr
    }

    /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Expr {
        let expr = self.term();

        while self.match_token([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().to_owned();
            let right = Box::new(self.term());

            return Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        expr
    }

    /// term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expr {
        let expr = self.factor();

        while self.match_token([TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().to_owned();
            let right = Box::new(self.factor());

            return Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }
        expr
    }

    /// factor → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Expr {
        let expr = self.unary();
        while self.match_token([TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().to_owned();

            let right = Box::from(self.unary());

            return Expr::Binary {
                left: Box::from(expr),
                operator,
                right,
            };
        }

        expr
    }

    /// (! | -) unary | primary
    fn unary(&mut self) -> Expr {
        if self.match_token([TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous().clone();
            let right = self.unary();

            return Expr::Unary {
                operator: operator.clone(),
                expression: Box::from(right),
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
            self.error(self.previous(), "Expecting right paren");
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

    fn error(&self, token: &Token, msg: &str) {
        ErrorMsg::error(token, msg);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            if matches!(
                self.peek().token_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{scanner::Scanner, token::Token};

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

    #[test]
    fn unary_test() {
        let text = "! (1 + 9)";
        let mut scanner = Scanner::new(text);
        let tokens = scanner.scan_tokens();

        println!("{tokens:?}");

        let mut parser = Parser::new(tokens.to_vec());

        println!("{:?}", parser.unary());
    }
}
