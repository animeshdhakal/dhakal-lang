use std::collections::HashMap;
use std::io;

use crate::ast::{CallExpression, Expression, Program, Statement};
use crate::error::EvalError;
use crate::object::{ArrayRef, Object};

pub type EvalResult = Result<Object, EvalError>;

#[derive(Default)]
pub struct Environment {
    bindings: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Eval {
    functions: HashMap<String, Object>,
    environment: Environment,
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
            Expression::Array(arr) => {
                let mut objects = Vec::with_capacity(arr.len());
                for element in arr {
                    objects.push(self.eval_expression(element, environment)?);
                }
                Ok(Object::array(objects))
            }
        }
    }

    fn expect_arity(name: &str, args: &[Expression], expected: usize) -> Result<(), EvalError> {
        if args.len() != expected {
            return Err(EvalError::new(format!(
                "{name}: expected {expected} argument(s), got {}",
                args.len()
            )));
        }
        Ok(())
    }

    fn array_handle(name: &str, target: Object) -> Result<ArrayRef, EvalError> {
        match target {
            Object::Array(items) => Ok(items),
            other => Err(EvalError::new(format!(
                "{name}: expected array, got `{other}`"
            ))),
        }
    }

    fn resolve_index(name: &str, value: Object, len: usize) -> Result<usize, EvalError> {
        let Object::Integer(index) = value else {
            return Err(EvalError::new(format!(
                "{name}: index must be an integer, got `{value}`"
            )));
        };
        if index < 0 || (index as usize) >= len {
            return Err(EvalError::new(format!(
                "{name}: index {index} out of bounds (len {len})"
            )));
        }
        Ok(index as usize)
    }

    fn builtin_push(&mut self, call: &CallExpression, env: &mut Environment) -> EvalResult {
        Self::expect_arity("push", &call.arguments, 2)?;
        let target = self.eval_expression(&call.arguments[0], env)?;
        let value = self.eval_expression(&call.arguments[1], env)?;
        let items = Self::array_handle("push", target)?;
        items.borrow_mut().push(value);
        Ok(Object::Null)
    }

    fn builtin_pop(&mut self, call: &CallExpression, env: &mut Environment) -> EvalResult {
        Self::expect_arity("pop", &call.arguments, 1)?;
        let target = self.eval_expression(&call.arguments[0], env)?;
        let items = Self::array_handle("pop", target)?;
        Ok(items.borrow_mut().pop().unwrap_or(Object::Null))
    }

    fn builtin_set(&mut self, call: &CallExpression, env: &mut Environment) -> EvalResult {
        Self::expect_arity("set", &call.arguments, 3)?;
        let target = self.eval_expression(&call.arguments[0], env)?;
        let index_value = self.eval_expression(&call.arguments[1], env)?;
        let value = self.eval_expression(&call.arguments[2], env)?;
        let items = Self::array_handle("set", target)?;
        let len = items.borrow().len();
        let index = Self::resolve_index("set", index_value, len)?;
        items.borrow_mut()[index] = value;
        Ok(Object::Null)
    }

    fn builtin_get(&mut self, call: &CallExpression, env: &mut Environment) -> EvalResult {
        Self::expect_arity("get", &call.arguments, 2)?;
        let target = self.eval_expression(&call.arguments[0], env)?;
        let index_value = self.eval_expression(&call.arguments[1], env)?;
        let items = Self::array_handle("get", target)?;
        let borrowed = items.borrow();
        let index = Self::resolve_index("get", index_value, borrowed.len())?;
        Ok(borrowed[index].clone())
    }

    fn builtin_len(&mut self, call: &CallExpression, env: &mut Environment) -> EvalResult {
        Self::expect_arity("len", &call.arguments, 1)?;
        let target = self.eval_expression(&call.arguments[0], env)?;
        let length = match target {
            Object::Array(items) => items.borrow().len(),
            Object::String(value) => value.chars().count(),
            other => {
                return Err(EvalError::new(format!(
                    "len: expected array or string, got `{other}`"
                )));
            }
        };
        Ok(Object::Integer(length as i64))
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
            let trimmed = buf.trim_end_matches(['\r', '\n']).to_string();
            return Ok(Object::String(trimmed));
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
            return Ok(Object::Integer(num));
        }

        match call.name.value.as_str() {
            "push" => return self.builtin_push(call, environment),
            "pop" => return self.builtin_pop(call, environment),
            "set" => return self.builtin_set(call, environment),
            "get" => return self.builtin_get(call, environment),
            "len" => return self.builtin_len(call, environment),
            _ => {}
        }

        let function = self
            .functions
            .get(&call.name.value)
            .cloned()
            .ok_or_else(|| EvalError::new(format!("undefined function: `{}`", call.name.value)))?;

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
                let value =
                    self.eval_expression(&expression_statement.expression, environment)?;
                if expression_statement.has_semicolon {
                    Ok(Object::Null)
                } else {
                    Ok(value)
                }
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
                self.eval_statement(&for_statement.initialization, environment)?;

                loop {
                    let condition_value =
                        self.eval_expression(&for_statement.condition, environment)?;

                    let Object::Boolean(value) = condition_value else {
                        return Err(EvalError::new(format!(
                            "for condition must be boolean, got `{condition_value}`"
                        )));
                    };

                    if !value {
                        break;
                    }

                    let body_result = self.eval_block(&for_statement.body, environment)?;
                    if matches!(body_result, Object::Return(_)) {
                        return Ok(body_result);
                    }
                    self.eval_statement(&for_statement.update, environment)?;
                }
                Ok(Object::Null)
            }
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Vec<Object>, EvalError> {
        let mut environment = std::mem::take(&mut self.environment);
        let mut results = Vec::with_capacity(program.statements.len());
        for statement in &program.statements {
            match self.eval_statement(statement, &mut environment) {
                Ok(object) => results.push(object),
                Err(error) => {
                    self.environment = environment;
                    return Err(error);
                }
            }
        }
        self.environment = environment;
        Ok(results)
    }

    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            environment: Environment::new(),
        }
    }
}
