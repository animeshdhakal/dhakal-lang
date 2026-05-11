use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::ast::Statement;

pub type ArrayRef = Rc<RefCell<Vec<Object>>>;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Box<Object>),
    Array(ArrayRef),
    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Null,
}

impl Object {
    pub fn array(items: Vec<Object>) -> Self {
        Object::Array(Rc::new(RefCell::new(items)))
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{value}"),
            Object::Boolean(value) => write!(f, "{value}"),
            Object::String(value) => write!(f, "{value}"),
            Object::Return(value) => write!(f, "{value}"),
            Object::Function {
                name, parameters, ..
            } => {
                write!(f, "fn {}({})", name, parameters.join(", "))
            }
            Object::Array(value) => {
                write!(f, "[")?;
                for (i, item) in value.borrow().iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Object::Null => write!(f, "null"),
        }
    }
}
