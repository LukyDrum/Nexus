use crate::language::{Value, Variables};

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
    #[error("Illegal combination of operator and values: {0:?}.")]
    IllegalOperation(Expression),
    #[error("Index out of bounds, value is: {value}, but index is: {index}")]
    IndexOutOfBounds { value: Value, index: usize },
    #[error("Expected a positive integer, found: {0}.")]
    UnexpecteNonPositiveInteger(i64),
    #[error("Unknown variable: {0}.")]
    UnknownVariable(String),
}

impl Expression {
    /// Evaluates the expression to a concrete `Value`.
    pub fn evaluate(&self, variables: &Variables) -> Result<Value, EvaluationError> {
        Ok(match self {
            Expression::Value(value) => value.clone(),
            Expression::Variable(name) => variables.get(name).cloned().unwrap_or_default(),
            Expression::Unary { operator, operand } => {
                let value = operand.evaluate(variables)?;
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
                let left = left.evaluate(variables)?;
                let right = right.evaluate(variables)?;

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
                let mut values = Vec::new();

                for expression in array {
                    values.push(expression.evaluate_or_null(variables));
                }

                Value::Array(values)
            }
        })
    }

    /// Like `Self::evaluate` but for simplicity, any illogical/illegal operation evaluates into a `Value::Null`.
    pub fn evaluate_or_null(&self, variables: &Variables) -> Value {
        self.evaluate(variables).unwrap_or_default()
    }
}
