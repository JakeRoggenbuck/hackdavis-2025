use hackdavis_2025_compiler::compile;

#[test]
fn test_compile_simple_program() {
    let input = r#"
    section start:
        mov forward, 10
        mov backward, 4
    "#.to_string();

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
fn test_compile_multiple_sections() {
    let input = r#"
    section first:
        mov forward, 5
    section second:
        mov backward, 3
        mov forward, 2
    "#.to_string();

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
    "#.to_string();

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
    "#.to_string();

    compile(input);
} 