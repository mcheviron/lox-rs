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
    #[error("")]
    UnexpectedCharacter,
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
                Err((error, tokens)) => {
                    for token in tokens {
                        match token {
                            Lexeme::Error(..) => {
                                eprintln!("{token}")
                            }
                            _ => println!("{token}"),
                        }
                    }
                    if let AppError::UnexpectedCharacter = error {
                        process::exit(65);
                    } else {
                        return Err(error);
                    }
                }
            }
        }
    }

    Ok(())
}
fn tokenize(input: &str) -> Result<Vec<Lexeme>, (AppError, Vec<Lexeme>)> {
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
                    chars.next();
                    chars.next();
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
            _ => {
                tokens.push(Lexeme::Error(line, c));
                has_error = true;
                chars.next();
            }
        }
    }

    tokens.push(Lexeme::Eof);

    if has_error {
        Err((AppError::UnexpectedCharacter, tokens))
    } else {
        Ok(tokens)
    }
}
