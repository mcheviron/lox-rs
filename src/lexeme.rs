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
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    UnexpectedCharError(usize, char),
    UnterminatedStringError(usize),
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
            Lexeme::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Lexeme::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Lexeme::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Lexeme::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Lexeme::Comma => write!(f, "COMMA , null"),
            Lexeme::Dot => write!(f, "DOT . null"),
            Lexeme::Semicolon => write!(f, "SEMICOLON ; null"),
            Lexeme::Equal => write!(f, "EQUAL = null"),
            Lexeme::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Lexeme::Bang => write!(f, "BANG ! null"),
            Lexeme::BangEqual => write!(f, "BANG_EQUAL != null"),
            Lexeme::Less => write!(f, "LESS < null"),
            Lexeme::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Lexeme::Greater => write!(f, "GREATER > null"),
            Lexeme::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            // errors
            Lexeme::UnexpectedCharError(line, ch) => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, ch)
            }
            Lexeme::UnterminatedStringError(line) => {
                write!(f, "[line {}] Error: Unterminated string.", line)
            }
        }
    }
}
