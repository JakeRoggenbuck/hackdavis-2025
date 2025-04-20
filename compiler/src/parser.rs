use crate::ir::{Command, Program, Section};
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

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut sections = Vec::new();

        while self.current_token != Token::Eof {
            sections.push(self.parse_section()?);
        }

        Ok(Program { sections })
    }

    fn parse_section(&mut self) -> Result<Section, String> {
        // Get section name
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.advance();
            name
        } else {
            return Err("Expected section name".to_string());
        };

        // Expect colon
        if let Token::Colon = self.current_token {
            self.advance();
        } else {
            return Err("Expected ':' after section name".to_string());
        }

        let mut commands = Vec::new();
        while self.current_token != Token::Eof {
            // Check if we've reached a new section
            if let Token::Identifier(_) = self.current_token {
                if self.peek_next_token() == Token::Colon {
                    break;
                }
            }
            commands.push(self.parse_command()?);
        }

        Ok(Section { name, commands })
    }

    fn parse_command(&mut self) -> Result<Command, String> {
        // Check if it's a jump instruction
        if let Token::Identifier(ref cmd) = self.current_token {
            if cmd == "jal" {
                self.advance();
                // Get label name
                let label = if let Token::Identifier(label) = self.current_token.clone() {
                    self.advance();
                    label
                } else {
                    return Err("Expected label after jal".to_string());
                };
                return Ok(Command::Jump { label });
            }
        }

        // Otherwise it's a mov command
        if let Token::Identifier(ref cmd) = self.current_token {
            if cmd != "mov" {
                return Err("Expected 'mov' command or 'jal'".to_string());
            }
            self.advance();
        } else {
            return Err("Expected 'mov' command or 'jal'".to_string());
        }

        // Get direction
        let direction = if let Token::Identifier(dir) = self.current_token.clone() {
            self.advance();
            dir
        } else {
            return Err("Expected direction".to_string());
        };

        // Expect comma
        if let Token::Comma = self.current_token {
            self.advance();
        } else {
            return Err("Expected ',' after direction".to_string());
        }

        // Get amount
        let amount = if let Token::Number(n) = self.current_token.clone() {
            self.advance();
            n
        } else {
            return Err("Expected number".to_string());
        };

        Ok(Command::Move {
            r#type: direction,
            amount,
        })
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn peek_next_token(&self) -> Token {
        self.lexer.peek_next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_section() {
        let input = r#"
        circle:
            mov direction, 1
            mov forward, 4
            mov direction, 0
        "#
        .to_string();

        let mut parser = Parser::new(input);
        let program = parser.parse().unwrap();

        assert_eq!(program.sections.len(), 1);
        assert_eq!(program.sections[0].name, "circle");
        assert_eq!(program.sections[0].commands.len(), 3);

        if let Command::Move { r#type, amount } = &program.sections[0].commands[0] {
            assert_eq!(r#type, "direction");
            assert_eq!(*amount, 1);
        } else {
            panic!("Expected Move command");
        }
    }

    #[test]
    fn test_parse_jump() {
        let input = r#"
        main:
            jal circle
            mov forward, 10
        "#
        .to_string();

        let mut parser = Parser::new(input);
        let program = parser.parse().unwrap();

        assert_eq!(program.sections.len(), 1);
        assert_eq!(program.sections[0].name, "main");
        assert_eq!(program.sections[0].commands.len(), 2);

        if let Command::Jump { label } = &program.sections[0].commands[0] {
            assert_eq!(label, "circle");
        } else {
            panic!("Expected Jump command");
        }
    }
}
