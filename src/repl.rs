use crate::lexer::Lexer;
use crate::token::TokenKind;
use std::io::{Result, Write};

const PROMPT: &'static str = ">> ";

pub fn start() -> Result<()> {
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            if token.kind == TokenKind::Eof {
                println!("");
                break;
            }
            println!("{:?}", token);
        }
    }
}
