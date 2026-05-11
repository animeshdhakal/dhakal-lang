use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
            line: 1,
            column: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;

        if self.ch == b'\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    pub fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let starting_position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        self.input[starting_position..self.position].to_string()
    }

    pub fn read_number(&mut self) -> String {
        let starting_position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[starting_position..self.position].to_string()
    }

    pub fn read_string(&mut self) -> String {
        self.read_char();
        let starting_position = self.position;
        while self.ch != b'"' && self.ch != 0 {
            self.read_char();
        }
        self.input[starting_position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;
        let mk = |t, l: &str| Token::with_position(t, l.to_string(), line, column);

        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    mk(TokenType::Equals, "==")
                } else {
                    mk(TokenType::Assign, "=")
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    mk(TokenType::NotEquals, "!=")
                } else {
                    mk(TokenType::Bang, "!")
                }
            }
            b';' => mk(TokenType::Semicolon, ";"),
            b'(' => mk(TokenType::LeftParenthesis, "("),
            b')' => mk(TokenType::RightParenthesis, ")"),
            b',' => mk(TokenType::Comma, ","),
            b'+' => mk(TokenType::Plus, "+"),
            b'-' => mk(TokenType::Minus, "-"),
            b'*' => mk(TokenType::Asterisk, "*"),
            b'/' => mk(TokenType::Slash, "/"),
            b'`' => {
                self.read_char();
                while self.ch != b'`' && self.ch != 0 {
                    self.read_char();
                }
                if self.ch == b'`' {
                    self.read_char();
                }
                return self.next_token();
            }
            b'{' => mk(TokenType::LeftBrace, "{"),
            b'}' => mk(TokenType::RightBrace, "}"),
            b'[' => mk(TokenType::LeftBracket, "["),
            b']' => mk(TokenType::RightBracket, "]"),
            b'&' => {
                self.read_char();
                mk(TokenType::LogicalAnd, "&&")
            }
            b'|' => {
                self.read_char();
                mk(TokenType::LogicalOr, "||")
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    mk(TokenType::GreaterThanOrEqual, ">=")
                } else {
                    mk(TokenType::GreaterThan, ">")
                }
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    mk(TokenType::LessThanOrEqual, "<=")
                } else {
                    mk(TokenType::LessThan, "<")
                }
            }
            b'"' => {
                let s = self.read_string();
                Token::with_position(TokenType::String, s, line, column)
            }
            0 => Token::with_position(TokenType::Eof, String::new(), line, column),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    let token_type = match identifier.as_str() {
                        "func" => TokenType::Function,
                        "val" => TokenType::Val,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "return" => TokenType::Return,
                        "for" => TokenType::For,
                        _ => TokenType::Identifier,
                    };
                    return Token::with_position(token_type, identifier, line, column);
                } else if self.ch.is_ascii_digit() {
                    let number = self.read_number();
                    return Token::with_position(TokenType::Integer, number, line, column);
                } else {
                    Token::with_position(
                        TokenType::Illegal,
                        (self.ch as char).to_string(),
                        line,
                        column,
                    )
                }
            }
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
