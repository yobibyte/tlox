use crate::ast::Expr;
use crate::scanner::{LiteralType, Token};
use crate::types::TokenType;
use std::io;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        while self.match_token_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        if self.match_token_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        //TODO: how do we get the line at this point?
        if self.match_token_type(TokenType::False) {
            return Expr::Literal(LiteralType::Bool(false));
        }
        if self.match_token_type(TokenType::True) {
            return Expr::Literal(LiteralType::Bool(false));
        }
        if self.match_token_type(TokenType::Nil) {
            return Expr::Literal(LiteralType::Null);
        }
        if self.match_token_types(vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal(self.previous().literal);
        }
        if self.match_token_type(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            );
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        panic!("We should not be here, all the tokens should have been exhausted.");
    }
    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if !self.is_at_end() && self.peek().ttype == token_type {
            return self.advance();
        }
        //TODO: get rid of this ugliness and use Result instead.
        panic!("{} {}", self.peek().to_string(), message);
    }
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        return self.peek().ttype == TokenType::Eof;
    }
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn match_token_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.match_token_type(token_type) {
                return true;
            }
        }
        false
    }
    fn match_token_type(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.peek().ttype == token_type {
            true => {
                self.advance();
                true
            }
            false => false,
        }
    }
    fn parse(&self) -> Result<Expr, io::Error> {
        //TODO this should not be io::Error, define our own error.
        let expr = self.expression()?;
        Ok(expr)
    }
}
