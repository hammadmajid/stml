#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::token::scanner::{Scanner, ScannerError};

    #[test]
    fn empty_input_produces_no_tokens() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn scans_single_keyword() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("doc").unwrap();

        assert_eq!(tokens.len(), 1);
        matches!(tokens[0], Token::Doc);
    }

    #[test]
    fn scans_braces() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("{ }").unwrap();

        assert_eq!(tokens.len(), 2);
        matches!(tokens[0], Token::LeftBrace);
        matches!(tokens[1], Token::RightBrace);
    }

    #[test]
    fn scans_string_literal() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("\"hello\"").unwrap();

       
    }

    #[test]
    fn invalid_character_errors() {
        let scanner = Scanner::new();
        let result = scanner.tokenize("@");

        assert!(matches!(result, Err(ScannerError::InvalidToken('@'))));
    }

    #[test]
    fn unterminated_string_errors() {
        let scanner = Scanner::new();
        let result = scanner.tokenize("\"oops");

        assert!(matches!(result, Err(ScannerError::UnterminatedToken('"'))));
    }
}
