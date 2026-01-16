mod token;
mod utils;

use crate::token::scanner::{Scanner, ScannerError};
use crate::utils::sysexits::{ExitCode, exit};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let input_file = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage:\n\tstml in.stml out.[html|md]");
        exit(ExitCode::Usage);
    });
    let output_file = env::args().nth(2).unwrap_or_else(|| {
        eprintln!("Usage:\n\tstml in.stml out.[html|md]");
        exit(ExitCode::Usage);
    });

    let input_path = PathBuf::from(&input_file);

    let input_content = fs::read_to_string(&input_path).unwrap_or_else(|_| {
        eprintln!(
            "Input file '{}' does not exist or cannot be read.",
            input_path.display()
        );
        exit(ExitCode::NoInput);
    });

    let scanner = Scanner::new();
    let tokens = scanner
        .tokenize(input_content.as_str())
        .unwrap_or_else(|err| {
            match err {
                ScannerError::InvalidToken(token) => {
                    eprintln!("Invalid token: {}", token);
                }
                ScannerError::UnterminatedToken(token) => {
                    eprintln!("Unterminated token: {}", token);
                }
            }
            exit(ExitCode::DataErr);
        });

    for token in tokens {
        match token {
            token::Token::Identifier(identifier) => println!("Identifier({identifier})"),
            token::Token::StringLiteral(string) => println!("String({string})"),
            _ => println!("{:?}", token),
        }
    }

    fs::write(&output_file, "").unwrap_or_else(|_| {
        eprintln!("Output file '{}' cannot be written.", output_file);
        exit(ExitCode::CantCreat);
    });
}
