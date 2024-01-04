use crate::token;

struct Lexer {
    input: &'static str,
    position: usize,
    next: usize,
    ch: Option<char>,
}

impl Lexer {
    fn new(input: &'static str) -> Self {
        let mut l = Lexer {
            input,
            ch: None,
            position: 0,
            next: 1,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        self.ch = match self.next {
            x if x <= self.input.len() => self.input.chars().nth(self.position),
            _ => None,
        };
        self.position = self.next;
        self.next += 1;
    }

    fn next_token(&mut self) -> token::Token {
        let token = match self.ch {
            Some('=') => token::Token::new(token::TokenKind::Assign, self.ch.unwrap().to_string()),
            Some(';') => token::Token::new(token::TokenKind::Semi, self.ch.unwrap().to_string()),
            Some('(') => token::Token::new(token::TokenKind::LParen, self.ch.unwrap().to_string()),
            Some(')') => token::Token::new(token::TokenKind::RParen, self.ch.unwrap().to_string()),
            Some(',') => token::Token::new(token::TokenKind::Comma, self.ch.unwrap().to_string()),
            Some('+') => token::Token::new(token::TokenKind::Plus, self.ch.unwrap().to_string()),
            Some('{') => token::Token::new(token::TokenKind::LBrace, self.ch.unwrap().to_string()),
            Some('}') => token::Token::new(token::TokenKind::RBrace, self.ch.unwrap().to_string()),
            None => token::Token::new(token::TokenKind::Eof, "".to_string()),
            _ => token::Token::new(token::TokenKind::Illegal, "".to_string()),
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected: Vec<token::Token> = vec![
            token::Token::new(token::TokenKind::Assign, "=".to_string()),
            token::Token::new(token::TokenKind::Plus, "+".to_string()),
            token::Token::new(token::TokenKind::LParen, "(".to_string()),
            token::Token::new(token::TokenKind::RParen, ")".to_string()),
            token::Token::new(token::TokenKind::LBrace, "{".to_string()),
            token::Token::new(token::TokenKind::RBrace, "}".to_string()),
            token::Token::new(token::TokenKind::Comma, ",".to_string()),
            token::Token::new(token::TokenKind::Semi, ";".to_string()),
            token::Token::new(token::TokenKind::Eof, "".to_string()),
        ];
        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<token::Token> = Vec::new();
        for _ in &expected {
            tokens.push(lexer.next_token());
        }
        assert_eq!(tokens, expected);
    }
}
