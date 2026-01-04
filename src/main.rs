mod token;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use crate::token::scanner::Scanner;

fn main() {
    let input_file = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage:\n\tstml in.stml out.[html|md]");
        exit(64);
    });
    let output_file = env::args().nth(2).unwrap_or_else(|| {
        eprintln!("Usage:\n\tstml in.stml out.[html|md]");
        exit(64);
    });

    let input_path = PathBuf::from(&input_file);

    let input_content = fs::read_to_string(&input_path).unwrap_or_else(|_| {
        eprintln!("Input file '{}' does not exist or cannot be read.", input_path.display());
        exit(64);
    });

    let mut scanner = Scanner::new(input_content);
    let tokens = scanner.tokenize().unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err);
        exit(64);
    });

    for token in tokens {
        println!("{:?}", token);
    }
}
