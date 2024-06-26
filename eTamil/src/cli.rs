use std::fs;
use crate::parser::parse;
use crate::compiler::compile_ast;

pub fn compile() {
    let code = fs::read_to_string("input.etamil").expect("Unable to read file");
    let ast = parse(&code).expect("Parsing failed");
    compile_ast(ast);
}
