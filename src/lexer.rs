use crate::token::*;

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

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            Some('=') => Token::new(TokenKind::Assign, self.ch.unwrap().to_string()),
            Some(';') => Token::new(TokenKind::Semi, self.ch.unwrap().to_string()),
            Some('(') => Token::new(TokenKind::LParen, self.ch.unwrap().to_string()),
            Some(')') => Token::new(TokenKind::RParen, self.ch.unwrap().to_string()),
            Some(',') => Token::new(TokenKind::Comma, self.ch.unwrap().to_string()),
            Some('+') => Token::new(TokenKind::Plus, self.ch.unwrap().to_string()),
            Some('{') => Token::new(TokenKind::LBrace, self.ch.unwrap().to_string()),
            Some('}') => Token::new(TokenKind::RBrace, self.ch.unwrap().to_string()),
            Some(ch) if ch.is_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "let" => Token::new(TokenKind::Let, identifier),
                    "fn" => Token::new(TokenKind::Function, identifier),
                    _ => Token::new(TokenKind::Ident, identifier),
                }
            }
            Some(c) if c.is_numeric() => Token::new(TokenKind::Int, self.read_integer()),
            None => Token::new(TokenKind::Eof, "".to_string()),
            _ => Token::new(TokenKind::Illegal, "".to_string()),
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position - 1;
        while self
            .input
            .chars()
            .nth(self.next - 1)
            .unwrap()
            .is_alphabetic()
        {
            self.read_char();
        }
        self.input[start..self.position].to_string()
    }

    fn read_integer(&mut self) -> String {
        let start = self.position - 1;
        while self.input.chars().nth(self.next - 1).unwrap().is_numeric() {
            self.read_char();
        }
        self.input[start..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        match self.ch {
            None => return,
            _ => {}
        };
        while self.ch.unwrap().is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);";
        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Let, "let".to_string()),
            Token::new(TokenKind::Ident, "five".to_string()),
            Token::new(TokenKind::Assign, "=".to_string()),
            Token::new(TokenKind::Int, "5".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::Let, "let".to_string()),
            Token::new(TokenKind::Ident, "ten".to_string()),
            Token::new(TokenKind::Assign, "=".to_string()),
            Token::new(TokenKind::Int, "10".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::Let, "let".to_string()),
            Token::new(TokenKind::Ident, "add".to_string()),
            Token::new(TokenKind::Assign, "=".to_string()),
            Token::new(TokenKind::Function, "fn".to_string()),
            Token::new(TokenKind::LParen, "(".to_string()),
            Token::new(TokenKind::Ident, "x".to_string()),
            Token::new(TokenKind::Comma, ",".to_string()),
            Token::new(TokenKind::Ident, "y".to_string()),
            Token::new(TokenKind::RParen, ")".to_string()),
            Token::new(TokenKind::LBrace, "{".to_string()),
            Token::new(TokenKind::Ident, "x".to_string()),
            Token::new(TokenKind::Plus, "+".to_string()),
            Token::new(TokenKind::Ident, "y".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::RBrace, "}".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::Let, "let".to_string()),
            Token::new(TokenKind::Ident, "result".to_string()),
            Token::new(TokenKind::Assign, "=".to_string()),
            Token::new(TokenKind::Ident, "add".to_string()),
            Token::new(TokenKind::LParen, "(".to_string()),
            Token::new(TokenKind::Ident, "five".to_string()),
            Token::new(TokenKind::Comma, ",".to_string()),
            Token::new(TokenKind::Ident, "ten".to_string()),
            Token::new(TokenKind::RParen, ")".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
        ];
        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        for _ in &expected {
            tokens.push(lexer.next_token());
        }
        assert_eq!(tokens, expected);
    }
}
