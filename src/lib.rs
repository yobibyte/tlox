struct Token<'a> {
    content: &'a str,
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
}
impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::<Token>::new(),
        }
    }
    fn scan_tokens(&mut self) {
        // Dummy implementation, every token is a char for now.
        let mut prev_idx = 0;
        for (idx, _) in self.source.char_indices() {
            if prev_idx < idx {
                self.tokens.push(Token {
                    content: &self.source[prev_idx..idx],
                });
            }
            prev_idx = idx;
        }
    }
}

pub fn run(line: &str) {
    let mut scanner = Scanner::new(line);
    scanner.scan_tokens();
    for tok in scanner.tokens {
        println!("{}", tok.content);
    }
}
