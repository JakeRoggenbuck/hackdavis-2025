mod codegen;
mod ir;
mod lexer;
mod parser;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Lexer error: {0}")]
    LexerError(String),
    #[error("Parser error: {0}")]
    ParserError(String),
    #[error("Codegen error: {0}")]
    CodegenError(String),
}

pub use codegen::generate_arduino_code;
pub use ir::{Command, Program, Section};
pub use parser::Parser;

/// Compiles the input assembly code to IR (JSON format)
/// 
/// # Arguments
/// 
/// * `input` - The assembly code to compile
/// 
/// # Returns
/// 
/// Returns a JSON string representing the IR, or an error if compilation fails
pub fn compile(input: String) -> Result<String, CompilerError> {
    let mut parser = Parser::new(input);
    let program = parser.parse().map_err(CompilerError::ParserError)?;
    serde_json::to_string_pretty(&program).map_err(|e| CompilerError::CodegenError(e.to_string()))
}

/// Compiles the input assembly code to Arduino C++ code
/// 
/// # Arguments
/// 
/// * `input` - The assembly code to compile
/// 
/// # Returns
/// 
/// Returns the generated Arduino C++ code, or an error if compilation fails
pub fn compile_to_arduino(input: String) -> Result<String, CompilerError> {
    let mut parser = Parser::new(input);
    let program = parser.parse().map_err(CompilerError::ParserError)?;
    generate_arduino_code(&program).map_err(CompilerError::CodegenError)
}
