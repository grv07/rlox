use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    token::{LiteralValue, Token},
    token_type::TokenType,
    ErrorMsg,
};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        HashMap::from_iter([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    };
}

#[derive(Default)]
pub struct Scanner<'a> {
    source: &'a str,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            ..Default::default()
        }
    }

    fn add_token(&mut self, token: TokenType, literal: Option<LiteralValue>) {
        let lexeme = &self.source.as_bytes()[self.start..self.current];

        let t = Token::new(
            token,
            literal,
            String::from_utf8(lexeme.to_vec()).unwrap(),
            self.line,
        );

        self.tokens.push(t)
    }

    // fn add_token(&mut self, token: TokenType, literal: Option<LiteralValue>) {}

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, None, "".to_string(), self.line));

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            ',' => self.add_token(TokenType::Comma, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, None)
                } else {
                    self.add_token(TokenType::Bang, None)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, None)
                } else {
                    self.add_token(TokenType::Less, None)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual, None)
                } else {
                    self.add_token(TokenType::Greater, None)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, None)
                } else {
                    self.add_token(TokenType::Equal, None)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.handle_string(),
            '0'..='9' => {
                self.number();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.identifier();
            }

            _ => {
                // ErrorMsg::error(self.line, &format!("Unexpected char >> {}", c));
                println!("Line {} Unexpected char >> {}", self.line, c);
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let lit_text = &self.source[self.start..self.current];

        let token_type = KEYWORDS
            .get(lit_text)
            .unwrap_or(&TokenType::Identifier)
            .clone();

        self.add_token(
            token_type,
            Some(LiteralValue::IdentifierValue(lit_text.to_string())),
        );
    }

    fn number(&mut self) {
        let mut is_float = false;

        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_nth(self.current + 1)) {
            is_float = true;
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }

        let number = &self.source[self.start..self.current];
        if is_float {
            let lit_type = LiteralValue::FValue(number.parse::<f64>().unwrap());
            self.add_token(TokenType::FNumber, Some(lit_type))
        } else {
            let lit_type = LiteralValue::IntValue(number.parse::<i64>().unwrap());
            self.add_token(TokenType::INumber, Some(lit_type))
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.char_at(self.current)
    }

    fn peek_nth(&self, n: usize) -> char {
        if self.source.len() <= n {
            return '\0';
        }

        self.char_at(n)
    }

    fn match_char(&mut self, expect: char) -> bool {
        if self.is_at_end() || self.char_at(self.current) != expect {
            return false;
        }

        self.current += 1;

        true
    }

    fn char_at(&self, index: usize) -> char {
        self.source.as_bytes()[index] as char
    }

    fn advance(&mut self) -> char {
        let c = self.char_at(self.current);
        // self.source.as_bytes()[self.current];

        self.current += 1;

        c as char
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            ErrorMsg::report(self.line, "", "Unterminated string");
        }

        self.advance();

        let sub_str = &self.source[self.start + 1..self.current - 1];
        self.add_token(
            TokenType::String,
            Some(LiteralValue::StringValue(sub_str.to_string())),
        )
    }
}
