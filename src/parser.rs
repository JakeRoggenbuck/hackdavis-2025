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
        self.expect_token(Token::Section);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.advance();
            name
        } else {
            panic!("Expected section name");
        };
        self.expect_token(Token::Colon);
        self.advance();

        let mut commands = Vec::new();
        while self.current_token != Token::Eof && self.current_token != Token::Section {
            commands.push(self.parse_command());
        }

        Section { name, commands }
    }

    fn parse_command(&mut self) -> Command {
        self.expect_token(Token::Identifier("mov".to_string()));
        self.advance();

        let direction = if let Token::Identifier(dir) = self.current_token.clone() {
            self.advance();
            dir
        } else {
            panic!("Expected direction");
        };

        self.expect_token(Token::Comma);
        self.advance();

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

    fn expect_token(&self, expected: Token) {
        if self.current_token != expected {
            panic!("Expected {:?}, got {:?}", expected, self.current_token);
        }
    }
} 