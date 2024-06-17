use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
    True,
    False,
    Nil,
}

impl ToString for LiteralValue {
    fn to_string(&self) -> String {
        match self {
            LiteralValue::IntValue(i) => i.to_string(),
            LiteralValue::FValue(f) => f.to_string(),
            LiteralValue::StringValue(s) => s.to_string(),
            LiteralValue::IdentifierValue(i) => i.to_string(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<LiteralValue>,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        literal: Option<LiteralValue>,
        lexeme: String,
        line: usize,
    ) -> Token {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
