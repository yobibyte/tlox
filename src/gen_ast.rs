// This file is not used for now. Until I understand what's going on, I'll write everything
// manually. The grammar is small for now.

use std::fs::File;
use std::io::{BufWriter, Write};

pub fn define_ast(output_fpath: String, base_name: String, types: Vec<String>) {
    // The original in the book had an output directory.
    // I think, using a file is better at this point.
    // This might change in the future.
    let path: String = output_fpath.to_string();
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(&file);
    writeln!(writer, "pub struct {base_name} {{").unwrap();
    writeln!(writer, "}}").unwrap();

    for type_str in types {
        let split_str: Vec<&str> = type_str.split(":").collect();
        let struct_name = split_str[0].trim().to_string();
        let fields = split_str[1].trim().to_string();
        define_type(&mut writer, base_name.clone(), struct_name, fields);
    }
}

fn define_type<W: Write>(writer: &mut W, _base_name: String, struct_name: String, fields: String) {
    writeln!(writer, "pub struct {struct_name} {{").unwrap();
    for field in fields.split(", ") {
        let field_str: Vec<&str> = field.split(" ").collect();
        let field_type = field_str[0].to_string();
        let field_name = field_str[1].to_string();
        writeln!(writer, "    {field_name}: {field_type},").unwrap();
    }
    writeln!(writer, "}}").unwrap();
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let output_fpath = args[1].clone();
            println!("Output fpath: {}", output_fpath);

            let grammar: Vec<String> = vec![
                String::from("Binary   : Expr left, Token operator, Expr right"),
                String::from("Grouping : Expr expression"),
                String::from("Literal  : Object value"),
                String::from("Unary    : Token operator, Expr right"),
            ];
            define_ast(output_fpath, String::from("Expr"), grammar);
        }
        _ => println!("Usage: generate_ast [output_file]"),
    }
}
