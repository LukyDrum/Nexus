#![expect(
    clippy::mutable_key_type,
    reason = "we only hash values without interior mutability"
)]

mod environment;
mod expression;
mod function;
mod importer;
pub mod libraries;
mod library;
mod statement;
mod value;

pub use environment::{Environment, SharedEnvironment};
pub use expression::{EvaluationError, Expression, Operator};
pub use function::{Function, FunctionCode, FunctionError, FunctionParams};
pub use importer::{LibraryLoadError, load_library};
pub use library::Library;
pub use statement::{Statement, StatementBlock, StatementExecutionError};
pub use value::Value;
