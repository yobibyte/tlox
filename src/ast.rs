 // defineAst(outputDir, "Expr", Arrays.asList(
 //      "Binary   : Expr left, Token operator, Expr right",
 //      "Grouping : Expr expression",
 //      "Literal  : Object value",
 //      "Unary    : Token operator, Expr right"
 //    ));
 //
 //     private static void defineAst(
  //     String outputDir, String baseName, List<String> types)
  //     throws IOException {
  //   String path = outputDir + "/" + baseName + ".java";
  //   PrintWriter writer = new PrintWriter(path, "UTF-8");
  //
  //   writer.println("package com.craftinginterpreters.lox;");
  //   writer.println();
  //   writer.println("import java.util.List;");
  //   writer.println();
  //   writer.println("abstract class " + baseName + " {");
  //
  //   writer.println("}");
  //   writer.close();
  // }
  //


fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        // 0th arg is always the program name.
        2 => println!("{}", args[1]),
        _ => println!("Usage: generate_ast [script]"),
    }
}
