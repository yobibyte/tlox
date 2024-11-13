use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
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

pub static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("and".to_string(), TokenType::And);
    m.insert("class".to_string(), TokenType::Class);
    m.insert("else".to_string(), TokenType::Else);
    m.insert("false".to_string(), TokenType::False);
    m.insert("for".to_string(), TokenType::For);
    m.insert("fun".to_string(), TokenType::Fun);
    m.insert("if".to_string(), TokenType::If);
    m.insert("nil".to_string(), TokenType::Nil);
    m.insert("or".to_string(), TokenType::Or);
    m.insert("print".to_string(), TokenType::Print);
    m.insert("return".to_string(), TokenType::Return);
    m.insert("super".to_string(), TokenType::Super);
    m.insert("this".to_string(), TokenType::This);
    m.insert("true".to_string(), TokenType::True);
    m.insert("var".to_string(), TokenType::Var);
    m.insert("while".to_string(), TokenType::While);
    m
});
