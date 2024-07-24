use crate::lexeme::{Lexeme, MathOp};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Lexeme),
    #[error("Unmatched parentheses")]
    UnmatchedParentheses,
    #[error("Expected token: {0:?}")]
    ExpectedToken(Lexeme),
    #[error("Empty grouping")]
    EmptyGrouping,
    #[error("Invalid unary operator: {0:?}")]
    InvalidUnaryOperator(Lexeme),
}

pub type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser<'a> {
    tokens: &'a [Lexeme],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Lexeme]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<String> {
        let mut output = String::new();
        while !self.is_at_end() {
            output.push_str(&self.parse_expression()?);
            output.push('\n');
        }
        Ok(output)
    }

    fn parse_expression(&mut self) -> Result<String> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<String> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.peek(), Lexeme::EqualEqual | Lexeme::BangEqual) {
            let operator = self.advance().clone();
            let right = self.parse_comparison()?;
            expr = match operator {
                Lexeme::EqualEqual => format!("(== {} {})", expr, right),
                Lexeme::BangEqual => format!("(!= {} {})", expr, right),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<String> {
        let mut expr = self.parse_term()?;

        while matches!(
            self.peek(),
            Lexeme::Greater | Lexeme::Less | Lexeme::GreaterEqual | Lexeme::LessEqual
        ) {
            let operator = self.advance().clone();
            let right = self.parse_term()?;
            expr = match operator {
                Lexeme::Greater => format!("(> {} {})", expr, right),
                Lexeme::Less => format!("(< {} {})", expr, right),
                Lexeme::GreaterEqual => format!("(>= {} {})", expr, right),
                Lexeme::LessEqual => format!("(<= {} {})", expr, right),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<String> {
        let mut expr = self.parse_factor()?;

        while matches!(
            self.peek(),
            Lexeme::Operator(MathOp::Plus) | Lexeme::Operator(MathOp::Minus)
        ) {
            let operator = self.advance().clone();
            let right = self.parse_factor()?;
            expr = match operator {
                Lexeme::Operator(MathOp::Plus) => format!("(+ {} {})", expr, right),
                Lexeme::Operator(MathOp::Minus) => format!("(- {} {})", expr, right),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<String> {
        let mut expr = self.parse_unary()?;

        while matches!(
            self.peek(),
            Lexeme::Operator(MathOp::Star) | Lexeme::Operator(MathOp::Slash)
        ) {
            let operator = self.advance().clone();
            let right = self.parse_unary()?;
            expr = match operator {
                Lexeme::Operator(MathOp::Star) => format!("(* {} {})", expr, right),
                Lexeme::Operator(MathOp::Slash) => format!("(/ {} {})", expr, right),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<String> {
        match self.peek() {
            Lexeme::Bang | Lexeme::Operator(MathOp::Minus) => {
                let operator = self.advance().clone();
                let right = self.parse_unary()?;
                match operator {
                    Lexeme::Bang => Ok(format!("(! {})", right)),
                    Lexeme::Operator(MathOp::Minus) => Ok(format!("(- {})", right)),
                    _ => Err(ParserError::InvalidUnaryOperator(operator)),
                }
            }
            Lexeme::LeftParen => self.parse_grouping(),
            _ => self.parse_literal(),
        }
    }

    fn parse_grouping(&mut self) -> Result<String> {
        self.advance();
        let expressions = self.parse_grouped_expressions()?;
        if expressions.is_empty() {
            return Err(ParserError::EmptyGrouping);
        }
        self.consume(Lexeme::RightParen)?;
        Ok(format!("(group {})", expressions.join(", ")))
    }

    fn parse_grouped_expressions(&mut self) -> Result<Vec<String>> {
        let mut expressions = Vec::new();

        loop {
            match self.peek() {
                Lexeme::RightParen => break,
                Lexeme::Eof => return Err(ParserError::UnmatchedParentheses),
                _ => {
                    expressions.push(self.parse_expression()?);
                    if self.peek() != &Lexeme::Comma {
                        break;
                    }
                    self.advance(); // consume the comma to avoid trailing comma
                }
            }
        }

        Ok(expressions)
    }

    fn parse_literal(&mut self) -> Result<String> {
        match self.advance() {
            Lexeme::Keyword(s) => Ok(match s.as_str() {
                "true" | "false" | "nil" => s.to_string(),
                _ => "unknown".to_string(),
            }),
            Lexeme::Number(_, n) => Ok(if n.fract() == 0.0 {
                format!("{:.1}", n)
            } else {
                n.to_string()
            }),
            Lexeme::String(s) => Ok(s.to_string()),
            unexpected => Err(ParserError::UnexpectedToken(unexpected.clone())),
        }
    }

    fn consume(&mut self, expected: Lexeme) -> Result<&Lexeme> {
        match self.peek() {
            lexeme if lexeme == &expected => Ok(self.advance()),
            Lexeme::Eof => Err(ParserError::UnmatchedParentheses),
            _ => Err(ParserError::ExpectedToken(expected)),
        }
    }

    fn advance(&mut self) -> &Lexeme {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Lexeme::Eof)
    }

    fn peek(&self) -> &Lexeme {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Lexeme {
        &self.tokens[self.current - 1]
    }
}
