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

    EOF,
}

struct Token {
    ttype: TokenType,
    lexeme: String,
    // literal: Object
    line: usize,
}
impl Token {
    fn new(ttype: TokenType, lexeme: String, line: usize) -> Token {
        //TODO: make this a parameter.
        Token {
            ttype,
            lexeme,
            line,
        }
    }
}

struct Scanner {
    // I use to have source: &str here before.
    // At this point it seems easier to store source as array of chars instead of just a &str.
    // String indexing is hard, and iterating over a .chars().ith(i) iterator sounds crazy.
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
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
    fn add_token(&mut self, ttype: TokenType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(ttype, text, self.line));
    }
    fn scan_token(&mut self) {
        let c: char = self.advance();
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
            _ => println!("UNK"), //TODO: remove? add error handling?
        }
    }
    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));
    }
    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }
}

pub struct ErrorHandler {
    pub has_error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { has_error: false }
    }
    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }
    fn report(&mut self, line: u32, loc: &str, message: &str) {
        eprintln!("[line {line}] Error {loc}: {message}.");
        self.has_error = true;
    }
    pub fn reset(&mut self) {
        self.has_error = false;
    }
}
pub fn run(line: &str, _err_handler: &ErrorHandler) {
    let mut scanner = Scanner::new(line);
    scanner.scan_tokens();
    for tok in scanner.tokens {
        println!("{}", tok.lexeme);
    }
}
