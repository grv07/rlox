use crate::{expr::Expr, token::Token, token_type::TokenType};

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

    fn unary() -> Expr {
        todo!()
    }

    fn primary() -> Expr {
        todo!()
    }

    fn match_token(&mut self, token_types: impl IntoIterator<Item = TokenType>) -> bool {
        if token_types.into_iter().any(|tt| self.check(tt)) {
            self.advance();
            return true;
        }
        false
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
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
