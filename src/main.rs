use std::fs;
use std::io;
use std::io::Write;
use std::process;

static INPUT_RELATED_EXIT_CODE: i32 = 65;

fn run_file(filepath: &str) {
    let fcontent = fs::read_to_string(filepath).expect("Could not load a file {filepath}");
    let err_handler = tlox::ErrorHandler::new();
    tlox::run(&fcontent, &err_handler);
    if err_handler.has_error {
        process::exit(INPUT_RELATED_EXIT_CODE);
    }
}

fn run_prompt() {
    let mut err_handler = tlox::ErrorHandler::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush.");
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut buf)
            .expect("Couldn't parse a line: {line}");
        if buf.trim().is_empty() {
            break;
        }
        tlox::run(&buf, &err_handler);
        err_handler.reset();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        // 0th arg is always the program name.
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Usage: tlox [script]"),
    }
}
