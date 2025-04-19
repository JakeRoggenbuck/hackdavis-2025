#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Colon,
    Identifier(String),
    Number(i32),
    Comma,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    chars: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            chars: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if self.position >= self.chars.len() {
            return Token::Eof;
        }

        let current_char = self.chars[self.position];
        
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
                while self.position < self.chars.len() && self.chars[self.position].is_digit(10) {
                    self.position += 1;
                }
                let num_str: String = self.chars[start..self.position].iter().collect();
                Token::Number(num_str.parse().unwrap())
            }
            _ if !current_char.is_whitespace() => {
                let start = self.position;
                while self.position < self.chars.len() {
                    let c = self.chars[self.position];
                    if c.is_whitespace() || c == ':' || c == ',' {
                        break;
                    }
                    self.position += 1;
                }
                let ident: String = self.chars[start..self.position].iter().collect();
                Token::Identifier(ident)
            }
            _ => panic!("Unexpected character: {}", current_char),
        }
    }

    pub fn peek_next_token(&self) -> Token {
        let mut clone = self.clone();
        clone.next_token()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.chars.len() {
            let c = self.chars[self.position];
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
        let input = "circle: mov direction, 1".to_string();
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Identifier("circle".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("direction".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_lexer_multiple_sections() {
        let input = r#"
        circle:
            mov direction, 1
        main:
            jal circle
        "#.to_string();
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Identifier("circle".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("direction".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::Identifier("main".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("jal".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("circle".to_string()));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_lexer_whitespace_handling() {
        let input = "  circle  :  mov  direction  ,  1  ".to_string();
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Identifier("circle".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Identifier("mov".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("direction".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::Eof);
    }
} 