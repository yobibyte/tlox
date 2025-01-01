use std::fmt;

use crate::types::{TokenType, KEYWORDS};

pub enum LiteralType {
    Str(String),
    Num(f64),
    Null,
}
impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::Str(s) => write!(f, "{s}"),
            LiteralType::Num(n) => write!(f, "{n}"),
            LiteralType::Null => write!(f, ""),
        }
    }
}

pub struct Scanner<'a> {
    // I use to have source: &str here before.
    // At this point it seems easier to store source as array of chars instead of just a &str.
    // String indexing is hard, and iterating over a .chars().ith(i) iterator sounds crazy.
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_handler: &'a mut ErrorHandler,
}
impl<'a> Scanner<'a> {
    pub fn new(source: &str, error_handler: &'a mut ErrorHandler) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
            error_handler,
        }
    }
    fn char_at(&self, index: usize) -> char {
        self.source[index]
    }
    fn advance(&mut self) -> char {
        let res = self.char_at(self.current);
        self.current += 1;
        res
    }
    fn add_token_wo_literal(&mut self, ttype: TokenType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, text, LiteralType::Null, self.line));
    }
    fn add_token(&mut self, ttype: TokenType, literal: LiteralType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, text, literal, self.line));
    }
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token_wo_literal(TokenType::LeftParen),
            ')' => self.add_token_wo_literal(TokenType::RightParen),
            '{' => self.add_token_wo_literal(TokenType::LeftBrace),
            '}' => self.add_token_wo_literal(TokenType::RightBrace),
            ',' => self.add_token_wo_literal(TokenType::Comma),
            '.' => self.add_token_wo_literal(TokenType::Dot),
            '-' => self.add_token_wo_literal(TokenType::Minus),
            '+' => self.add_token_wo_literal(TokenType::Plus),
            ';' => self.add_token_wo_literal(TokenType::Semicolon),
            '*' => self.add_token_wo_literal(TokenType::Star),
            '!' => {
                let matched_equal: bool = self.cond_match('=');
                self.add_token_wo_literal(if matched_equal {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
            }
            '=' => {
                let matched_equal: bool = self.cond_match('=');
                self.add_token_wo_literal(if matched_equal {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
            }
            '<' => {
                let matched_equal: bool = self.cond_match('=');
                self.add_token_wo_literal(if matched_equal {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                });
            }
            '>' => {
                let matched_equal: bool = self.cond_match('=');
                self.add_token_wo_literal(if matched_equal {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                });
            }
            '/' => {
                let matched_slash: bool = self.cond_match('/');
                if matched_slash {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_wo_literal(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.process_string(),
            c if c.is_ascii_digit() => self.process_number(),
            c if c == 'o' && self.cond_match('r') => {
                self.add_token_wo_literal(TokenType::Or);
            }
            c if c.is_ascii_alphabetic() || c == '_' => self.process_identifier(),
            _ => self.error_handler.error(self.line, "Unexpected character."),
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.char_at(self.current)
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.char_at(self.current + 1)
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token_wo_literal(TokenType::Eof);
    }
    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }
    fn cond_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }
    fn process_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error_handler.error(self.line, "Unterminated string.");
            return;
        }

        // Go beyond closing quote "
        // current is post "string"x
        //_________________________^
        self.advance();

        // Trim quotes from the string.
        // We are post quote now, and right index is exclusive -> current-1.
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::String, LiteralType::Str(value));
    }
    fn process_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        // fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            //consume the dot
            self.advance();
        }
        // get the post dot digits
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        // TODO: do error handling with the handler. Proceed further. Add a NULL token mb?
        let value: f64 = value
            .to_string()
            .parse()
            .expect("Could not parse a double.");
        self.add_token(TokenType::Number, LiteralType::Num(value));
    }
    fn process_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let ttype: TokenType = match KEYWORDS.get(&text) {
            Some(t) => *t,
            None => TokenType::Identifier,
        };
        self.add_token_wo_literal(ttype);
    }
}

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: LiteralType,
    line: usize,
}
impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: LiteralType, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.lexeme, self.literal)
    }
}
#[derive(Default)]
pub struct ErrorHandler {
    pub has_error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
    fn report(&mut self, line: usize, loc: &str, message: &str) {
        eprintln!("[line {line}] Error {loc}: {message}.");
        self.has_error = true;
    }
    pub fn reset(&mut self) {
        self.has_error = false;
    }
}
