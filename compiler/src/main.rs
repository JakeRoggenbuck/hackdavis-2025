use hackdavis_2025_compiler::{compile, compile_to_arduino};

fn main() {
    let input = r#"
circle:
    mov direction, 1
    mov forward, 4
    mov direction, 0

main:
    jal circle
    jal circle
    mov forward, 10
    jal circle
"#.to_string();

    println!("IR Output:");
    let ir_output = compile(input.clone());
    println!("{}", ir_output);

    println!("\nArduino Output:");
    let arduino_output = compile_to_arduino(input);
    println!("{}", arduino_output);
}
