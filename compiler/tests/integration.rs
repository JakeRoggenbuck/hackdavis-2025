use hackdavis_2025_compiler::{compile, compile_to_arduino};

#[test]
fn test_compile_simple_program() {
    let input = r#"
    section start:
        mov forward, 10
        mov backward, 4
    "#
    .to_string();

    let output = compile(input);
    let expected = r#"{
  "sections": [
    {
      "name": "start",
      "commands": [
        {
          "type": "forward",
          "amount": 10
        },
        {
          "type": "backward",
          "amount": 4
        }
      ]
    }
  ]
}"#;

    assert_eq!(output, expected);
}

#[test]
fn test_compile_to_arduino() {
    let input = r#"
    section start:
        mov forward, 10
        mov backward, 4
        mov direction, 1
        mov straight, 0
        mov wait, 2
    "#.to_string();

    let output = compile_to_arduino(input);
    
    // Check for key components in the generated Arduino code
    assert!(output.contains("int enA = 9;"));
    assert!(output.contains("int in1 = 3;"));
    assert!(output.contains("void setup()"));
    assert!(output.contains("void loop()"));
    assert!(output.contains("forward(10);"));
    assert!(output.contains("backwards(4);"));
    assert!(output.contains("left();"));
    assert!(output.contains("straight();"));
    assert!(output.contains("wait(2);"));
    assert!(output.contains("void forward(float time)"));
    assert!(output.contains("void backwards(float time)"));
    assert!(output.contains("void left()"));
    assert!(output.contains("void straight()"));
    assert!(output.contains("void wait(float time)"));
}

#[test]
fn test_compile_multiple_sections() {
    let input = r#"
    section first:
        mov forward, 5
    section second:
        mov backward, 3
        mov forward, 2
    "#
    .to_string();

    let output = compile(input);
    let expected = r#"{
  "sections": [
    {
      "name": "first",
      "commands": [
        {
          "type": "forward",
          "amount": 5
        }
      ]
    },
    {
      "name": "second",
      "commands": [
        {
          "type": "backward",
          "amount": 3
        },
        {
          "type": "forward",
          "amount": 2
        }
      ]
    }
  ]
}"#;

    assert_eq!(output, expected);
}

#[test]
fn test_compile_empty_section() {
    let input = r#"
    section empty:
    "#
    .to_string();

    let output = compile(input);
    let expected = r#"{
  "sections": [
    {
      "name": "empty",
      "commands": []
    }
  ]
}"#;

    assert_eq!(output, expected);
}

#[test]
#[should_panic]
fn test_compile_invalid_syntax() {
    let input = r#"
    section invalid:
        mov forward 10  # Missing comma
    "#
    .to_string();

    compile(input);
}
