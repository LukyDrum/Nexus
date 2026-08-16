use crate::language::{EvaluationError, Expression, Operator, SharedEnvironment, Value};

#[derive(Clone, Debug)]
pub enum Statement {
    VariableDeclaration {
        variable: String,
        right_side: Expression,
    },
    Assignment {
        target: Expression,
        right_side: Expression,
    },
    Expression {
        expression: Expression,
    },
}

#[derive(Clone, Debug)]
pub enum StatementExecutionError {
    UnknownVariable(String),
    VariableRedeclaration(String),
    FunctionRedefinition(String),
    ExpressionEvaluation(EvaluationError),
}

impl Statement {
    /// Normally a statement would not return a value, but since we want have implicit returns than this change comes in handy.
    pub fn execute(
        &self,
        environment: &SharedEnvironment,
    ) -> Result<Value, StatementExecutionError> {
        let value = match self {
            Self::VariableDeclaration {
                variable,
                right_side,
            } => {
                let value = right_side
                    .evaluate(environment)
                    .map_err(StatementExecutionError::ExpressionEvaluation)?;
                if environment
                    .define_variable(variable.clone(), value.clone())
                    .is_some()
                {
                    return Err(StatementExecutionError::VariableRedeclaration(
                        variable.clone(),
                    ));
                }

                value
            }
            Statement::Assignment { target, right_side } => {
                let right_side_value = right_side
                    .evaluate(environment)
                    .map_err(StatementExecutionError::ExpressionEvaluation)?;

                match target {
                    Expression::Variable(variable) => {
                        if environment
                            .set_variable(variable, right_side_value.clone())
                            .is_none()
                        {
                            return Err(StatementExecutionError::UnknownVariable(variable.clone()));
                        }

                        right_side_value
                    }
                    // Handle assignment into indexed values
                    Expression::Binary {
                        operator: Operator::Index,
                        left,
                        right,
                    } => {
                        let target_value = left
                            .evaluate(environment)
                            .map_err(StatementExecutionError::ExpressionEvaluation)?;
                        let index = right
                            .evaluate(environment)
                            .map_err(StatementExecutionError::ExpressionEvaluation)?;

                        match (target_value, index) {
                            (Value::Array(array), Value::Int(index)) => {
                                let Ok(index) = usize::try_from(index) else {
                                    return Err(StatementExecutionError::ExpressionEvaluation(
                                        EvaluationError::UnexpecteNonPositiveInteger(index),
                                    ));
                                };

                                let mut array_write = array.write().expect("Lock poisoned");
                                if let Some(target) = array_write.get_mut(index) {
                                    *target = right_side_value;

                                    target.clone()
                                } else {
                                    return Err(StatementExecutionError::ExpressionEvaluation(
                                        EvaluationError::IndexOutOfBounds {
                                            value: Value::Array(array.clone()),
                                            index,
                                        },
                                    ));
                                }
                            }
                            (Value::HashMap(map), key) => {
                                map.write()
                                    .expect("Lock poisoned")
                                    .insert(key, right_side_value.clone());

                                right_side_value
                            }

                            // The rest are non-indexable
                            _ => {
                                return Err(StatementExecutionError::ExpressionEvaluation(
                                    EvaluationError::IllegalOperation(target.clone()),
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(StatementExecutionError::ExpressionEvaluation(
                            EvaluationError::IllegalOperation(target.clone()),
                        ));
                    }
                }
            }
            Statement::Expression { expression } => expression
                .evaluate(environment)
                .map_err(StatementExecutionError::ExpressionEvaluation)?,
        };

        Ok(value)
    }
}

/// Used in `Statement` and `Function` as well.
#[derive(Clone, Debug, Default)]
pub struct StatementBlock {
    pub statements: Vec<Statement>,
}

impl StatementBlock {
    pub fn execute(
        &self,
        environment: &SharedEnvironment,
    ) -> Result<Value, StatementExecutionError> {
        let mut return_value = Value::Null;
        for statement in &self.statements {
            return_value = statement.execute(environment)?;
        }

        Ok(return_value)
    }
}
