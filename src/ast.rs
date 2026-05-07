#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Boolean(BooleanLiteral),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
