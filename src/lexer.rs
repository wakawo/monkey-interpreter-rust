use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input,
            pos: 0,
            read_pos: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    pub fn peak_char(&self) -> u8{
        if self.read_pos >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_pos];
        }
    }

    pub fn next_char_is(&self, char: u8) -> bool {
        // ref: https://github.com/wadackel/rs-monkey-lang/blob/master/src/lexer/mod.rs#L42
        // かっこよかったので採用
        self.peak_char() == char
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b'=' => {
                if self.next_char_is( b'=') {
                    self.read_char();
                    Token::Equal
                }
                else {
                    Token::Assign
                }
            }
            b'!' => {
                if self.next_char_is( b'=') {
                    self.read_char();
                    Token::NotEqual
                }
                else {
                    Token::Bang
                }
            }
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LowerThan,
            b'>' => Token::GraterThan,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            0 => Token::EOF,
            _ => {
                // ちょっとカッコ悪い
                if self.is_letter(self.ch) {
                    return self.read_identifier()
                } else if self.is_digit(self.ch) {
                    return self.read_number()
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        token
    }

    fn is_letter(&self, ch: u8) -> bool {
        return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
    }

    fn read_identifier(&mut self) -> Token {
        let start_pos = self.pos;
        while self.is_letter(self.ch) {
            self.read_char();
        }

        let literal = &self.input[start_pos..self.pos];

        match literal {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(String::from(literal)),
        }
    }

    fn is_digit(&self, ch: u8) -> bool {
        return b'0' <= ch && ch <= b'9';
    }

    fn read_number(&mut self) -> Token {
        let start_pos = self.pos;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        let literal = &self.input[start_pos..self.pos];
        Token::Int(literal.parse::<i32>().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    pub fn get_input(&self) {
        println!("{}", self.input);
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_next_token() {
        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let input: &str = "=+(){},;";
        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens.iter() {
            assert_eq!(&lexer.next_token(), expected_token);
        }
    }

    #[test]
    fn test_next_token_2() {
        let expected_tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,  // !
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LowerThan,
            Token::Int(10),
            Token::GraterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::If,  // if section
            Token::Lparen,
            Token::Int(5),
            Token::LowerThan,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::Bool(true),
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::Bool(false),
            Token::Semicolon,
            Token::Rbrace,
            Token::Int(10),  // == section
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
            Token::EOF,
        ];

        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ";
        let mut lexer = Lexer::new(input);

        for expected_token in expected_tokens.iter() {
            assert_eq!(&lexer.next_token(), expected_token);
        }
    }
}
