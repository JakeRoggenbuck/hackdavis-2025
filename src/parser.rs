use crate::ir::{Program, Section, Command};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut sections = Vec::new();
        
        while self.current_token != Token::Eof {
            sections.push(self.parse_section());
        }

        Program { sections }
    }

    fn parse_section(&mut self) -> Section {
        // Expect "section" keyword
        if let Token::Section = self.current_token {
            self.advance();
        } else {
            panic!("Expected 'section' keyword");
        }

        // Get section name
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.advance();
            name
        } else {
            panic!("Expected section name");
        };

        // Expect colon
        if let Token::Colon = self.current_token {
            self.advance();
        } else {
            panic!("Expected ':' after section name");
        }

        let mut commands = Vec::new();
        while self.current_token != Token::Eof && self.current_token != Token::Section {
            commands.push(self.parse_command());
        }

        Section { name, commands }
    }

    fn parse_command(&mut self) -> Command {
        // Expect "mov" keyword
        if let Token::Identifier(ref cmd) = self.current_token {
            if cmd != "mov" {
                panic!("Expected 'mov' command");
            }
            self.advance();
        } else {
            panic!("Expected 'mov' command");
        }

        // Get direction
        let direction = if let Token::Identifier(dir) = self.current_token.clone() {
            self.advance();
            dir
        } else {
            panic!("Expected direction");
        };

        // Expect comma
        if let Token::Comma = self.current_token {
            self.advance();
        } else {
            panic!("Expected ',' after direction");
        }

        // Get amount
        let amount = if let Token::Number(n) = self.current_token.clone() {
            self.advance();
            n
        } else {
            panic!("Expected number");
        };

        Command {
            r#type: direction,
            amount,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_section() {
        let input = r#"
        section start:
            mov forward, 10
            mov backward, 4
        "#.to_string();
        
        let mut parser = Parser::new(input);
        let program = parser.parse();
        
        assert_eq!(program.sections.len(), 1);
        assert_eq!(program.sections[0].name, "start");
        assert_eq!(program.sections[0].commands.len(), 2);
        
        assert_eq!(program.sections[0].commands[0].r#type, "forward");
        assert_eq!(program.sections[0].commands[0].amount, 10);
        
        assert_eq!(program.sections[0].commands[1].r#type, "backward");
        assert_eq!(program.sections[0].commands[1].amount, 4);
    }

    #[test]
    fn test_parse_multiple_sections() {
        let input = r#"
        section first:
            mov forward, 5
        section second:
            mov backward, 3
            mov forward, 2
        "#.to_string();
        
        let mut parser = Parser::new(input);
        let program = parser.parse();
        
        assert_eq!(program.sections.len(), 2);
        
        assert_eq!(program.sections[0].name, "first");
        assert_eq!(program.sections[0].commands.len(), 1);
        assert_eq!(program.sections[0].commands[0].r#type, "forward");
        assert_eq!(program.sections[0].commands[0].amount, 5);
        
        assert_eq!(program.sections[1].name, "second");
        assert_eq!(program.sections[1].commands.len(), 2);
        assert_eq!(program.sections[1].commands[0].r#type, "backward");
        assert_eq!(program.sections[1].commands[0].amount, 3);
        assert_eq!(program.sections[1].commands[1].r#type, "forward");
        assert_eq!(program.sections[1].commands[1].amount, 2);
    }

    #[test]
    #[should_panic(expected = "Expected section name")]
    fn test_parse_invalid_section() {
        let input = "section : mov forward, 10".to_string();
        let mut parser = Parser::new(input);
        parser.parse();
    }

    #[test]
    #[should_panic(expected = "Expected direction")]
    fn test_parse_invalid_command() {
        let input = "section start: mov , 10".to_string();
        let mut parser = Parser::new(input);
        parser.parse();
    }
} 