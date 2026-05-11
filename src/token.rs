#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    Identifier,
    Integer,
    String,

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    LogicalAnd,
    LogicalOr,
    Bang,
    Slash,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    // Delimiters
    Comma,
    Semicolon,

    // Symbols
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Keywords
    Function,
    Val,
    True,
    False,
    Return,
    Else,
    If,
    For,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
            line: 0,
            column: 0,
        }
    }

    pub fn with_position(
        token_type: TokenType,
        literal: String,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            literal,
            line,
            column,
        }
    }
}
