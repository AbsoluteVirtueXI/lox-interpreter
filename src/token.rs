use crate::token_type::*;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
