use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
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
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Tokenize { file } => {
            eprintln!("Tokenizing file: {:?}", file);

            let file_contents = fs::read_to_string(file).map_err(AppError::FileReadError)?;

            let tokens = tokenize(&file_contents)?;

            for token in tokens {
                println!("{}", token);
            }
        }
    }

    Ok(())
}
fn tokenize(input: &str) -> Result<Vec<Lexeme>, AppError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
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
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        number.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Lexeme::Number(number.parse().unwrap()));
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
                    "and", "class", "else", "false", "for", "fun", "if", "let", "nil", "or",
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
                chars.next();
                let mut string = String::new();
                while let Some(&d) = chars.peek() {
                    if d == '"' {
                        chars.next();
                        break;
                    } else {
                        string.push(d);
                        chars.next();
                    }
                }
                tokens.push(Lexeme::String(string));
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
                    // This is a comment
                    chars.next(); // Consume the first '/'
                    chars.next(); // Consume the second '/'
                    let mut comment = String::new();
                    while let Some(&d) = chars.peek() {
                        if d == '\n' {
                            break;
                        } else {
                            comment.push(d);
                            chars.next();
                        }
                    }
                    tokens.push(Lexeme::Comment(comment));
                } else {
                    tokens.push(Lexeme::Operator(MathOp::Slash));
                    chars.next();
                }
            }
            _ => return Err(AppError::UnexpectedCharacter(c)),
        }
    }

    tokens.push(Lexeme::Eof);
    Ok(tokens)
}
