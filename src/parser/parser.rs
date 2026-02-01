use crate::parser::types::{Document, Section};
use crate::parser::{ParseError, Parser};
use crate::token::Token;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Document, ParseError> {
        self.parse_document()
    }
}

impl Parser {
    fn parse_document(&self) -> Result<Document, ParseError> {
        match self.tokens.as_slice() {
            [
                Token::Doc,
                Token::StringLiteral(title),
                Token::LeftBrace,
                ..,
                Token::RightBrace,
                Token::EOF,
            ] => Ok(Document {
                title: title.clone(),
                sections: self.parse_section()?,
            }),

            [t, ..] => Err(ParseError::UnexpectedToken(t.clone())),
            [] => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_section(&self) -> Result<Vec<Section>, ParseError> {
        Ok(vec![])
    }
}
