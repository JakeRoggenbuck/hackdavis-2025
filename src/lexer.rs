#[derive(Debug, PartialEq)]
pub enum Token {
    Section,
    Colon,
    Identifier(String),
    Number(i32),
    Comma,
    Eof,
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Token::Eof;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();
        
        match current_char {
            ':' => {
                self.position += 1;
                Token::Colon
            }
            ',' => {
                self.position += 1;
                Token::Comma
            }
            '0'..='9' => {
                let start = self.position;
                while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_digit(10) {
                    self.position += 1;
                }
                let num_str = &self.input[start..self.position];
                Token::Number(num_str.parse().unwrap())
            }
            _ if current_char.is_alphabetic() => {
                let start = self.position;
                while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_alphanumeric() {
                    self.position += 1;
                }
                let ident = &self.input[start..self.position];
                if ident == "section" {
                    Token::Section
                } else {
                    Token::Identifier(ident.to_string())
                }
            }
            _ => panic!("Unexpected character: {}", current_char),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_whitespace() {
            self.position += 1;
        }
    }
} 