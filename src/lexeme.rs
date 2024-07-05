use std::fmt;

pub enum Lexeme {
    Eof,
    Identifier(String),
    Number(f64),
    String(String),
    Operator(String),
    Keyword(String),
    Comment(String),
    LeftParen,
    RightParen,
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lexeme::Eof => write!(f, "EOF  null"),
            Lexeme::Identifier(s) => write!(f, "IDENTIFIER {} null", s),
            Lexeme::Number(n) => write!(f, "NUMBER {} {}", n, n),
            Lexeme::String(s) => write!(f, "STRING \"{}\" {}", s, s),
            Lexeme::Operator(op) => write!(f, "OPERATOR {} null", op),
            Lexeme::Keyword(kw) => write!(f, "KEYWORD {} null", kw),
            Lexeme::Comment(c) => write!(f, "COMMENT {} null", c),
            Lexeme::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Lexeme::RightParen => write!(f, "RIGHT_PAREN ) null"),
        }
    }
}
