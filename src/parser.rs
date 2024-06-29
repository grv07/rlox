use crate::{
    expr::Expr,
    token::{LiteralValue, Token},
    token_type::TokenType,
    ErrorMsg,
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    // env: &'a mut Environment,
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Block(Vec<Stmt>),

    Variable {
        token: Token,
        expression: Expr,
    },

    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            // env,
        }
    }

    pub fn parse(mut self) -> Vec<Stmt> {
        let mut stmts = vec![];

        while !self.is_at_end() {
            stmts.push(self.declaration());
        }
        stmts
        // TODO: Handle synax errors
    }

    fn _statement(&mut self) -> Stmt {
        if self.match_token([TokenType::Print]) {
            return self.print_stmt();
        }
        self.expression_stmt()
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_token([TokenType::Var]) {
            return self.var_declaration();
        }

        if self.match_token([TokenType::Print]) {
            return self.print_stmt();
        }

        if self.match_token([TokenType::If]) {
            return self.if_stmt();
        }

        if self.match_token([TokenType::LeftBrace]) {
            let mut statements = vec![];

            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                statements.push(self.declaration());
            }

            // dbg!(&statements);
            self.consume(TokenType::RightBrace, "Expect } after block");

            return Stmt::Block(statements);
        }

        self.expression_stmt()
    }

    fn if_stmt(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'");

        let condition = self.expression();

        self.consume(TokenType::RightParen, "Expect ')' after 'if'");

        let then_stmt = self.declaration();

        let else_stmt = if self.match_token([TokenType::Else]) {
            Some(Box::new(self.declaration()))
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch: Box::new(then_stmt),
            else_branch: else_stmt,
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        let token = self.consume(TokenType::Identifier, "Expect identifier");
        self.consume(TokenType::Equal, "Expect '=' after identifer");

        let expression = self.expression();

        self.consume(TokenType::Semicolon, "Expect ';' after statement");

        // TODO: Handle the case to not have an expression ex: var test;
        Stmt::Variable { token, expression }
    }

    fn expression_stmt(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after statement");

        Stmt::Expression(expr)
    }

    fn print_stmt(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after statement");

        Stmt::Print(expr)
    }

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.match_token([TokenType::Equal]) {
            let token = self.previous().clone();
            let value = self.assignment();

            return match expr {
                Expr::Variable { name } => Expr::Assign {
                    name,
                    value: Box::new(value),
                },
                _ => {
                    ErrorMsg::error(&token, "Invalid assignment target");
                    expr
                }
            };
        }

        expr
    }

    // "Logical  : Expr left, Token operator, Expr right",
    fn or(&mut self) -> Expr {
        let expr = self.and();

        if self.match_token([TokenType::Or]) {
            let operator = self.previous().clone();

            let right = Box::new(self.and());

            return Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        expr
    }

    // "Logical  : Expr left, Token operator, Expr right",
    fn and(&mut self) -> Expr {
        let expr = self.equality();

        if self.match_token([TokenType::And]) {
            let operator = self.previous().clone();

            let right = Box::new(self.equality());

            return Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        expr
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
                value: LiteralValue::False,
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

        if self.match_token([TokenType::Identifier]) {
            return Some(Expr::Variable {
                name: self.previous().to_owned(),
            });
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

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Token {
        let token = self.advance().clone();
        if token.token_type != token_type {
            self.error(self.previous(), msg);
        }

        token
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

    fn _synchronize(&mut self) {
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

        println!("{:?}", parser.primary());
    }

    #[test]
    fn unary_test() {
        let text = "!1.9 + 9";
        let mut scanner = Scanner::new(text);
        let tokens = scanner.scan_tokens();

        println!("{tokens:?}");

        let mut parser = Parser::new(tokens.to_vec());

        println!("{:?}", parser.expression());
    }
}
