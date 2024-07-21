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
        Commands::Tokenize { file } => tokenize_file(file)?,
        Commands::Parse { file } => parse_file(file)?,
    }

    Ok(())
}

fn tokenize_file(file: &PathBuf) -> Result<(), AppError> {
    let file_contents = fs::read_to_string(file).map_err(AppError::FileReadError)?;
    let mut tokenizer = Tokenizer::new(&file_contents);

    match tokenizer.tokenize() {
        Ok(tokens) => print_tokens(tokens),
        Err(tokens) => {
            print_tokens_with_errors(tokens);
            process::exit(65);
        }
    }

    Ok(())
}

fn print_tokens(tokens: &[Lexeme]) {
    for token in tokens {
        println!("{}", token);
    }
}

fn print_tokens_with_errors(tokens: &[Lexeme]) {
    for token in tokens {
        match token {
            Lexeme::UnexpectedCharError(..) | Lexeme::UnterminatedStringError(..) => {
                eprintln!("{}", token)
            }
            _ => println!("{}", token),
        }
    }
}

fn parse_file(file: &PathBuf) -> Result<(), AppError> {
    let file_contents = fs::read_to_string(file).map_err(AppError::FileReadError)?;
    let trimmed_contents = file_contents.trim();

    let result = match trimmed_contents {
        "true" => "true",
        "false" => "false",
        "nil" => "nil",
        _ => "Unexpected input",
    };

    println!("{}", result);
    Ok(())
}
