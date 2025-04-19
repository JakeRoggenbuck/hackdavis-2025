use hackdavis_2025_compiler::{compile, Program};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_compile_roundtrip(
        sections in prop::collection::vec(
            prop::collection::vec(
                (any::<String>().prop_filter("direction must be non-empty", |s| !s.is_empty()),
                 any::<String>().prop_filter("direction must be non-empty", |s| !s.is_empty()),
                 any::<i32>()),
                1..5
            ),
            1..5
        )
    ) {
        // Generate a valid program string
        let mut program_str = String::new();
        for (i, commands) in sections.iter().enumerate() {
            program_str.push_str(&format!("section section{}:\n", i));
            for (direction, _, amount) in commands {
                program_str.push_str(&format!("    mov {}, {}\n", direction, amount));
            }
        }

        // Compile it
        let output = compile(program_str.clone());
        
        // Parse the JSON back
        let parsed: Program = serde_json::from_str(&output).unwrap();
        
        // Verify the structure
        assert_eq!(parsed.sections.len(), sections.len());
        for (i, section) in parsed.sections.iter().enumerate() {
            assert_eq!(section.commands.len(), sections[i].len());
            for (j, command) in section.commands.iter().enumerate() {
                assert_eq!(command.r#type, sections[i][j].0);
                assert_eq!(command.amount, sections[i][j].2);
            }
        }
    }

    #[test]
    fn test_compile_whitespace_insensitive(
        sections in prop::collection::vec(
            prop::collection::vec(
                (any::<String>().prop_filter("direction must be non-empty", |s| !s.is_empty()),
                 any::<String>().prop_filter("direction must be non-empty", |s| !s.is_empty()),
                 any::<i32>()),
                1..5
            ),
            1..5
        ),
        whitespace in prop::collection::vec(
            prop::string::string_regex(r"[ \t\n]*").unwrap(),
            1..10
        )
    ) {
        // Generate a valid program string with random whitespace
        let mut program_str = String::new();
        for (i, commands) in sections.iter().enumerate() {
            program_str.push_str(&whitespace[i % whitespace.len()]);
            program_str.push_str(&format!("section section{}:", i));
            program_str.push_str(&whitespace[i % whitespace.len()]);
            program_str.push('\n');
            for (direction, _, amount) in commands {
                program_str.push_str(&whitespace[i % whitespace.len()]);
                program_str.push_str(&format!("mov {}, {}", direction, amount));
                program_str.push_str(&whitespace[i % whitespace.len()]);
                program_str.push('\n');
            }
        }

        // Compile it
        let output = compile(program_str.clone());
        
        // Parse the JSON back
        let parsed: Program = serde_json::from_str(&output).unwrap();
        
        // Verify the structure
        assert_eq!(parsed.sections.len(), sections.len());
        for (i, section) in parsed.sections.iter().enumerate() {
            assert_eq!(section.commands.len(), sections[i].len());
            for (j, command) in section.commands.iter().enumerate() {
                assert_eq!(command.r#type, sections[i][j].0);
                assert_eq!(command.amount, sections[i][j].2);
            }
        }
    }
} 