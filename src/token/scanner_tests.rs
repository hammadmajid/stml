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

        assert_eq!(tokens.len(), 1);
        assert!(matches!(&tokens[0], Token::StringLiteral(s) if s == "hello"));
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

    #[test]
    fn scans_section_with_id() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("section #intro { }").unwrap();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::Section));
        assert!(matches!(tokens[1], Token::Hash));
        assert!(matches!(&tokens[2], Token::Identifier(s) if s == "intro"));
        assert!(matches!(tokens[3], Token::LeftBrace));
        assert!(matches!(tokens[4], Token::RightBrace));
    }

    #[test]
    fn scans_paragraph() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("p \"Single line paragraph.\"").unwrap();

        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0], Token::P));
        assert!(matches!(&tokens[1], Token::StringLiteral(s) if s == "Single line paragraph."));
    }

    #[test]
    fn scans_triple_quoted_string() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("\"\"\"multi\nline\"\"\"").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(&tokens[0], Token::StringLiteral(s) if s == "multi\nline"))
    }

    #[test]
    fn scans_list_keywords() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("ol ul li").unwrap();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Token::Ol));
        assert!(matches!(tokens[1], Token::Ul));
        assert!(matches!(tokens[2], Token::Li));
    }

    #[test]
    fn scans_ref_with_type() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("ref section:#intro").unwrap();

        assert!(tokens.len() >= 4);
        assert!(matches!(tokens[0], Token::Ref));
        assert!(matches!(tokens[1], Token::Section));
        assert!(matches!(tokens[2], Token::Colon));
        assert!(matches!(tokens[3], Token::Hash));
    }

    #[test]
    fn scans_image_block() {
        let scanner = Scanner::new();
        let tokens = scanner
            .tokenize("image { src \"url\" alt \"text\" fallback { } }")
            .unwrap();

        assert!(tokens.iter().any(|t| matches!(t, Token::Image)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Src)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Alt)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Fallback)));
    }

    #[test]
    fn scans_link_block() {
        let scanner = Scanner::new();
        let tokens = scanner
            .tokenize("link { to \"https://example.com\" text \"example\" }")
            .unwrap();

        assert!(tokens.iter().any(|t| matches!(t, Token::Link)));
        assert!(tokens.iter().any(|t| matches!(t, Token::To)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Text)));
    }

    #[test]
    fn scans_maybe_keyword() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("maybe image { }").unwrap();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], Token::Maybe));
        assert!(matches!(tokens[1], Token::Image));
    }

    #[test]
    fn scans_complex_document() {
        let scanner = Scanner::new();
        let input = r#"doc {
            section #intro {
                p "Hello world"
            }
        }"#;
        let tokens = scanner.tokenize(input).unwrap();

        assert_eq!(tokens.len(), 10);
        assert!(matches!(tokens[0], Token::Doc));
        assert!(matches!(tokens[1], Token::LeftBrace));
        assert!(matches!(tokens[2], Token::Section));
        assert!(matches!(tokens[3], Token::Hash));
        assert!(matches!(&tokens[4], Token::Identifier(s) if s == "intro"));
        assert!(matches!(tokens[5], Token::LeftBrace));
        assert!(matches!(tokens[6], Token::P));
        assert!(matches!(&tokens[7], Token::StringLiteral(s) if s == "Hello world"));
        assert!(matches!(tokens[8], Token::RightBrace));
        assert!(matches!(tokens[9], Token::RightBrace));
    }

    #[test]
    fn scans_all_punctuation() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("{ } # . :").unwrap();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::LeftBrace));
        assert!(matches!(tokens[1], Token::RightBrace));
        assert!(matches!(tokens[2], Token::Hash));
        assert!(matches!(tokens[3], Token::Dot));
        assert!(matches!(tokens[4], Token::Colon));
    }

    #[test]
    fn whitespace_is_ignored() {
        let scanner1 = Scanner::new();
        let tokens1 = scanner1.tokenize("doc\"title\"{}").unwrap();
        let scanner2 = Scanner::new();
        let tokens2 = scanner2.tokenize("doc  \"title\"  {  }").unwrap();

        assert_eq!(tokens1.len(), tokens2.len());
    }

    #[test]
    fn string_with_spaces() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("\"hello world\"").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(&tokens[0], Token::StringLiteral(s) if s == "hello world"));
    }

    #[test]
    fn scans_nested_braces() {
        let scanner = Scanner::new();
        let tokens = scanner.tokenize("{ { } }").unwrap();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], Token::LeftBrace));
        assert!(matches!(tokens[1], Token::LeftBrace));
        assert!(matches!(tokens[2], Token::RightBrace));
        assert!(matches!(tokens[3], Token::RightBrace));
    }
}
