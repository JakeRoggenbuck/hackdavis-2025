use hackdavis_2025_compiler::{compile, compile_to_arduino};

#[test]
fn test_compile_simple_program() {
    let input = r#"
    circle:
        mov direction, 1
        mov forward, 4
        mov direction, 0
    "#
    .to_string();

    let output = compile(input);
    let expected = r#"{
  "sections": [
    {
      "name": "circle",
      "commands": [
        {
          "Move": {
            "type": "direction",
            "amount": 1
          }
        },
        {
          "Move": {
            "type": "forward",
            "amount": 4
          }
        },
        {
          "Move": {
            "type": "direction",
            "amount": 0
          }
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
    circle:
        mov direction, 1
        mov forward, 4
        mov direction, 0

    main:
        jal circle
        jal circle
        mov forward, 10
        jal circle
    "#
    .to_string();

    let output = compile_to_arduino(input);

    // Check for key components in the generated Arduino code
    assert!(output.contains("int enA = 9;"));
    assert!(output.contains("int in1 = 3;"));
    assert!(output.contains("void setup()"));
    assert!(output.contains("void loop()"));
    assert!(output.contains("void circle()"));
    assert!(output.contains("void main_loop()"));
    assert!(output.contains("left();"));
    assert!(output.contains("forward(4);"));
    assert!(output.contains("straight();"));
    assert!(output.contains("forward(10);"));
    assert!(output.contains("void forward(float time)"));
    assert!(output.contains("void backwards(float time)"));
    assert!(output.contains("void left()"));
    assert!(output.contains("void straight()"));
    assert!(output.contains("void wait(float time)"));
}

#[test]
fn test_compile_multiple_sections() {
    let input = r#"
    circle:
        mov direction, 1
        mov forward, 4
        mov direction, 0

    main:
        jal circle
        mov forward, 10
        jal circle
    "#
    .to_string();

    let output = compile(input);
    let expected = r#"{
  "sections": [
    {
      "name": "circle",
      "commands": [
        {
          "Move": {
            "type": "direction",
            "amount": 1
          }
        },
        {
          "Move": {
            "type": "forward",
            "amount": 4
          }
        },
        {
          "Move": {
            "type": "direction",
            "amount": 0
          }
        }
      ]
    },
    {
      "name": "main",
      "commands": [
        {
          "Jump": {
            "label": "circle"
          }
        },
        {
          "Move": {
            "type": "forward",
            "amount": 10
          }
        },
        {
          "Jump": {
            "label": "circle"
          }
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
    empty:
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
    invalid:
        mov forward 10  # Missing comma
    "#
    .to_string();

    compile(input);
}
