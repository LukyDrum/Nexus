use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};

pub(super) fn math_functions() -> Library {
    Library::from([
        ("min", min_function()),
        ("max", max_function()),
        ("clamp", clamp_function()),
    ])
}

fn min_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let min_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        match tail {
            Value::Array(array) => {
                let array = array.read().expect("Lock poisoned");
                let Some(mut min_value) = array.first() else {
                    return Value::Null;
                };

                for value in &*array {
                    match (min_value, value) {
                        (Value::Int(min), Value::Int(cur)) if cur < min => min_value = value,
                        (Value::String(min), Value::String(cur)) if cur < min => min_value = value,
                        (Value::Bool(min), Value::Bool(cur)) if cur < min => min_value = value,
                        _ => {}
                    }
                }

                min_value.clone()
            }
            Value::HashMap(map) => {
                let map = map.read().expect("Lock poisoned");
                let mut maybe_min_value = None;

                for value in map.values() {
                    let min_value = maybe_min_value.get_or_insert(value);

                    match (min_value, value) {
                        (Value::Int(min), Value::Int(cur)) if cur < min => {
                            maybe_min_value = Some(value)
                        }
                        (Value::String(min), Value::String(cur)) if cur < min => {
                            maybe_min_value = Some(value)
                        }
                        (Value::Bool(min), Value::Bool(cur)) if cur < min => {
                            maybe_min_value = Some(value)
                        }
                        _ => {}
                    }
                }

                maybe_min_value.cloned().unwrap_or_default()
            }
            single @ (Value::Bool(_) | Value::Int(_) | Value::String(_)) => single,
            _ => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(min_impl),
        closure: None,
    }
}

fn max_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let max_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        match tail {
            Value::Array(array) => {
                let array = array.read().expect("Lock poisoned");
                let Some(mut min_value) = array.first() else {
                    return Value::Null;
                };

                for value in &*array {
                    match (min_value, value) {
                        (Value::Int(min), Value::Int(cur)) if cur > min => min_value = value,
                        (Value::String(min), Value::String(cur)) if cur > min => min_value = value,
                        (Value::Bool(min), Value::Bool(cur)) if cur > min => min_value = value,
                        _ => {}
                    }
                }

                min_value.clone()
            }
            Value::HashMap(map) => {
                let map = map.read().expect("Lock poisoned");
                let mut maybe_min_value = None;

                for value in map.values() {
                    let min_value = maybe_min_value.get_or_insert(value);

                    match (min_value, value) {
                        (Value::Int(min), Value::Int(cur)) if cur > min => {
                            maybe_min_value = Some(value)
                        }
                        (Value::String(min), Value::String(cur)) if cur > min => {
                            maybe_min_value = Some(value)
                        }
                        (Value::Bool(min), Value::Bool(cur)) if cur > min => {
                            maybe_min_value = Some(value)
                        }
                        _ => {}
                    }
                }

                maybe_min_value.cloned().unwrap_or_default()
            }
            single @ (Value::Bool(_) | Value::Int(_) | Value::String(_)) => single,
            _ => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(max_impl),
        closure: None,
    }
}

fn clamp_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const MIN_PARAM: &str = "min";
    const MAX_PARAM: &str = "max";

    let params = FunctionParams::default()
        .with_param(MIN_PARAM, Some(Value::Int(i64::MIN)))
        .with_param(MAX_PARAM, Some(Value::Int(i64::MAX)))
        .with_tail(TAIL_PARAM);
    let clamp_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();
        let min = environment.get_variable(MIN_PARAM).unwrap_or_default();
        let max = environment.get_variable(MAX_PARAM).unwrap_or_default();

        match (tail, min, max) {
            (Value::Int(value), Value::Int(min), Value::Int(max)) => {
                Value::Int(value.clamp(min, max))
            }
            _ => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(clamp_impl),
        closure: None,
    }
}
