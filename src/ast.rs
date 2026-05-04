pub struct Identifier {
    pub value: String,
}

pub enum Expression {
    Identifier(Identifier),
}

pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

pub struct ReturnStatement {
    pub return_value: Expression,
}

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
