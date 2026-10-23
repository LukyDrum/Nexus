use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};
use crate::TryFromValue;

pub(super) fn collections_functions() -> Library {
    Library::from([
        ("map", map_function()),
        ("filter", filter_function()),
        ("repeat", repeat_function()),
        ("enumerateMap", enumerate_map_function()),
        ("insert", insert_function()),
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

// Expects a function and an array. The function should take a named `index` and named `value` parameter.
fn enumerate_map_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const FUNC_PARAM: &str = "func";

    const INDEX_PARAM: &str = "index";
    const VALUE_PARAM: &str = "value";

    let params = FunctionParams::default()
        .with_param(FUNC_PARAM, None)
        .with_tail(TAIL_PARAM);
    let enumerate_map_impl = |environment: &SharedEnvironment| -> Value {
        let Some(Value::Function(func)) = environment.get_variable(FUNC_PARAM) else {
            return Value::Null;
        };
        let Some(Value::Array(array)) = environment.get_variable(TAIL_PARAM) else {
            return Value::Null;
        };
        let read = array.read().expect("Lock poisoned");

        let enumerated_mapped = read
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let mut args = func.default_args();
                let _ = args.set_arg(INDEX_PARAM, Value::Int(index as i64));
                let _ = args.set_arg(VALUE_PARAM, value.clone());

                func.call(args).unwrap_or_default()
            })
            .collect();

        Value::new_array(enumerated_mapped)
    };

    Function {
        params,
        code: FunctionCode::new_host(enumerate_map_impl),
        closure: None,
    }
}

/// Works for inserting values into an array or a hashmap.
fn insert_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const VALUE_PARAM: &str = "value";
    const KEY_PARAM: &str = "key";

    let params = FunctionParams::default()
        .with_param(VALUE_PARAM, None)
        .with_param(KEY_PARAM, None)
        .with_tail(TAIL_PARAM);
    let insert_impl = |environment: &SharedEnvironment| -> Value {
        let value = environment.get_variable(VALUE_PARAM).unwrap_or_default();
        match environment.get_variable(TAIL_PARAM).unwrap_or_default() {
            Value::Array(array) => {
                let mut array = array.write().expect("Lock poisoned");
                let maybe_index = environment
                    .get_variable(KEY_PARAM)
                    .and_then(usize::try_from_value);

                if let Some(index) = maybe_index {
                    array.insert(index, value);
                } else {
                    array.push(value);
                }

                Value::Null
            }
            Value::HashMap(map) => {
                let mut map = map.write().expect("Lock poisoned");
                let maybe_key = environment
                    .get_variable(KEY_PARAM)
                    .and_then(String::try_from_value);

                if let Some(key) = maybe_key {
                    map.insert(key, value).unwrap_or_default()
                } else {
                    Value::Null
                }
            }
            _ => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(insert_impl),
        closure: None,
    }
}
