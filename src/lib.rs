pub mod ast;
pub mod gen_ast;
pub mod parser;
pub mod scanner;
pub mod types;

use ast::Expr;
use parser::Parser;
use scanner::{ErrorHandler, Scanner};

pub fn run(line: &str, err_handler: &mut ErrorHandler) {
    let mut scanner = Scanner::new(line, err_handler);
    scanner.scan_tokens();

    let parser: Parser = Parser::new(scanner.tokens);
    //TODO: make this a bit more useful.
    let expression: Expr = parser.parse().expect("Parser encountered errors!");

    println!("{}", expression);
}
