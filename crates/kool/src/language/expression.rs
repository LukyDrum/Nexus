use crate::language::{Value, Variables};

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Power,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Illegal combination of operator and values: {0:?}.")]
    IllegalOperation(Expression),
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
                    _ => {
                        return Err(EvaluationError::IllegalOperation(Expression::Unary {
                            operator: *operator,
                            operand: operand.clone(),
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
                    (left, operator, right) => {
                        return Err(EvaluationError::IllegalOperation(Expression::Binary {
                            operator: *operator,
                            left: Box::new(Expression::Value(left)),
                            right: Box::new(Expression::Value(right)),
                        }));
                    }
                }
            }
        })
    }

    /// Like `Self::evaluate` but for simplicity, any illogical/illegal operation evaluates into a `Value::Null`.
    pub fn evaluate_or_null(&self, variables: &Variables) -> Value {
        self.evaluate(variables).unwrap_or_default()
    }
}
