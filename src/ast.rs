use core::fmt;
// In the book, they generate this automatically.
// I will do it manually until I understand what's going on.
use crate::scanner::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Expr::Unary { operator, right } => write!(f, "({operator} {right})"),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({operator} {left} {right})")
            }
            // TODO: the book prints nil if literal is null, how will we do it?
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Grouping { expression } => write!(f, "(group {expression})"),
        }
    }
}

#[cfg(test)]
mod dests {
    use super::*;
    use crate::scanner::{LiteralType, Numeric};
    use crate::types::TokenType;

    #[test]
    fn test_expr_display() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), LiteralType::Null, 1),
                right: Box::new(Expr::Literal {
                    value: Token::new(
                        TokenType::Number,
                        "123".to_string(),
                        LiteralType::Num(Numeric::Integer(123)),
                        1,
                    ),
                }),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), LiteralType::Null, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Token::new(
                        TokenType::Number,
                        "45.67".to_string(),
                        LiteralType::Num(Numeric::Float(45.67)),
                        1,
                    ),
                }),
            }),
        };
        let res = format!("{expression}");
        println!("{res}");
        assert!(format!("{expression}") == "(* (- 123) (group 45.67))");
    }
}
