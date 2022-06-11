use crate::{token::Token, token_type::TokenType};

#[derive(Default)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from(source),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, (usize, String)> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?; //TODO: Do we detect all errors in one go
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            String::new(),
            self.line,
        ));

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), (usize, String)> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => return Err((self.line, "Unexpected character".to_string())),
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;
        self.source.as_bytes()[current] as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, String::from(""));
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
