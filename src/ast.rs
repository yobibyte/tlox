use std::fs::File;
use std::io::{BufWriter, Write};

fn define_ast(output_dir: String, base_name: String, types: Vec<String>) {
    let path: String = format!("{output_dir}/{base_name}.rs");
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(&file);
    writeln!(writer, "trait {base_name} {{").unwrap();
    writeln!(writer, "").unwrap();
    writeln!(writer, "}}").unwrap();

    for type_str in types {
        let split_str: Vec<&str> = type_str.split(":").collect();
        let struct_name = split_str[0].trim().to_string();
        let fields = split_str[1].trim().to_string();
        define_type(&mut writer, base_name.clone(), struct_name, fields);
    }
}

fn define_type<W: Write>(writer: &mut W, base_name: String, struct_name: String, fields: String) {
    writeln!(writer, "pub struct {struct_name} impl {base_name} {{").unwrap();
    for field in fields.split(", ") {
        let field_str: Vec<&str> = field.split(" ").collect();
        let field_type = field_str[0].to_string();
        let field_name = field_str[1].to_string();
        writeln!(writer, "    {field_name}: {field_type},").unwrap();
    }
    writeln!(writer, "}}").unwrap();
    writeln!(writer, "").unwrap();
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
