use std::collections::HashMap;

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
            Expression::Integer(integer) => Object::Integer(integer.value as i64),
            Expression::Boolean(boolean) => Object::Boolean(boolean.value),
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
            Expression::If(if_expression) => {
                let condition = self.eval_expression(&if_expression.condition, environment);

                let Object::Boolean(condition) = condition else {
                    return Object::Null;
                };

                let branch = if condition {
                    &if_expression.consequence
                } else {
                    &if_expression.alternative
                };

                self.eval_block(branch, environment)
            }
            Expression::Function(func) => {
                let obj = Object::Function {
                    name: func.name.value.clone(),
                    parameters: func.parameters.iter().map(|v| v.value.clone()).collect(),
                    body: func.body.clone(),
                };
                self.functions.insert(func.name.value.clone(), obj.clone());
                obj
            }
            Expression::Call(call) => {
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
            Statement::Let(let_statement) => {
                let value = self.eval_expression(&let_statement.value, environment);
                environment
                    .bindings
                    .insert(let_statement.name.value.clone(), value);
                Object::Null
            }
        }
    }

    pub fn eval_program(&mut self, program: Program) {
        let mut environment = Environment::new();

        for statement in program.statements {
            let object = self.eval_statement(&statement, &mut environment);
            println!("Object: {:?}", object);
        }
    }

    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}
