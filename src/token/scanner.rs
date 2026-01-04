use crate::token::Token;

#[derive(Debug)]
pub enum ScannerError {
    InvalidToken(char),
    UnterminatedToken(char),
    UnexpectedEOF,
}

pub struct Scanner {
    string: String,
    pos: usize,
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        Scanner {
            string: input,
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut tokens: Vec<Token> = Vec::new();

        todo!();

        Ok(tokens)
    }
}
