use std::fs;

fn run_file(filepath: &str) {
    let fcontent = fs::read_to_string(filepath).expect("Could not load a file {filepath}");
    println!("{fcontent}");
}
fn run_prompt() {
    println!("Run prompt.")
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
