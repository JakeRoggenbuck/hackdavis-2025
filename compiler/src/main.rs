use hackdavis_2025_compiler::{compile, compile_to_arduino};

fn main() {
    let input = r#"
section start:
    mov forward, 10
    mov backward, 4
    mov direction, 1
    mov straight, 0
    mov wait, 2

section two:
    mov forward, 7
    mov direction, 2
"#.to_string();

    println!("IR Output:");
    let ir_output = compile(input.clone());
    println!("{}", ir_output);

    println!("\nArduino Output:");
    let arduino_output = compile_to_arduino(input);
    println!("{}", arduino_output);
}
