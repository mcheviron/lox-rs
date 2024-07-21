use thiserror::Error;

use crate::lexeme::Lexeme;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Lexeme),
}

pub struct Parser<'a> {
    tokens: &'a [Lexeme],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Lexeme]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<String, ParserError> {
        let mut output = String::new();
        while !self.is_at_end() {
            output.push_str(&self.parse_token()?);
            output.push('\n');
        }
        Ok(output)
    }

    fn parse_token(&mut self) -> Result<String, ParserError> {
        match self.advance() {
            Lexeme::Keyword(s) if s == "true" => Ok("true".to_string()),
            Lexeme::Keyword(s) if s == "false" => Ok("false".to_string()),
            Lexeme::Keyword(s) if s == "nil" => Ok("nil".to_string()),
            Lexeme::Number(_, n) => {
                if n.fract() == 0.0 {
                    Ok(format!("{:.1}", n))
                } else {
                    Ok(n.to_string())
                }
            }
            _ => Ok("unknown".to_string()),
        }
    }

    fn advance(&mut self) -> &Lexeme {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == &Lexeme::Eof
    }

    fn peek(&self) -> &Lexeme {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Lexeme {
        &self.tokens[self.current - 1]
    }
}
