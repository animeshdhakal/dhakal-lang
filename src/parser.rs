use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Parser {
            lexer: lexer,
            current_token: Token::new(TokenType::Illegal, "".to_string()),
            peek_token: Token::new(TokenType::Illegal, "".to_string()),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().clone();
    }

    pub fn wait_for_semicolon(&mut self) {
        while self.current_token.token_type != TokenType::Semicolon {
            self.next_token();
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();

        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let statement = LetStatement {
            name: Identifier {
                value: self.current_token.literal.clone(),
            },
            value: Expression::Identifier(Identifier {
                value: "".to_string(),
            }),
        };

        self.wait_for_semicolon();

        Some(Statement::Let(statement))
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        None
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token.token_type == token_type {
            self.next_token();
            true
        } else {
            false
        }
    }

    pub fn parse_program(&mut self) {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.token_type != TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }
    }
}
