use crate::lexeme::{Lexeme, MathOp};

pub struct Tokenizer<'a> {
    tokens: Vec<Lexeme>,
    // lifetime of the input &str. chars should never live longer than the input from which they were created
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    has_error: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            tokens: Vec::new(),
            chars: input.chars().peekable(),
            line: 1,
            has_error: false,
        }
    }

    pub fn tokenize(&mut self) -> Result<&[Lexeme], &[Lexeme]> {
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                }
                '\n' => {
                    self.chars.next();
                    self.line += 1;
                }
                '(' => self.add_token(Lexeme::LeftParen),
                ')' => self.add_token(Lexeme::RightParen),
                '{' => self.add_token(Lexeme::LeftBrace),
                '}' => self.add_token(Lexeme::RightBrace),
                ',' => self.add_token(Lexeme::Comma),
                '.' => self.add_token(Lexeme::Dot),
                ';' => self.add_token(Lexeme::Semicolon),
                '=' => self.handle_equal(),
                '!' => self.handle_bang(),
                '<' => self.handle_less(),
                '>' => self.handle_greater(),
                '0'..='9' => self.handle_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.handle_identifier(),
                '"' => self.handle_string(),
                '+' => self.add_token(Lexeme::Operator(MathOp::Plus)),
                '-' => self.add_token(Lexeme::Operator(MathOp::Minus)),
                '*' => self.add_token(Lexeme::Operator(MathOp::Star)),
                '/' => self.handle_slash(),
                _ => {
                    self.tokens.push(Lexeme::UnexpectedCharError(self.line, c));
                    self.has_error = true;
                    self.chars.next();
                }
            }
        }

        self.tokens.push(Lexeme::Eof);

        if self.has_error {
            Err(&self.tokens)
        } else {
            Ok(&self.tokens)
        }
    }

    fn add_token(&mut self, lexeme: Lexeme) {
        self.tokens.push(lexeme);
        self.chars.next();
    }

    fn handle_equal(&mut self) {
        self.chars.next();
        if let Some(&'=') = self.chars.peek() {
            self.add_token(Lexeme::EqualEqual);
        } else {
            self.tokens.push(Lexeme::Equal);
        }
    }

    fn handle_bang(&mut self) {
        self.chars.next();
        if let Some(&'=') = self.chars.peek() {
            self.add_token(Lexeme::BangEqual);
        } else {
            self.tokens.push(Lexeme::Bang);
        }
    }

    fn handle_less(&mut self) {
        self.chars.next();
        if let Some(&'=') = self.chars.peek() {
            self.add_token(Lexeme::LessEqual);
        } else {
            self.tokens.push(Lexeme::Less);
        }
    }

    fn handle_greater(&mut self) {
        self.chars.next();
        if let Some(&'=') = self.chars.peek() {
            self.add_token(Lexeme::GreaterEqual);
        } else {
            self.tokens.push(Lexeme::Greater);
        }
    }

    fn handle_number(&mut self) {
        let mut number = String::new();
        let mut has_decimal = false;
        while let Some(&d) = self.chars.peek() {
            match d {
                '0'..='9' => {
                    number.push(d);
                    self.chars.next();
                }
                '.' if !has_decimal => {
                    if self
                        .chars
                        .clone()
                        .nth(1)
                        .map_or(false, |next| next.is_ascii_digit())
                    {
                        number.push(d);
                        has_decimal = true;
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        let n = number.parse().unwrap();
        self.tokens.push(Lexeme::Number(number, n));

        if let Some(&'.') = self.chars.peek() {
            self.tokens.push(Lexeme::Dot);
            self.chars.next();
        }
    }

    fn handle_identifier(&mut self) {
        let mut identifier = String::new();
        while let Some(&d) = self.chars.peek() {
            if d.is_alphanumeric() || d == '_' {
                identifier.push(d);
                self.chars.next();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "and" | "class" | "else" | "false" | "for" | "fun" | "if" | "let" | "nil" | "or"
            | "return" | "super" | "this" | "true" | "var" | "while" | "print" => {
                self.tokens.push(Lexeme::Keyword(identifier));
            }
            _ => {
                self.tokens.push(Lexeme::Identifier(identifier));
            }
        }
    }

    fn handle_string(&mut self) {
        self.chars.next();
        let mut string = String::new();
        let start_line = self.line;
        let mut terminated = false;

        while let Some(&d) = self.chars.peek() {
            if d == '"' {
                self.chars.next();
                terminated = true;
                break;
            } else if d == '\n' {
                self.line += 1;
            }
            string.push(d);
            self.chars.next();
        }

        if terminated {
            self.tokens.push(Lexeme::String(string));
        } else {
            self.tokens
                .push(Lexeme::UnterminatedStringError(start_line));
            self.has_error = true;
        }
    }

    fn handle_slash(&mut self) {
        if let Some('/') = self.chars.clone().nth(1) {
            self.chars.next();
            self.chars.next();
            while let Some(&d) = self.chars.peek() {
                if d == '\n' {
                    break;
                }
                self.chars.next();
            }
        } else {
            self.add_token(Lexeme::Operator(MathOp::Slash));
        }
    }
}
