#[derive(Debug, PartialEq, Clone)]
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
            _ if current_char.is_alphabetic() || current_char == '_' => {
                let start = self.position;
                while self.position < self.input.len() {
                    let c = self.input.chars().nth(self.position).unwrap();
                    if !c.is_alphanumeric() && c != '_' {
                        break;
                    }
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
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if !c.is_whitespace() {
                break;
            }
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let input = "section start: mov forward, 10".to_string();
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Section);
        assert_eq!(lexer.next_token(), Token::Identifier("start".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("forward".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(10));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_lexer_multiple_sections() {
        let input = r#"
        section one:
            mov forward, 5
        section two:
            mov backward, 3
        "#.to_string();
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Section);
        assert_eq!(lexer.next_token(), Token::Identifier("one".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("forward".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(5));
        assert_eq!(lexer.next_token(), Token::Section);
        assert_eq!(lexer.next_token(), Token::Identifier("two".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("backward".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(3));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_lexer_whitespace_handling() {
        let input = "  section  start  :  mov  forward  ,  10  ".to_string();
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Section);
        assert_eq!(lexer.next_token(), Token::Identifier("start".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("forward".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(10));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_lexer_unicode_identifiers() {
        let input = "section 你好: mov 世界, 10".to_string();
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Section);
        assert_eq!(lexer.next_token(), Token::Identifier("你好".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("世界".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(10));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    #[should_panic(expected = "Unexpected character")]
    fn test_lexer_invalid_char() {
        let input = "section start: mov forward, @10".to_string();
        let mut lexer = Lexer::new(input);
        
        // Should panic on '@'
        while lexer.next_token() != Token::Eof {}
    }
} 