use std::collections::HashMap;

use crate::language::{Environment, StatementBlock, StatementExecutionError, Value};

const TAIL_VAR_NAME: &str = "tail";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub params: Vec<FunctionParam>,
    pub code: StatementBlock,
}

impl Function {
    pub fn default_args(&self) -> FunctionCallArgs {
        let args = self
            .params
            .iter()
            .map(|FunctionParam { name, default }| {
                (name.to_owned(), default.clone().unwrap_or_default())
            })
            .collect();

        FunctionCallArgs {
            args,
            tail: Vec::new(),
        }
    }

    pub fn call(
        &self,
        args: FunctionCallArgs,
        environment: &mut Environment,
    ) -> Result<Value, FunctionError> {
        let FunctionCallArgs { args, mut tail } = args;

        // Prepare functions environment
        environment.push_scope();

        for (arg, value) in args {
            environment.define_variable(arg, value);
        }

        let tail = if tail.len() == 1 {
            tail.remove(0)
        } else {
            Value::Array(tail)
        };
        environment.define_variable(TAIL_VAR_NAME.to_owned(), tail);

        // Execute functions code in the newly created environment
        let return_value = self
            .code
            .execute(environment)
            .map_err(FunctionError::Execution);

        environment.pop_scope();

        return_value
    }

    pub fn call_with_default_args(
        &self,
        environment: &mut Environment,
    ) -> Result<Value, FunctionError> {
        self.call(self.default_args(), environment)
    }
}

/// Defined in function definition
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionParam {
    pub name: String,
    pub default: Option<Value>,
}

/// A set of evaluated arguments to be passed into a function.
/// Can only be obtained from a function definition (`Function`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCallArgs {
    /// Argmuments passed in as param name and value pairs.
    args: HashMap<String, Value>,
    /// Rest of the arguments passed in without any params names.
    tail: Vec<Value>,
}

#[derive(Clone, Debug)]
pub enum FunctionError {
    InvalidParameter(String),
    Execution(StatementExecutionError),
}

impl FunctionCallArgs {
    pub fn set_arg(&mut self, name: &str, value: Value) -> Result<(), FunctionError> {
        if let Some(arg) = self.args.get_mut(name) {
            *arg = value;
            Ok(())
        } else {
            Err(FunctionError::InvalidParameter(name.to_owned()))
        }
    }

    pub fn add_tail(&mut self, value: Value) {
        self.tail.push(value);
    }
}
