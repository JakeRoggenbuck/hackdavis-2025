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
                if ident == "section" {
                    Token::Section
                } else {
                    Token::Identifier(ident)
                }
            }
            _ => panic!("Unexpected character: {}", current_char),
        }
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
} 