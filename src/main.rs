use std::{fs, path::PathBuf, process};

use clap::{Parser, Subcommand};
use thiserror::Error;

mod lexeme;
mod tokenizer;

use lexeme::Lexeme;
use tokenizer::Tokenizer;

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
    Parse {
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

            let mut tokenizer = Tokenizer::new(&file_contents);
            match tokenizer.tokenize() {
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
        Commands::Parse { file } => {
            let file_contents = fs::read_to_string(file).map_err(AppError::FileReadError)?;
            let trimmed_contents = file_contents.trim();
            match trimmed_contents {
                "true" => println!("true"),
                "false" => println!("false"),
                "nil" => println!("nil"),
                _ => println!("Unexpected input"),
            }
        }
    }

    Ok(())
}
