enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

enum LiteralType {
    Str(String),
    Num(f64),
    Null,
}

struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: LiteralType,
    line: usize,
}
impl Token {
    fn new(ttype: TokenType, lexeme: String, literal: LiteralType, line: usize) -> Token {
        //TODO: make this a parameter.
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}

struct Scanner<'a> {
    // I use to have source: &str here before.
    // At this point it seems easier to store source as array of chars instead of just a &str.
    // String indexing is hard, and iterating over a .chars().ith(i) iterator sounds crazy.
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error_handler: &'a mut ErrorHandler,
}
impl<'a> Scanner<'a> {
    fn new(source: &str, error_handler: &'a mut ErrorHandler) -> Self {
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
            c if c.is_digit(10) => self.process_number(),
            // TODO check why we get 'unexpected character' error at '()' code. Is it \n symbol?
            _ => self.error_handler.error(self.line, "Unexpected character."),
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'; //double check that this is correct one in rust @critical
        }
        self.char_at(self.current)
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0'; //double check that this is correct one in rust @critical
        }
        self.char_at(self.current + 1)
    }
    fn scan_tokens(&mut self) {
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

        // Take the closing quote "
        self.advance();

        // Trim quotes from the string.
        // TODO shouldn't we set the start properly?
        // TODO check that we take the last char of a string properly. @critical
        let value: String = self.source[self.start + 1..self.current].iter().collect();
        self.add_token(TokenType::String, LiteralType::Str(value));
    }
    fn process_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        // fractional part
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            //consume the dot
            self.advance();
        }
        // get the post dot digits
        while self.peek().is_digit(10) {
            self.advance();
        }

        // TODO: check that we take the last digit due to boundary exclusion. @critical
        let value: String = self.source[self.start..self.current].iter().collect();
        // TODO: do error handling with the handler. Proceed further. Add a NULL token mb?
        let value: f64 = value
            .to_string()
            .parse()
            .expect("Could not parse a double.");
        self.add_token(TokenType::Number, LiteralType::Num(value));
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
pub fn run(line: &str, err_handler: &mut ErrorHandler) {
    let mut scanner = Scanner::new(line, err_handler);
    scanner.scan_tokens();
    for tok in scanner.tokens {
        println!("{}", tok.lexeme);
    }
}
