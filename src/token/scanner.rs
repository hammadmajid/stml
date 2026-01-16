use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum ScannerError {
    InvalidToken(char),
    UnterminatedToken(char),
}

pub struct Scanner;

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn tokenize(self, input: &str) -> Result<Vec<Token>, ScannerError> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                ' ' | '\t' | '\n' => {}

                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                '#' => tokens.push(Token::Hash),
                '.' => tokens.push(Token::Dot),
                ':' => tokens.push(Token::Colon),

                '"' => {
                    let token = Self::consume_string_literal(&mut chars)?;
                    tokens.push(token);
                }

                'a'..='z' => {
                    let token = Self::consume_keyword_or_literal(ch, &mut chars)?;
                    tokens.push(token);
                }

                _ => return Err(ScannerError::InvalidToken(ch)),
            }
        }

        Ok(tokens)
    }
}

impl Scanner {
    fn consume_string_literal(chars: &mut Peekable<Chars>) -> Result<Token, ScannerError> {
        let mut quote_len = 1;

        while matches!(chars.peek(), Some('"')) && quote_len < 3 {
            chars.next();
            quote_len += 1;
        }

        let terminator = quote_len;
        let mut value = String::new();
        let mut run = 0;

        while let Some(ch) = chars.peek() {
            if *ch == '"' {
                chars.next();
                run += 1;
                if run == terminator {
                    return Ok(Token::StringLiteral(value));
                }
            } else {
                if run > 0 {
                    for _ in 0..run {
                        value.push('"');
                    }
                    run = 0;
                }
                value.push(*ch);
                chars.next();
            }
        }

        Err(ScannerError::UnterminatedToken('"'))
    }

    fn consume_keyword_or_literal(
        first: char,
        chars: &mut Peekable<Chars>,
    ) -> Result<Token, ScannerError> {
        let mut value = String::new();
        value.push(first);

        while let Some(ch) = chars.peek() {
            if ch.is_ascii_lowercase() {
                value.push(*ch);
                chars.next();
            } else {
                break;
            }
        }

        match value.as_str() {
            "doc" => Ok(Token::Doc),
            "p" => Ok(Token::P),
            "image" => Ok(Token::Image),
            "section" => Ok(Token::Section),
            "fallback" => Ok(Token::Fallback),
            "maybe" => Ok(Token::Maybe),
            "src" => Ok(Token::Src),
            "alt" => Ok(Token::Alt),
            "link" => Ok(Token::Link),
            "to" => Ok(Token::To),
            "text" => Ok(Token::Text),
            "ul" => Ok(Token::Ul),
            "li" => Ok(Token::Li),
            "ol" => Ok(Token::Ol),
            "ref" => Ok(Token::Ref),
            "meta" => Ok(Token::Meta),
            _ => Ok(Token::Identifier(value)),
        }
    }
}
