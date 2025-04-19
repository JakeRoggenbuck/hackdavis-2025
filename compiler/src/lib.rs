mod ir;
mod lexer;
mod parser;
mod codegen;

pub use ir::{Program, Section, Command};
pub use parser::Parser;
pub use codegen::generate_arduino_code;

pub fn compile(input: String) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse();
    serde_json::to_string_pretty(&program).unwrap()
}

pub fn compile_to_arduino(input: String) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse();
    generate_arduino_code(&program)
} 