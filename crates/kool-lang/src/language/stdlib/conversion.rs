use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};

pub(super) fn conversion_functions() -> Library {
    Library::from([
        ("bool", bool_function()),
        ("int", int_function()),
        ("string", string_function()),
    ])
}

fn bool_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let bool_impl = |environment: &SharedEnvironment| -> Value {
        let value = environment.get_variable(TAIL_PARAM).unwrap_or_default();
        Value::Bool(value.is_truthy())
    };

    Function {
        params,
        code: FunctionCode::new_host(bool_impl),
        closure: None,
    }
}

fn int_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let int_impl = |environment: &SharedEnvironment| -> Value {
        let value = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        match value {
            Value::Int(int) => Value::Int(int),
            Value::Bool(false) => Value::Int(0),
            Value::Bool(true) => Value::Int(1),
            Value::String(string) => string.parse::<i64>().map(Value::Int).unwrap_or_default(),
            _ => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(int_impl),
        closure: None,
    }
}

fn string_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let string_impl = |environment: &SharedEnvironment| -> Value {
        let value = environment.get_variable(TAIL_PARAM).unwrap_or_default();
        Value::new_string(value.to_string())
    };

    Function {
        params,
        code: FunctionCode::new_host(string_impl),
        closure: None,
    }
}
