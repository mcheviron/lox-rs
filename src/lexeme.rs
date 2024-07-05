use std::fmt;

pub enum MathOp {
    Plus,
    Minus,
    Star,
    Slash,
}

impl fmt::Display for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathOp::Plus => write!(f, "PLUS +"),
            MathOp::Minus => write!(f, "MINUS -"),
            MathOp::Star => write!(f, "STAR *"),
            MathOp::Slash => write!(f, "SLASH /"),
        }
    }
}

pub enum Lexeme {
    Eof,
    Identifier(String),
    Number(f64),
    String(String),
    Operator(MathOp),
    Keyword(String),
    Comment(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lexeme::Eof => write!(f, "EOF  null"),
            Lexeme::Identifier(s) => write!(f, "IDENTIFIER {} null", s),
            Lexeme::Number(n) => write!(f, "NUMBER {} {}", n, n),
            Lexeme::String(s) => write!(f, "STRING \"{}\" {}", s, s),
            Lexeme::Operator(op) => write!(f, "{} null", op),
            Lexeme::Keyword(kw) => write!(f, "KEYWORD {} null", kw),
            Lexeme::Comment(c) => write!(f, "COMMENT {} null", c),
            Lexeme::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Lexeme::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Lexeme::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Lexeme::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Lexeme::Comma => write!(f, "COMMA , null"),
            Lexeme::Dot => write!(f, "DOT . null"),
            Lexeme::Semicolon => write!(f, "SEMICOLON ; null"),
        }
    }
}
