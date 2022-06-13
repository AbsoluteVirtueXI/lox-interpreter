use crate::{token::Token, token_type::TokenType};
use std::collections::HashMap;

#[derive(Default)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Self {
            keywords,
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
            '!' => {
                let is_ok = self.is_next('=');
                self.add_token(if is_ok {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let is_ok = self.is_next('=');
                self.add_token(if is_ok {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let is_ok = self.is_next('=');
                self.add_token(if is_ok {
                    TokenType::LesserEqual
                } else {
                    TokenType::Lesser
                })
            }
            '>' => {
                let is_ok = self.is_next('=');
                self.add_token(if is_ok {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                let is_ok = self.is_next('/');
                if is_ok {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {
                // ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => self.scan_string()?,

            _ => {
                if c.is_numeric() {
                    self.scan_number()?;
                } else if is_identifier_first_char(c) {
                    self.scan_identifier();
                } else {
                    return Err((self.line, "Unexpected character".to_string()));
                }
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;
        self.source.as_bytes()[current] as char
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current] as char
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.as_bytes()[self.current + 1] as char
        }
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

    fn is_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn scan_string(&mut self) -> Result<(), (usize, String)> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err((self.line, "Unterminated string.".to_string()));
        }

        self.advance();
        let text = &self.source[self.start + 1..self.current - 1];
        self.add_token_with_literal(TokenType::String, String::from(text));
        Ok(())
    }

    fn scan_number(&mut self) -> Result<(), (usize, String)> {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            // consume the "."
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.add_token_with_literal(
            TokenType::Number,
            self.source[self.start..self.current].to_string(), // TODO: maybe we can get a number directly here
        );

        Ok(())
    }

    fn scan_identifier(&mut self) -> Result<(), (usize, String)> {
        while is_identifier_char(self.peek()) {
            self.advance();
        }
        let text = String::from(&self.source[self.start..self.current]);
        let token_type = match self.keywords.get(&text) {
            Some(token_type) => *token_type,
            None => TokenType::Identifier,
        };
        self.add_token(token_type);
        Ok(())
    }
}

fn is_identifier_first_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_char(c: char) -> bool {
    is_identifier_first_char(c) || c.is_numeric()
}
