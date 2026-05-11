use std::collections::{BinaryHeap, HashMap};

use crate::ast::{Expression, Program, Statement};
use crate::object::Object;

pub struct Environment {
    bindings: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}

pub struct Eval {
    functions: HashMap<String, Object>,
}

impl Eval {
    pub fn eval_infix_integer(&self, left: i64, operator: &str, right: i64) -> Object {
        match operator {
            "+" => Object::Integer(left + right),
            "-" => Object::Integer(left - right),
            "/" => Object::Integer(left / right),
            "*" => Object::Integer(left * right),
            "==" => Object::Boolean(left == right),
            "!=" => Object::Boolean(left != right),
            "<" => Object::Boolean(left < right),
            ">" => Object::Boolean(left > right),
            "<=" => Object::Boolean(left <= right),
            ">=" => Object::Boolean(left >= right),
            _ => Object::Null,
        }
    }

    pub fn eval_expression(
        &mut self,
        expression: &Expression,
        environment: &mut Environment,
    ) -> Object {
        match expression {
            Expression::Integer(value) => Object::Integer(*value as i64),
            Expression::Boolean(value) => Object::Boolean(*value),
            Expression::String(str) => Object::String(str.clone()),
            Expression::Identifier(identifier) => environment
                .bindings
                .get(&identifier.value)
                .unwrap_or(&Object::Null)
                .clone(),
            Expression::Prefix(prefix) => {
                let right = self.eval_expression(&prefix.right, environment);

                if prefix.operator == "!" {
                    if let Object::Boolean(value) = right {
                        return Object::Boolean(!value);
                    }
                }

                if prefix.operator == "-" {
                    if let Object::Integer(value) = right {
                        return Object::Integer(-value);
                    }
                }

                Object::Null
            }
            Expression::Infix(infix) => {
                let left = self.eval_expression(&infix.left, environment);
                let right = self.eval_expression(&infix.right, environment);

                if let Object::Integer(left_val) = left
                    && let Object::Integer(right_val) = right
                {
                    return self.eval_infix_integer(left_val, &infix.operator, right_val);
                }

                Object::Null
            }
            Expression::Function(func) => {
                let obj = Object::Function {
                    name: func.name.value.clone(),
                    parameters: func.parameters.iter().map(|v| v.value.clone()).collect(),
                    body: func.body.clone(),
                };
                self.functions.insert(func.name.value.clone(), obj);
                Object::Null
            }
            Expression::Call(call) => {
                if call.name.value == "print" {
                    let parts: Vec<String> = call
                        .arguments
                        .iter()
                        .map(|argument| self.eval_expression(argument, environment).to_string())
                        .collect();
                    println!("{}", parts.join(" "));
                    return Object::Null;
                }

                let Some(function) = self.functions.get(&call.name.value).cloned() else {
                    return Object::Null;
                };
                let Object::Function {
                    name: _name,
                    parameters,
                    body,
                } = function
                else {
                    return Object::Null;
                };

                let mut isolated_env = Environment {
                    bindings: environment.bindings.clone(),
                };

                if call.arguments.len() != parameters.len() {
                    return Object::Null;
                }

                for i in 0..parameters.len() {
                    let argument = call.arguments[i].clone();
                    let obj = self.eval_expression(&argument, environment);
                    isolated_env
                        .bindings
                        .insert(parameters.get(i).unwrap().clone(), obj);
                }

                match self.eval_block(&body, &mut isolated_env) {
                    Object::Return(object) => *object,
                    other => other,
                }
            }
        }
    }

    fn eval_block(&mut self, statements: &[Statement], environment: &mut Environment) -> Object {
        let mut result = Object::Null;
        for statement in statements {
            result = self.eval_statement(statement, environment);
            if matches!(result, Object::Return(_)) {
                return result;
            }
        }
        result
    }

    pub fn eval_statement(
        &mut self,
        statement: &Statement,
        environment: &mut Environment,
    ) -> Object {
        match statement {
            Statement::Expression(expression_statement) => {
                self.eval_expression(&expression_statement.expression, environment)
            }
            Statement::Return(return_statement) => {
                let exp = self.eval_expression(&return_statement.return_value, environment);
                Object::Return(Box::new(exp))
            }
            Statement::Val(val_statement) => {
                let value = self.eval_expression(&val_statement.value, environment);
                environment
                    .bindings
                    .insert(val_statement.name.value.clone(), value);
                Object::Null
            }
            Statement::If(if_statement) => {
                let condition = self.eval_expression(&if_statement.condition, environment);

                let Object::Boolean(condition) = condition else {
                    return Object::Null;
                };

                let branch = if condition {
                    &if_statement.consequence
                } else {
                    &if_statement.alternative
                };

                self.eval_block(branch, environment)
            }

            Statement::For(for_statement) => {
                let mut isolated_enivornment = Environment {
                    bindings: environment.bindings.clone(),
                };

                self.eval_statement(&for_statement.initialization, &mut isolated_enivornment);

                loop {
                    let Object::Boolean(value) =
                        self.eval_expression(&for_statement.condition, &mut isolated_enivornment)
                    else {
                        return Object::Null;
                    };

                    if !value {
                        break;
                    }

                    self.eval_block(&for_statement.body, &mut isolated_enivornment);
                    self.eval_statement(&for_statement.update, &mut isolated_enivornment);
                }
                Object::Null
            }
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Vec<Object> {
        let mut environment = Environment::new();
        program
            .statements
            .iter()
            .map(|statement| self.eval_statement(statement, &mut environment))
            .collect()
    }

    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}
