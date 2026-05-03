#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    Identifier,
    Integer,

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Bang,
    Slash,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,

    // Delimiters
    Comma,
    Semicolon,

    // Symbols
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,

    // Keywords
    Function,
    Let,
    True,
    False,
    Return,
    Else,
    If,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}
