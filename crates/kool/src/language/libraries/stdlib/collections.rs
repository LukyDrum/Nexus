use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};

pub(super) fn collections_functions() -> Library {
    Library::from([
        ("map", map_function()),
        ("filter", filter_function()),
        ("repeat", repeat_function()),
    ])
}

/// Func is expected to have a tail parameter.
fn map_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const FUNC_PARAM: &str = "func";

    let params = FunctionParams::default()
        .with_param(FUNC_PARAM, None)
        .with_tail(TAIL_PARAM);
    let map_impl = |environment: &SharedEnvironment| -> Value {
        let Some(Value::Function(func)) = environment.get_variable(FUNC_PARAM) else {
            return Value::Null;
        };
        let collection = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        match collection {
            Value::Array(array) => {
                let array = array.read().expect("Lock poisoned");

                let mapped_array = array
                    .iter()
                    .map(|value| {
                        let mut call_args = func.default_args();
                        let _ = call_args.set_singular_arg(value.clone());

                        func.call(call_args).unwrap_or_default()
                    })
                    .collect();

                Value::new_array(mapped_array)
            }
            Value::HashMap(map) => {
                let map = map.read().expect("Lock poisoned");

                let mapped_map = map
                    .iter()
                    .map(|(key, value)| {
                        let mut call_args = func.default_args();
                        let _ = call_args.set_singular_arg(value.clone());

                        (key.clone(), func.call(call_args).unwrap_or_default())
                    })
                    .collect();

                Value::new_hash_map(mapped_map)
            }
            _ => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(map_impl),
        closure: None,
    }
}

/// Func is expected to have a tail parameter.
fn filter_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const FUNC_PARAM: &str = "func";

    let params = FunctionParams::default()
        .with_param(FUNC_PARAM, None)
        .with_tail(TAIL_PARAM);
    let filter_impl = |environment: &SharedEnvironment| -> Value {
        let Some(Value::Function(func)) = environment.get_variable(FUNC_PARAM) else {
            return Value::Null;
        };
        let collection = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        match collection {
            Value::Array(array) => {
                let array = array.read().expect("Lock poisoned");

                let filtered_array = array
                    .iter()
                    .filter(|value| {
                        let mut call_args = func.default_args();
                        let _ = call_args.set_singular_arg((*value).clone());

                        func.call(call_args).unwrap_or_default().is_truthy()
                    })
                    .cloned()
                    .collect();

                Value::new_array(filtered_array)
            }
            Value::HashMap(map) => {
                let map = map.read().expect("Lock poisoned");

                let filtered_map = map
                    .iter()
                    .filter(|(_, value)| {
                        let mut call_args = func.default_args();
                        let _ = call_args.set_singular_arg((*value).clone());

                        func.call(call_args).unwrap_or_default().is_truthy()
                    })
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect();

                Value::new_hash_map(filtered_map)
            }
            _ => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(filter_impl),
        closure: None,
    }
}

fn repeat_function() -> Function {
    const VALUE_PARAM: &str = "value";
    const TIMES_PARAM: &str = "times";

    let params = FunctionParams::default()
        .with_param(VALUE_PARAM, None)
        .with_param(TIMES_PARAM, Some(Value::Int(1)));
    let repeat_impl = |environment: &SharedEnvironment| -> Value {
        let value = environment.get_variable(VALUE_PARAM).unwrap_or_default();
        let Some(Value::Int(times)) = environment.get_variable(TIMES_PARAM) else {
            return Value::Null;
        };

        Value::new_array(vec![value; times.max(0) as usize])
    };

    Function {
        params,
        code: FunctionCode::new_host(repeat_impl),
        closure: None,
    }
}
