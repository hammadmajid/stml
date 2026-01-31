mod types;
mod parser;

use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken(Token),
}

