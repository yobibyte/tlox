fn run_file() {
    println!("Run file.")
}
fn run_prompt() {
    println!("Run prompt.")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        // 0th arg is always the program name.
        1 => run_prompt(),
        2 => run_file(),
        _ => println!("Usage: tlox [script]"),
    }
}
