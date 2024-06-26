mod lexer;
mod parser;
mod ast;
mod compiler;
mod cli;

use cli::compile;

fn main() {
    compile();
}
