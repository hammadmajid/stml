use crate::parser::types::Document;
use crate::parser::{ParseError, Parser};
use crate::token::Token;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<Document, ParseError> {
        Ok(Document {
            title: "".to_string(),
            sections: vec![],
        })
    }
}
