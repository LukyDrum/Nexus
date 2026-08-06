use crate::language::{Environment, EvaluationError, Expression, Value};

#[derive(Clone, Debug)]
pub enum Statement {
    VariableDeclaration {
        variable: String,
        right_side: Expression,
    },
    VariableAssignment {
        variable: String,
        right_side: Expression,
    },
    Expression {
        expression: Expression,
    },
    Block {
        block: StatementBlock,
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
    pub fn execute(&self, environment: &mut Environment) -> Result<Value, StatementExecutionError> {
        let value = match self {
            Self::VariableDeclaration {
                variable,
                right_side,
            } => {
                let value = right_side
                    .evaluate(environment)
                    .map_err(StatementExecutionError::ExpressionEvaluation)?;
                if environment
                    .define_variable(variable.clone(), value)
                    .is_some()
                {
                    return Err(StatementExecutionError::VariableRedeclaration(
                        variable.clone(),
                    ));
                }

                Value::Null
            }
            Statement::VariableAssignment {
                variable,
                right_side,
            } => {
                let value = right_side
                    .evaluate(environment)
                    .map_err(StatementExecutionError::ExpressionEvaluation)?;
                if environment.set_variable(variable, value).is_none() {
                    return Err(StatementExecutionError::UnknownVariable(variable.clone()));
                }

                Value::Null
            }
            Statement::Expression { expression } => expression
                .evaluate(environment)
                .map_err(StatementExecutionError::ExpressionEvaluation)?,
            Statement::Block { block } => block.execute(environment)?,
        };

        Ok(value)
    }
}

/// Used in `Statement` and `Function` as well.
#[derive(Clone, Debug)]
pub struct StatementBlock {
    pub statements: Vec<Statement>,
}

impl StatementBlock {
    pub fn execute(&self, environment: &mut Environment) -> Result<Value, StatementExecutionError> {
        let mut return_value = Value::Null;
        for statement in &self.statements {
            return_value = statement.execute(environment)?;
        }

        Ok(return_value)
    }
}
