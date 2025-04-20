use compiler::{compile, compile_to_arduino};

#[test]
fn test_compile_simple_program() {
    let input = r#"
    circle:
        mov direction, 1
        mov forward, 4
        mov direction, 0
    "#
    .to_string();

    let output = compile(input).unwrap();
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
        mov forward, 4
        jal circle
    "#
    .to_string();

    let output = compile_to_arduino(input).unwrap();
    assert!(output.contains("void circle()"));
    assert!(output.contains("void main_loop()"));
    assert!(output.contains("forward(4)"));
    assert!(output.contains("circle();"));
}

#[test]
fn test_compile_multiple_sections() {
    let input = r#"
    section1:
        mov forward, 1
        mov direction, 1

    section2:
        mov backward, 2
        mov direction, 2

    main:
        jal section1
        jal section2
    "#
    .to_string();

    let output = compile(input).unwrap();
    let expected = r#"{
  "sections": [
    {
      "name": "section1",
      "commands": [
        {
          "Move": {
            "type": "forward",
            "amount": 1
          }
        },
        {
          "Move": {
            "type": "direction",
            "amount": 1
          }
        }
      ]
    },
    {
      "name": "section2",
      "commands": [
        {
          "Move": {
            "type": "backward",
            "amount": 2
          }
        },
        {
          "Move": {
            "type": "direction",
            "amount": 2
          }
        }
      ]
    },
    {
      "name": "main",
      "commands": [
        {
          "Jump": {
            "label": "section1"
          }
        },
        {
          "Jump": {
            "label": "section2"
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

    let output = compile(input).unwrap();
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
fn test_compile_invalid_syntax() {
    let input = r#"
    invalid:
        mov direction 1  # Missing comma
    "#
    .to_string();

    let result = compile(input);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Expected ',' after direction"));
}
