use std::collections::HashMap;

use crate::{
    language::{Function, FunctionError, SharedEnvironment, Value},
    utils::CloneInner,
};

#[derive(Clone, Debug)]
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
    Function(Function),
    FunctionCall {
        callee: Box<Expression>,
        args: HashMap<String, Expression>,
        tail: Vec<Expression>,
    },
    HashMap(Vec<(Expression, Expression)>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    /* Math */
    Add,
    Sub,
    Mul,
    Div,
    Power,
    /* Indexing */
    Index,
    /* Logical */
    And,
    Or,
    Not,
    /* Comparison */
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Function error: {0:?}")]
    Function(Box<FunctionError>),
    #[error("Illegal combination of operator and values: {0:?}.")]
    IllegalOperation(Expression),
    #[error("Index out of bounds, value is: {value}, but index is: {index}")]
    IndexOutOfBounds { value: Value, index: usize },
    #[error("Only values of type function can be called: {0}")]
    NotCallable(Value),
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
    pub fn evaluate(&self, environment: &SharedEnvironment) -> Result<Value, EvaluationError> {
        Ok(match self {
            Expression::Value(value) => value.clone(),
            Expression::Variable(name) => environment.get_variable(name).unwrap_or_default(),
            Expression::Unary { operator, operand } => {
                let value = operand.evaluate(environment)?;
                match (operator, value) {
                    (Operator::Sub, Value::Number(number)) => Value::Number(-number),
                    (Operator::Not, value) => Value::Bool(!value.is_truthy()),

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

                // Short-circuit logical operators
                match operator {
                    Operator::And => {
                        if !left.is_truthy() {
                            return Ok(Value::Bool(false));
                        }
                        let right_val = right.evaluate(environment)?;
                        return Ok(Value::Bool(right_val.is_truthy()));
                    }
                    Operator::Or => {
                        if left.is_truthy() {
                            return Ok(Value::Bool(true));
                        }
                        let right_val = right.evaluate(environment)?;
                        return Ok(Value::Bool(right_val.is_truthy()));
                    }
                    _ => {}
                }

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

                        Value::new_string(string.repeat(times))
                    }
                    (Value::String(left), Operator::Add, Value::String(right)) => {
                        Value::new_string(left.clone_inner() + &right)
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

                        Value::new_string(sub_string)
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
                    (Value::Array(left), Operator::Add, Value::Array(right)) => {
                        let mut joined = left.clone_inner();
                        joined.extend(right.iter().cloned());
                        Value::new_array(joined)
                    }

                    // HashMap operations
                    (Value::HashMap(map), Operator::Index, key) => {
                        map.get(&key).cloned().unwrap_or_default()
                    }
                    (Value::HashMap(left), Operator::Add, Value::HashMap(right)) => {
                        let mut joined = left.clone_inner();
                        joined.extend(
                            right
                                .iter()
                                .map(|(key, value)| (key.clone(), value.clone())),
                        );
                        Value::new_hash_map(joined)
                    }

                    // Comparisons
                    (left, Operator::Eq, right) => Value::Bool(left == right),
                    (left, Operator::NotEq, right) => Value::Bool(left != right),
                    (Value::Number(left), Operator::Less, Value::Number(right)) => {
                        Value::Bool(left < right)
                    }
                    (Value::Number(left), Operator::LessEq, Value::Number(right)) => {
                        Value::Bool(left <= right)
                    }
                    (Value::Number(left), Operator::Greater, Value::Number(right)) => {
                        Value::Bool(left > right)
                    }
                    (Value::Number(left), Operator::GreaterEq, Value::Number(right)) => {
                        Value::Bool(left >= right)
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

                Value::new_array(values)
            }
            Expression::Function(function) => {
                let mut function = function.clone();
                function.closure = Some(environment.clone());

                Value::new_function(function)
            }
            Expression::FunctionCall { callee, args, tail } => {
                let callee = callee.evaluate(environment)?;
                let Value::Function(function) = callee else {
                    return Err(EvaluationError::NotCallable(callee));
                };

                let mut call_args = function.default_args();
                for (param, arg) in args {
                    let arg = arg.evaluate(environment)?;
                    call_args
                        .set_arg(param, arg)
                        .map_err(EvaluationError::function)?;
                }
                for expr in tail {
                    let value = expr.evaluate(environment)?;
                    call_args
                        .add_tail(value)
                        .map_err(|error| EvaluationError::Function(Box::new(error)))?;
                }

                function
                    .call(call_args)
                    .map_err(EvaluationError::function)?
            }
            Expression::HashMap(key_value_pairs) => {
                let mut map = HashMap::new();
                for (key, value) in key_value_pairs {
                    let key = key.evaluate_or_null(environment);
                    let value = value.evaluate_or_null(environment);
                    map.insert(key, value);
                }

                Value::new_hash_map(map)
            }
        })
    }

    /// Like `Self::evaluate` but for simplicity, any illogical/illegal operation evaluates into a `Value::Null`.
    pub fn evaluate_or_null(&self, environment: &SharedEnvironment) -> Value {
        self.evaluate(environment).unwrap_or_default()
    }
}

impl From<Value> for Expression {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}
