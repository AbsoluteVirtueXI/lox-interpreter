#[derive(Debug)]
pub struct Token {}

#[derive(Default)]
pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan_tokens(&self, source: &str) -> Vec<Token> {
        vec![Token {}]
    }
}
