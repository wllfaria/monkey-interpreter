use crate::token::*;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    next: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
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

    fn match_identifier(&self, identifier: String) -> Token {
        match identifier.as_str() {
            "let" => Token::new(TokenKind::Let, identifier),
            "fn" => Token::new(TokenKind::Function, identifier),
            "true" => Token::new(TokenKind::True, identifier),
            "false" => Token::new(TokenKind::False, identifier),
            "if" => Token::new(TokenKind::If, identifier),
            "else" => Token::new(TokenKind::Else, identifier),
            "return" => Token::new(TokenKind::Return, identifier),
            _ => Token::new(TokenKind::Ident, identifier),
        }
    }

    fn peek_char(&self) -> Option<char> {
        match self.next {
            x if x <= self.input.len() => self.input.chars().nth(self.next - 1),
            _ => None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            Some('=') => {
                let peek = self.peek_char();
                match peek {
                    Some('=') => {
                        self.read_char();
                        Token::new(TokenKind::Eq, "==".to_string())
                    }
                    _ => Token::new(TokenKind::Assign, self.ch.unwrap().to_string()),
                }
            }
            Some('+') => Token::new(TokenKind::Plus, self.ch.unwrap().to_string()),
            Some('-') => Token::new(TokenKind::Minus, self.ch.unwrap().to_string()),
            Some('!') => {
                let peek = self.peek_char();
                match peek {
                    Some('=') => {
                        self.read_char();
                        Token::new(TokenKind::NotEq, "!=".to_string())
                    }
                    _ => Token::new(TokenKind::Bang, self.ch.unwrap().to_string()),
                }
            }
            Some('/') => Token::new(TokenKind::Slash, self.ch.unwrap().to_string()),
            Some('*') => Token::new(TokenKind::Asterisk, self.ch.unwrap().to_string()),
            Some('<') => Token::new(TokenKind::Lt, self.ch.unwrap().to_string()),
            Some('>') => Token::new(TokenKind::Gt, self.ch.unwrap().to_string()),
            Some(';') => Token::new(TokenKind::Semi, self.ch.unwrap().to_string()),
            Some('(') => Token::new(TokenKind::LParen, self.ch.unwrap().to_string()),
            Some(')') => Token::new(TokenKind::RParen, self.ch.unwrap().to_string()),
            Some(',') => Token::new(TokenKind::Comma, self.ch.unwrap().to_string()),
            Some('{') => Token::new(TokenKind::LBrace, self.ch.unwrap().to_string()),
            Some('}') => Token::new(TokenKind::RBrace, self.ch.unwrap().to_string()),
            Some(ch) if ch.is_alphabetic() => {
                let identifier = self.read_identifier();
                self.match_identifier(identifier)
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
        while self.ch.is_some() {
            if self.ch.unwrap().is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            ! =/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10;
            10 != 9;
            ";

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
            Token::new(TokenKind::Bang, "!".to_string()),
            Token::new(TokenKind::Assign, "=".to_string()),
            Token::new(TokenKind::Slash, "/".to_string()),
            Token::new(TokenKind::Asterisk, "*".to_string()),
            Token::new(TokenKind::Int, "5".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::Int, "5".to_string()),
            Token::new(TokenKind::Lt, "<".to_string()),
            Token::new(TokenKind::Int, "10".to_string()),
            Token::new(TokenKind::Gt, ">".to_string()),
            Token::new(TokenKind::Int, "5".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::If, "if".to_string()),
            Token::new(TokenKind::LParen, "(".to_string()),
            Token::new(TokenKind::Int, "5".to_string()),
            Token::new(TokenKind::Lt, "<".to_string()),
            Token::new(TokenKind::Int, "10".to_string()),
            Token::new(TokenKind::RParen, ")".to_string()),
            Token::new(TokenKind::LBrace, "{".to_string()),
            Token::new(TokenKind::Return, "return".to_string()),
            Token::new(TokenKind::True, "true".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::RBrace, "}".to_string()),
            Token::new(TokenKind::Else, "else".to_string()),
            Token::new(TokenKind::LBrace, "{".to_string()),
            Token::new(TokenKind::Return, "return".to_string()),
            Token::new(TokenKind::False, "false".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::RBrace, "}".to_string()),
            Token::new(TokenKind::Int, "10".to_string()),
            Token::new(TokenKind::Eq, "==".to_string()),
            Token::new(TokenKind::Int, "10".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::Int, "10".to_string()),
            Token::new(TokenKind::NotEq, "!=".to_string()),
            Token::new(TokenKind::Int, "9".to_string()),
            Token::new(TokenKind::Semi, ";".to_string()),
            Token::new(TokenKind::Eof, "".to_string()),
        ];

        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while lexer.ch.is_some() {
            tokens.push(lexer.next_token());
        }
        assert_eq!(tokens, expected);
    }
}
