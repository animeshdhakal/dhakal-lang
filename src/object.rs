use crate::ast::Statement;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Return(Box<Object>),
    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Null,
}
