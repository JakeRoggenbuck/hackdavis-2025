use hackdavis_2025_compiler::compile;

fn main() {
    let input = r#"
section start:
    mov forward, 10
    mov backward, 4

section two:
    mov forward, 7
"#.to_string();

    let output = compile(input);
    println!("{}", output);
}
