use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf, process};
use thiserror::Error;

mod lexeme;
use lexeme::{Lexeme, MathOp};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tokenize {
        #[arg(value_name = "FILE", help = "Path to the source file")]
        file: PathBuf,
    },
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Tokenize { file } => {
            let file_contents = fs::read_to_string(file).map_err(AppError::FileReadError)?;

            match tokenize(&file_contents) {
                Ok(tokens) => {
                    for token in tokens {
                        println!("{}", token);
                    }
                }
                Err(tokens) => {
                    for token in tokens {
                        match token {
                            Lexeme::UnexpectedCharError(..)
                            | Lexeme::UnterminatedStringError(..) => {
                                eprintln!("{token}")
                            }
                            _ => println!("{token}"),
                        }
                    }
                    process::exit(65);
                }
            }
        }
    }

    Ok(())
}
fn tokenize(input: &str) -> Result<Vec<Lexeme>, Vec<Lexeme>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut has_error = false;

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' => {
                chars.next();
            }
            '\n' => {
                chars.next();
                line += 1;
            }
            '(' => {
                tokens.push(Lexeme::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Lexeme::RightParen);
                chars.next();
            }
            '{' => {
                tokens.push(Lexeme::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Lexeme::RightBrace);
                chars.next();
            }
            ',' => {
                tokens.push(Lexeme::Comma);
                chars.next();
            }
            '.' => {
                tokens.push(Lexeme::Dot);
                chars.next();
            }
            ';' => {
                tokens.push(Lexeme::Semicolon);
                chars.next();
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Lexeme::EqualEqual);
                    chars.next();
                } else {
                    tokens.push(Lexeme::Equal);
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Lexeme::BangEqual);
                    chars.next();
                } else {
                    tokens.push(Lexeme::Bang);
                }
            }
            '<' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Lexeme::LessEqual);
                    chars.next();
                } else {
                    tokens.push(Lexeme::Less);
                }
            }
            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    tokens.push(Lexeme::GreaterEqual);
                    chars.next();
                } else {
                    tokens.push(Lexeme::Greater);
                }
            }
            '0'..='9' => {
                let mut number = String::new();
                let mut has_decimal = false;
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        number.push(d);
                        chars.next();
                    } else if d == '.' && !has_decimal {
                        if chars.peek().map_or(false, |&next| next.is_ascii_digit()) {
                            number.push(d);
                            has_decimal = true;
                            chars.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let n = number.parse().unwrap();
                tokens.push(Lexeme::Number(number, n));

                if let Some(&'.') = chars.peek() {
                    tokens.push(Lexeme::Dot);
                    chars.next();
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        identifier.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if [
                    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print",
                    "return", "super", "this", "true", "var", "while",
                ]
                .contains(&identifier.as_str())
                {
                    tokens.push(Lexeme::Keyword(identifier));
                } else {
                    tokens.push(Lexeme::Identifier(identifier));
                }
            }
            '"' => {
                // Consume the opening quote
                chars.next();
                let mut string = String::new();
                let start_line = line;
                let mut terminated = false;

                while let Some(&d) = chars.peek() {
                    if d == '"' {
                        // Consume the closing quote
                        chars.next();
                        terminated = true;
                        break;
                    } else if d == '\n' {
                        line += 1;
                    }
                    string.push(d);
                    chars.next();
                }

                if terminated {
                    tokens.push(Lexeme::String(string));
                } else {
                    tokens.push(Lexeme::UnterminatedStringError(start_line));
                    has_error = true;
                }
            }
            '+' => {
                tokens.push(Lexeme::Operator(MathOp::Plus));
                chars.next();
            }
            '-' => {
                tokens.push(Lexeme::Operator(MathOp::Minus));
                chars.next();
            }
            '*' => {
                tokens.push(Lexeme::Operator(MathOp::Star));
                chars.next();
            }
            '/' => {
                if let Some('/') = chars.clone().nth(1) {
                    // Skip two characters "//"
                    chars.next();
                    chars.next();
                    while let Some(&d) = chars.peek() {
                        if d == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    tokens.push(Lexeme::Operator(MathOp::Slash));
                    chars.next();
                }
            }
            _ => {
                tokens.push(Lexeme::UnexpectedCharError(line, c));
                has_error = true;
                chars.next();
            }
        }
    }

    tokens.push(Lexeme::Eof);

    if has_error {
        Err(tokens)
    } else {
        Ok(tokens)
    }
}
