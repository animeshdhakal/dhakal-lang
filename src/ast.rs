#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub value: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub value: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: Vec<Statement>,
    pub alternative: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Boolean(BooleanLiteral),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}
