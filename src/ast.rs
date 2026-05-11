#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub value: String,
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
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub consequence: Vec<Statement>,
    pub alternative: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionExpression {
    pub name: Identifier,
    pub body: Vec<Statement>,
    pub parameters: Vec<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallExpression {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    String(String),
    Boolean(bool),
    Integer(u64),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Function(FunctionExpression),
    Call(CallExpression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub return_value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Val(ValStatement),
    Return(ReturnStatement),
    If(IfStatement),
    Expression(ExpressionStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub statements: Vec<Statement>,
}
