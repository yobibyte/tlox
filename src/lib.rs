pub mod scanner;

use scanner::{ErrorHandler, Scanner};

pub fn run(line: &str, err_handler: &mut ErrorHandler) {
    let mut scanner = Scanner::new(line, err_handler);
    scanner.scan_tokens();
    for tok in scanner.tokens {
        println!("{}", tok);
    }
}
