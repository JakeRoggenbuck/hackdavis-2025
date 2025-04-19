mod ir;
mod lexer;
mod parser;

pub use ir::{Program, Section, Command};
pub use parser::Parser;

pub fn compile(input: String) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse();
    serde_json::to_string_pretty(&program).unwrap()
} 