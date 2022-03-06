#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    EOF,

    // identifier, literal
    Ident(String),
    Int(i32),
    Bool(bool),

    // operator
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Equal,
    NotEqual,
    LowerThan,
    GraterThan,

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
    Return,

    // Statements
    If,
    Else,
}
