pub mod scanner;
mod scanner_tests;

#[derive(Debug)]
pub enum Token {
    // Structural keywords
    Doc,
    Meta,
    Section,

    // Block elements
    P,
    Ul,
    Ol,
    Li,

    // Inline elements
    Link,
    Ref,

    // Resource and linking keywords
    Use,
    As,
    To,

    // Media related keywords
    File,
    Image,
    Src,
    Alt,
    Fallback,

    // Content and control keywords
    Text,
    Maybe,

    // Literals
    Identifier(String),
    StringLiteral(String),

    // Multi-character tokens
    TripleQuote,

    // Single-character tokens
    LeftBrace,
    RightBrace,
    Hash,
    Dot,
    Colon,
}
