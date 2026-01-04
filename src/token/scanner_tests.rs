#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::token::scanner::{Scanner, ScannerError};

    #[test]
    fn empty_input_produces_no_tokens() {
        let mut scanner = Scanner::new(String::new());
        let tokens = scanner.tokenize().unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn scans_single_keyword() {
        let mut scanner = Scanner::new("doc".to_string());
        let tokens = scanner.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        matches!(tokens[0], Token::Doc);
    }

    #[test]
    fn scans_braces() {
        let mut scanner = Scanner::new("{ }".to_string());
        let tokens = scanner.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        matches!(tokens[0], Token::LeftBrace);
        matches!(tokens[1], Token::RightBrace);
    }

    #[test]
    fn scans_string_literal() {
        let mut scanner = Scanner::new("\"hello\"".to_string());
        let tokens = scanner.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        matches!(tokens[0], Token::StringLiteral);
    }

    #[test]
    fn invalid_character_errors() {
        let mut scanner = Scanner::new("@".to_string());
        let result = scanner.tokenize();

        assert!(matches!(result, Err(ScannerError::InvalidToken('@'))));
    }

    #[test]
    fn unterminated_string_errors() {
        let mut scanner = Scanner::new("\"oops".to_string());
        let result = scanner.tokenize();

        assert!(matches!(result, Err(ScannerError::UnterminatedToken('"'))));
    }
}
