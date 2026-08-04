use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::language::{Environment, StatementBlock, StatementExecutionError, Value};

#[derive(Clone, Debug)]
pub struct Function {
    pub params: FunctionParams,
    pub code: FunctionCode,
}

impl Function {
    pub fn default_args(&self) -> FunctionCallArgs {
        let FunctionParams { params, tail_param } = self.params.clone();

        FunctionCallArgs {
            args: params,
            tail: tail_param.map(|tail| (tail, Vec::new())),
        }
    }

    pub fn call(
        &self,
        args: FunctionCallArgs,
        environment: &mut Environment,
    ) -> Result<Value, FunctionError> {
        let FunctionCallArgs { args, tail } = args;

        // Prepare functions environment
        environment.push_scope();

        for (arg, value) in args {
            environment.define_variable(arg, value);
        }

        // Only insert tail value if the tail param was defined
        if let Some((tail_param, mut tail)) = tail {
            let tail = if tail.len() <= 1 {
                tail.pop().unwrap_or_default()
            } else {
                Value::Array(tail)
            };

            environment.define_variable(tail_param, tail);
        }

        // Execute functions code in the newly created environment
        let return_value = self.code.execute(environment);

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

#[derive(Clone)]
pub enum FunctionCode {
    Block(StatementBlock),
    Host(Arc<dyn Fn(&mut Environment) -> Value + Send + Sync>),
}

impl Debug for FunctionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(block) => f.debug_tuple("Block").field(block).finish(),
            Self::Host(_) => write!(f, "host function"),
        }
    }
}

impl FunctionCode {
    pub fn new_host(function: impl Fn(&mut Environment) -> Value + Send + Sync + 'static) -> Self {
        Self::Host(Arc::new(function))
    }

    pub fn execute(&self, environment: &mut Environment) -> Result<Value, FunctionError> {
        match self {
            Self::Block(block) => block.execute(environment).map_err(FunctionError::Execution),
            Self::Host(function) => Ok(function(environment)),
        }
    }
}

/// Defined in function definition
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FunctionParams {
    /// Named parameters along with their default values.
    params: HashMap<String, Value>,
    /// Name of the optional _tail_ parameter.
    /// Its default value is always null.
    tail_param: Option<String>,
}

impl FunctionParams {
    /// Builder method for adding a parameter along with its default value.
    pub fn with_param(mut self, param: impl Into<String>, default: Option<Value>) -> Self {
        self.params
            .insert(param.into(), default.unwrap_or_default());
        self
    }

    /// Builder method for adding the tail parameter.
    pub fn with_tail(mut self, tail: impl Into<String>) -> Self {
        self.tail_param = Some(tail.into());
        self
    }
}

/// A set of evaluated arguments to be passed into a function.
/// Can only be obtained from a function definition (`Function`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCallArgs {
    /// Arguments passed in as param name and value pairs.
    args: HashMap<String, Value>,
    /// Rest of the arguments passed in without any params names.
    tail: Option<(String, Vec<Value>)>,
}

#[derive(Clone, Debug)]
pub enum FunctionError {
    Execution(StatementExecutionError),
    InvalidParameter(String),
    NoTailParameter,
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

    pub fn add_tail(&mut self, value: Value) -> Result<(), FunctionError> {
        let (_, tail) = self.tail.as_mut().ok_or(FunctionError::NoTailParameter)?;
        tail.push(value);

        Ok(())
    }
}
