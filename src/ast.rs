use std::fs::File;
use std::io::{BufWriter, Write};

fn define_ast(output_dir: String, base_name: String, types: Vec<String>) {
    let path: String = format!("{output_dir}/{base_name}.rs");
    for el in types {
        println!("{}", el);
    }
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(&file);
    writeln!(writer, "struct {base_name} {{").unwrap();
    writeln!(writer, "").unwrap();
    writeln!(writer, "}}").unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let output_dir = args[1].clone();
            println!("Output dir: {}", output_dir);

            let grammar: Vec<String> = vec![
                String::from("Binary   : Expr left, Token operator, Expr right"),
                String::from("Grouping : Expr expression"),
                String::from("Literal  : Object value"),
                String::from("Unary    : Token operator, Expr right"),
            ];
            define_ast(output_dir, String::from("Expr"), grammar);
        }
        _ => println!("Usage: generate_ast [output_dir]"),
    }
}
