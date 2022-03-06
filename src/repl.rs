use std::io;

use crate::token::Token;
use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn start() -> Result<(), io::Error> {
    eprint!("{}", PROMPT);
    let input = {
        // ref: https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let mut lexer = Lexer::new(&input);
    
    let mut remaind_tokens = true;
    while remaind_tokens {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            remaind_tokens = false;
        }
    }
    Ok(())
}