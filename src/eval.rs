use std::collections::HashMap;
use std::io;

use crate::ast::{CallExpression, Expression, Program, Statement};
use crate::error::EvalError;
use crate::object::Object;

pub type EvalResult = Result<Object, EvalError>;

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
    pub fn eval_infix_integer(&self, left: i64, operator: &str, right: i64) -> EvalResult {
        let obj = match operator {
            "+" => Object::Integer(left + right),
            "-" => Object::Integer(left - right),
            "*" => Object::Integer(left * right),
            "/" => {
                if right == 0 {
                    return Err(EvalError::new("division by zero"));
                }
                Object::Integer(left / right)
            }
            "==" => Object::Boolean(left == right),
            "!=" => Object::Boolean(left != right),
            "<" => Object::Boolean(left < right),
            ">" => Object::Boolean(left > right),
            "<=" => Object::Boolean(left <= right),
            ">=" => Object::Boolean(left >= right),
            other => {
                return Err(EvalError::new(format!(
                    "unknown integer operator: `{other}`"
                )));
            }
        };
        Ok(obj)
    }

    pub fn eval_infix_boolean(&self, left: bool, operator: &str, right: bool) -> EvalResult {
        match operator {
            "&&" => Ok(Object::Boolean(left && right)),
            "||" => Ok(Object::Boolean(left || right)),
            "==" => Ok(Object::Boolean(left == right)),
            "!=" => Ok(Object::Boolean(left != right)),
            other => Err(EvalError::new(format!(
                "unknown boolean operator: `{other}`"
            ))),
        }
    }

    pub fn eval_expression(
        &mut self,
        expression: &Expression,
        environment: &mut Environment,
    ) -> EvalResult {
        match expression {
            Expression::Integer(value) => Ok(Object::Integer(*value as i64)),
            Expression::Boolean(value) => Ok(Object::Boolean(*value)),
            Expression::String(str) => Ok(Object::String(str.clone())),
            Expression::Identifier(identifier) => environment
                .bindings
                .get(&identifier.value)
                .cloned()
                .ok_or_else(|| {
                    EvalError::new(format!("undefined identifier: `{}`", identifier.value))
                }),
            Expression::Prefix(prefix) => {
                let right = self.eval_expression(&prefix.right, environment)?;

                match (prefix.operator.as_str(), &right) {
                    ("!", Object::Boolean(value)) => Ok(Object::Boolean(!value)),
                    ("-", Object::Integer(value)) => Ok(Object::Integer(-value)),
                    (op, _) => Err(EvalError::new(format!(
                        "unsupported prefix `{op}` on value `{right}`"
                    ))),
                }
            }
            Expression::Infix(infix) => {
                let left = self.eval_expression(&infix.left, environment)?;
                let right = self.eval_expression(&infix.right, environment)?;

                match (&left, &right) {
                    (Object::Integer(l), Object::Integer(r)) => {
                        self.eval_infix_integer(*l, &infix.operator, *r)
                    }
                    (Object::Boolean(l), Object::Boolean(r)) => {
                        self.eval_infix_boolean(*l, &infix.operator, *r)
                    }
                    _ => Err(EvalError::new(format!(
                        "type mismatch: `{left}` {} `{right}`",
                        infix.operator
                    ))),
                }
            }
            Expression::Function(func) => {
                let obj = Object::Function {
                    name: func.name.value.clone(),
                    parameters: func.parameters.iter().map(|v| v.value.clone()).collect(),
                    body: func.body.clone(),
                };
                self.functions.insert(func.name.value.clone(), obj);
                Ok(Object::Null)
            }
            Expression::Call(call) => self.eval_call(call, environment),
        }
    }

    fn eval_call(&mut self, call: &CallExpression, environment: &mut Environment) -> EvalResult {
        if call.name.value == "write" {
            let mut parts = Vec::with_capacity(call.arguments.len());
            for argument in &call.arguments {
                parts.push(self.eval_expression(argument, environment)?.to_string());
            }
            println!("{}", parts.join(" "));
            return Ok(Object::Null);
        }

        if call.name.value == "read" {
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .map_err(|e| EvalError::new(format!("failed to read from stdin: {e}")))?;
            return Ok(Object::Return(Box::new(Object::String(buf))));
        }

        if call.name.value == "read_int" {
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .map_err(|e| EvalError::new(format!("failed to read from stdin: {e}")))?;
            let trimmed = buf.trim();
            let num = trimmed.parse::<i64>().map_err(|e| {
                EvalError::new(format!("read_int: `{trimmed}` is not an integer ({e})"))
            })?;
            return Ok(Object::Return(Box::new(Object::Integer(num))));
        }

        let function = self
            .functions
            .get(&call.name.value)
            .cloned()
            .ok_or_else(|| {
                EvalError::new(format!("undefined function: `{}`", call.name.value))
            })?;

        let Object::Function {
            name: _name,
            parameters,
            body,
        } = function
        else {
            return Err(EvalError::new(format!(
                "`{}` is not callable",
                call.name.value
            )));
        };

        if call.arguments.len() != parameters.len() {
            return Err(EvalError::new(format!(
                "function `{}` expected {} argument(s), got {}",
                call.name.value,
                parameters.len(),
                call.arguments.len()
            )));
        }

        let mut isolated_env = Environment {
            bindings: environment.bindings.clone(),
        };

        for (parameter, argument) in parameters.iter().zip(call.arguments.iter()) {
            let obj = self.eval_expression(argument, environment)?;
            isolated_env.bindings.insert(parameter.clone(), obj);
        }

        match self.eval_block(&body, &mut isolated_env)? {
            Object::Return(object) => Ok(*object),
            other => Ok(other),
        }
    }

    fn eval_block(
        &mut self,
        statements: &[Statement],
        environment: &mut Environment,
    ) -> EvalResult {
        let mut result = Object::Null;
        for statement in statements {
            result = self.eval_statement(statement, environment)?;
            if matches!(result, Object::Return(_)) {
                return Ok(result);
            }
        }
        Ok(result)
    }

    pub fn eval_statement(
        &mut self,
        statement: &Statement,
        environment: &mut Environment,
    ) -> EvalResult {
        match statement {
            Statement::Expression(expression_statement) => {
                self.eval_expression(&expression_statement.expression, environment)
            }
            Statement::Return(return_statement) => {
                let exp = self.eval_expression(&return_statement.return_value, environment)?;
                Ok(Object::Return(Box::new(exp)))
            }
            Statement::Val(val_statement) => {
                let value = self.eval_expression(&val_statement.value, environment)?;
                environment
                    .bindings
                    .insert(val_statement.name.value.clone(), value);
                Ok(Object::Null)
            }
            Statement::If(if_statement) => {
                let condition = self.eval_expression(&if_statement.condition, environment)?;

                let Object::Boolean(condition) = condition else {
                    return Err(EvalError::new(format!(
                        "if condition must be boolean, got `{condition}`"
                    )));
                };

                let branch = if condition {
                    &if_statement.consequence
                } else {
                    &if_statement.alternative
                };

                self.eval_block(branch, environment)
            }
            Statement::For(for_statement) => {
                let mut isolated_environment = Environment {
                    bindings: environment.bindings.clone(),
                };

                self.eval_statement(&for_statement.initialization, &mut isolated_environment)?;

                loop {
                    let condition_value = self
                        .eval_expression(&for_statement.condition, &mut isolated_environment)?;

                    let Object::Boolean(value) = condition_value else {
                        return Err(EvalError::new(format!(
                            "for condition must be boolean, got `{condition_value}`"
                        )));
                    };

                    if !value {
                        break;
                    }

                    self.eval_block(&for_statement.body, &mut isolated_environment)?;
                    self.eval_statement(&for_statement.update, &mut isolated_environment)?;
                }
                Ok(Object::Null)
            }
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Vec<Object>, EvalError> {
        let mut environment = Environment::new();
        let mut results = Vec::with_capacity(program.statements.len());
        for statement in &program.statements {
            results.push(self.eval_statement(statement, &mut environment)?);
        }
        Ok(results)
    }

    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}
