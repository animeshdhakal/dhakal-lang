use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::Equals, "==".to_string())
                } else {
                    Token::new(TokenType::Assign, "=".to_string())
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::new(TokenType::NotEquals, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, "!".to_string())
                }
            }
            b';' => Token::new(TokenType::Semicolon, ";".to_string()),
            b'(' => Token::new(TokenType::LeftParenthesis, "(".to_string()),
            b')' => Token::new(TokenType::RightParenthesis, ")".to_string()),
            b',' => Token::new(TokenType::Comma, ",".to_string()),
            b'+' => Token::new(TokenType::Plus, "+".to_string()),
            b'-' => Token::new(TokenType::Minus, "-".to_string()),
            b'{' => Token::new(TokenType::LeftBrace, "{".to_string()),
            b'}' => Token::new(TokenType::RightBrace, "}".to_string()),
            b'[' => Token::new(TokenType::LeftBracket, "[".to_string()),
            b']' => Token::new(TokenType::RightBracket, "]".to_string()),
            b'>' => Token::new(TokenType::GreaterThan, ">".to_string()),
            b'<' => Token::new(TokenType::LessThan, "<".to_string()),
            0 => Token::new(TokenType::Eof, "".to_string()),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    let token_type = match identifier.as_str() {
                        "fn" => TokenType::Function,
                        "let" => TokenType::Let,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "return" => TokenType::Return,
                        _ => TokenType::Identifier,
                    };
                    return Token::new(token_type, identifier);
                } else if self.ch.is_ascii_digit() {
                    return Token::new(TokenType::Integer, self.read_number());
                } else {
                    Token::new(TokenType::Illegal, (self.ch as char).to_string())
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
