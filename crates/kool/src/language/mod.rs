mod environment;
mod expression;
mod function;
mod statement;
mod stdlib;
mod value;

pub use environment::Environment;
pub use expression::{EvaluationError, Expression, Operator};
pub use function::{Function, FunctionCode, FunctionError, FunctionParam, TAIL_VAR};
pub use statement::{Statement, StatementBlock, StatementExecutionError};
pub use stdlib::standard_environment;
pub use value::Value;
