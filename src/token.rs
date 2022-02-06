pub enum Token {
    Illegal,
    EOF,
    // identifier, literal
    Ident(String),
    Int(i32),
    // operator
    Assign,
    Plus,
    // delimiter
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // keyword
    Function,
    Let,
}
