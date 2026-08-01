use std::collections::HashMap;

use crate::language::{Environment, FunctionError, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Value(Value),
    Variable(String),
    Unary {
        operator: Operator,
        operand: Box<Expression>,
    },
    Binary {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Array(Vec<Expression>),
    FunctionCall {
        name: String,
        args: HashMap<String, Expression>,
        tail: Vec<Expression>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Power,
    Index,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Function error: {0:?}")]
    Function(Box<FunctionError>),
    #[error("Illegal combination of operator and values: {0:?}.")]
    IllegalOperation(Expression),
    #[error("Index out of bounds, value is: {value}, but index is: {index}")]
    IndexOutOfBounds { value: Value, index: usize },
    #[error("Expected a positive integer, found: {0}.")]
    UnexpecteNonPositiveInteger(i64),
    #[error("Unknown variable: {0}.")]
    UnknownVariable(String),
    #[error("Unknown function: {0}.")]
    UnknownFunction(String),
}

impl EvaluationError {
    fn function(error: FunctionError) -> Self {
        Self::Function(Box::new(error))
    }
}

impl Expression {
    /// Evaluates the expression to a concrete `Value`.
    pub fn evaluate(&self, environment: &mut Environment) -> Result<Value, EvaluationError> {
        Ok(match self {
            Expression::Value(value) => value.clone(),
            Expression::Variable(name) => {
                environment.get_variable(name).cloned().unwrap_or_default()
            }
            Expression::Unary { operator, operand } => {
                let value = operand.evaluate(environment)?;
                match (operator, value) {
                    (Operator::Sub, Value::Number(number)) => Value::Number(-number),

                    // Other
                    (operator, value) => {
                        return Err(EvaluationError::IllegalOperation(Expression::Unary {
                            operator: *operator,
                            operand: Box::new(Expression::Value(value)),
                        }));
                    }
                }
            }
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                let left = left.evaluate(environment)?;
                let right = right.evaluate(environment)?;

                match (left, operator, right) {
                    // Number math
                    (Value::Number(left), Operator::Add, Value::Number(right)) => {
                        Value::Number(left + right)
                    }
                    (Value::Number(left), Operator::Sub, Value::Number(right)) => {
                        Value::Number(left - right)
                    }
                    (Value::Number(left), Operator::Mul, Value::Number(right)) => {
                        Value::Number(left * right)
                    }
                    (Value::Number(left), Operator::Div, Value::Number(right)) => {
                        Value::Number(left / right)
                    }
                    (Value::Number(left), Operator::Power, Value::Number(right)) => {
                        let Ok(right) = u32::try_from(right) else {
                            return Err(EvaluationError::UnexpecteNonPositiveInteger(right));
                        };

                        Value::Number(left.pow(right))
                    }

                    // String operations
                    (Value::Number(times), Operator::Mul, Value::String(string))
                    | (Value::String(string), Operator::Mul, Value::Number(times)) => {
                        let Ok(times) = usize::try_from(times) else {
                            return Err(EvaluationError::UnexpecteNonPositiveInteger(times));
                        };

                        Value::String(string.repeat(times))
                    }
                    (Value::String(left), Operator::Add, Value::String(right)) => {
                        Value::String(left + &right)
                    }
                    (Value::String(string), Operator::Index, Value::Number(index)) => {
                        let Ok(index) = usize::try_from(index) else {
                            return Err(EvaluationError::UnexpecteNonPositiveInteger(index));
                        };
                        let sub_string = string.get(index..=index).map(str::to_owned).ok_or(
                            EvaluationError::IndexOutOfBounds {
                                value: Value::String(string),
                                index,
                            },
                        )?;

                        Value::String(sub_string.to_owned())
                    }

                    // Array operations
                    // TODO: Consider not evaluating the whole array and actually only taking the value we need
                    (Value::Array(array), Operator::Index, Value::Number(index)) => {
                        let Ok(index) = usize::try_from(index) else {
                            return Err(EvaluationError::UnexpecteNonPositiveInteger(index));
                        };

                        array
                            .get(index)
                            .cloned()
                            .ok_or(EvaluationError::IndexOutOfBounds {
                                value: Value::Array(array),
                                index,
                            })?
                    }
                    (Value::Array(mut left), Operator::Add, Value::Array(right)) => {
                        left.extend(right);
                        Value::Array(left)
                    }

                    // Other
                    (left, operator, right) => {
                        return Err(EvaluationError::IllegalOperation(Expression::Binary {
                            operator: *operator,
                            left: Box::new(Expression::Value(left)),
                            right: Box::new(Expression::Value(right)),
                        }));
                    }
                }
            }
            Expression::Array(array) => {
                let mut values = Vec::with_capacity(array.len());
                for expression in array {
                    let value = expression.evaluate_or_null(environment);
                    values.push(value);
                }

                Value::Array(values)
            }
            Expression::FunctionCall { name, args, tail } => {
                let function = environment
                    .get_function(name)
                    .ok_or(EvaluationError::UnknownFunction(name.clone()))?;

                let mut call_args = function.default_args();
                for (param, arg) in args {
                    let arg = arg.evaluate(environment)?;
                    call_args
                        .set_arg(param, arg)
                        .map_err(EvaluationError::function)?;
                }
                for expr in tail {
                    let value = expr.evaluate(environment)?;
                    call_args.add_tail(value);
                }

                function
                    .call(call_args, environment)
                    .map_err(EvaluationError::function)?
            }
        })
    }

    /// Like `Self::evaluate` but for simplicity, any illogical/illegal operation evaluates into a `Value::Null`.
    pub fn evaluate_or_null(&self, environment: &mut Environment) -> Value {
        self.evaluate(environment).unwrap_or_default()
    }
}

impl From<Value> for Expression {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}
