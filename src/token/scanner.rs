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

        tokens.push(Token::EOF);

        Ok(tokens)
    }
}

impl Scanner {
    fn consume_string_literal(chars: &mut Peekable<Chars>) -> Result<Token, ScannerError> {
        // count opening quotes (1 to 3)
        let mut quotes = 1;
        while quotes < 3 && matches!(chars.peek(), Some('"')) {
            chars.next();
            quotes += 1;
        }

        let mut value = String::new();
        let mut closing = 0;

        while let Some(ch) = chars.next() {
            if ch == '"' {
                closing += 1;
                if closing == quotes {
                    return Ok(Token::StringLiteral(value));
                }
            } else {
                if closing > 0 {
                    value.extend(std::iter::repeat('"').take(closing));
                    closing = 0;
                }
                value.push(ch);
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
