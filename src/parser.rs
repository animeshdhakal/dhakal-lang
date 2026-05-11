use crate::{
    ast::{
        CallExpression, Expression, ExpressionStatement, FunctionExpression, Identifier,
        IfStatement, InfixExpression, LetStatement, PrefixExpression, Program, ReturnStatement,
        Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,

    pub errors: Vec<String>,
}

#[derive(PartialEq, PartialOrd, Clone, Debug, Copy)]
pub enum Precedence {
    Lowest = 1,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

pub fn get_precedence(token: &Token) -> Precedence {
    match token.token_type {
        TokenType::Equals => Precedence::Equals,
        TokenType::NotEquals => Precedence::Equals,

        TokenType::GreaterThan => Precedence::LessGreater,
        TokenType::LessThan => Precedence::LessGreater,
        TokenType::GreaterThanOrEqual => Precedence::LessGreater,
        TokenType::LessThanOrEqual => Precedence::LessGreater,

        TokenType::Plus => Precedence::Sum,
        TokenType::Minus => Precedence::Sum,

        TokenType::Slash => Precedence::Product,
        TokenType::Asterisk => Precedence::Product,

        TokenType::LeftParenthesis => Precedence::Call,

        _ => Precedence::Lowest,
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::Illegal, String::new()),
            peek_token: Token::new(TokenType::Illegal, String::new()),
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Identifier) {
            self.errors
                .push(format!("Expected Identifier but not found"));
            return None;
        }

        let name = self.current_token.literal.clone();

        if !self.expect_peek(TokenType::Assign) {
            self.errors.push(format!(
                "Expected Assignment operator but found {:?}",
                self.peek_token.token_type
            ));
            return None;
        }

        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;

        Some(Statement::Let(LetStatement {
            value: expression,
            name: Identifier { value: name },
        }))
    }

    pub fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RightParenthesis) {
            return None;
        }

        Some(expression)
    }

    pub fn parse_if_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::LeftParenthesis) {
            return None;
        }

        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RightParenthesis) {
            return None;
        }

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        let mut alternative: Vec<Statement> = Vec::new();

        if self.peek_token.token_type == TokenType::Else {
            self.next_token();

            if !self.expect_peek(TokenType::LeftBrace) {
                return None;
            }

            alternative = self.parse_block_statement();
        }

        Some(Statement::If(IfStatement {
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    pub fn parse_function_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let name = Identifier {
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::LeftParenthesis) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let block = self.parse_block_statement();

        Some(Expression::Function(FunctionExpression {
            name,
            body: block,
            parameters,
        }))
    }

    pub fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut arguments: Vec<Expression> = Vec::new();

        if self.expect_peek(TokenType::RightParenthesis) {
            return arguments;
        }

        self.next_token();

        match self.parse_expression(Precedence::Lowest) {
            Some(expression) => arguments.push(expression),
            None => return Vec::new(),
        }

        while self.peek_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();

            let expression = self.parse_expression(Precedence::Lowest);

            if let Some(expression) = expression {
                arguments.push(expression);
            } else {
                return Vec::new();
            }
        }

        if !self.expect_peek(TokenType::RightParenthesis) {
            return Vec::new();
        }

        arguments
    }

    pub fn parse_call_expression(&mut self, left: Expression) -> Option<Expression> {
        let Expression::Identifier(identifier) = left else {
            return None;
        };

        let arguments = self.parse_call_arguments();

        Some(Expression::Call(CallExpression {
            name: identifier,
            arguments,
        }))
    }

    pub fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers: Vec<Identifier> = Vec::new();

        if self.expect_peek(TokenType::RightParenthesis) {
            return identifiers;
        }

        self.next_token();

        identifiers.push(Identifier {
            value: self.current_token.literal.clone(),
        });

        while self.peek_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();

            identifiers.push(Identifier {
                value: self.current_token.literal.clone(),
            });
        }

        if !self.expect_peek(TokenType::RightParenthesis) {
            return Vec::new();
        }

        identifiers
    }

    pub fn parse_block_statement(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        self.next_token();

        while self.current_token.token_type != TokenType::RightBrace
            && self.current_token.token_type != TokenType::Eof
        {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        statements
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest)?;

        Some(Statement::Return(ReturnStatement { return_value }))
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Expression(ExpressionStatement { expression }))
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        match self.current_token.token_type {
            TokenType::Identifier => self.parse_identifier(),
            TokenType::Integer => self.parse_integer(),
            TokenType::String => self.parse_string(),
            TokenType::True => self.parse_boolean(),
            TokenType::False => self.parse_boolean(),
            TokenType::Bang => self.parse_prefix(),
            TokenType::Minus => self.parse_prefix(),
            TokenType::LeftParenthesis => self.parse_grouped_expression(),
            TokenType::Function => self.parse_function_expression(),
            _ => None,
        }
    }

    pub fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        match self.current_token.token_type {
            TokenType::Asterisk => self.parse_infix(left),
            TokenType::Slash => self.parse_infix(left),
            TokenType::Plus => self.parse_infix(left),
            TokenType::Minus => self.parse_infix(left),
            TokenType::Equals => self.parse_infix(left),
            TokenType::NotEquals => self.parse_infix(left),
            TokenType::GreaterThan => self.parse_infix(left),
            TokenType::LessThan => self.parse_infix(left),
            TokenType::GreaterThanOrEqual => self.parse_infix(left),
            TokenType::LessThanOrEqual => self.parse_infix(left),
            TokenType::LeftParenthesis => self.parse_call_expression(left),
            _ => None,
        }
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_expression = self.parse_prefix_expression()?;

        while self.peek_token.token_type != TokenType::Semicolon
            && (precedence as u32) < (get_precedence(&self.peek_token) as u32)
        {
            self.next_token();

            if let Some(infix) = self.parse_infix_expression(left_expression.clone()) {
                left_expression = infix;
            }
        }

        Some(left_expression)
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            _ => self.parse_expression_statement(),
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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.token_type != TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        program
    }

    pub fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            value: self.current_token.literal.clone(),
        }))
    }

    pub fn parse_integer(&mut self) -> Option<Expression> {
        let integer: u64 = self.current_token.literal.parse::<u64>().unwrap();
        Some(Expression::Integer(integer))
    }

    pub fn parse_string(&mut self) -> Option<Expression> {
        Some(Expression::String(self.current_token.literal.clone()))
    }

    pub fn parse_boolean(&mut self) -> Option<Expression> {
        let value = self.current_token.token_type == TokenType::True;
        Some(Expression::Boolean(value))
    }

    pub fn parse_prefix(&mut self) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;

        Some(Expression::Prefix(PrefixExpression {
            operator,
            right: Box::new(right),
        }))
    }

    pub fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
        let precedence = get_precedence(&self.current_token);
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Some(Expression::Infix(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }
}
