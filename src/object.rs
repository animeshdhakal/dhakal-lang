use std::fmt;

use crate::ast::Statement;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Box<Object>),
    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{value}"),
            Object::Boolean(value) => write!(f, "{value}"),
            Object::String(value) => write!(f, "{value}"),
            Object::Return(value) => write!(f, "{value}"),
            Object::Function { name, parameters, .. } => {
                write!(f, "fn {}({})", name, parameters.join(", "))
            }
            Object::Null => write!(f, "null"),
        }
    }
}
