pub mod scanner;
mod scanner_tests;

#[derive(Debug, PartialEq, Clone)]
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
    To,
    Src,

    // Media related keywords
    Image,
    Alt,
    Fallback,

    // Content and control keywords
    Text,
    Maybe,

    // Literals
    Identifier(String),
    StringLiteral(String),

    // Single-character tokens
    LeftBrace,
    RightBrace,
    Hash,
    Dot,
    Colon,
    
    // End of file
    EOF,
}
