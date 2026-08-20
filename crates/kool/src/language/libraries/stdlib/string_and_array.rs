use crate::{
    language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value},
    utils::CloneInner,
};

pub(super) fn string_and_array_functions() -> Library {
    Library::from([
        ("to_upper", to_upper_function()),
        ("to_lower", to_lower_function()),
        ("trim", trim_function()),
        ("len", len_function()),
        ("replace", replace_function()),
        ("split", split_function()),
    ])
}

fn to_upper_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let to_upper_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        let Value::String(string) = tail else {
            return Value::Null;
        };

        Value::new_string(string.to_uppercase())
    };

    Function {
        params,
        code: FunctionCode::new_host(to_upper_impl),
        closure: None,
    }
}

fn to_lower_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let to_lower_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        let Value::String(string) = tail else {
            return Value::Null;
        };

        Value::new_string(string.to_lowercase())
    };

    Function {
        params,
        code: FunctionCode::new_host(to_lower_impl),
        closure: None,
    }
}

fn trim_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let trim_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        let Value::String(string) = tail else {
            return Value::Null;
        };

        Value::new_string(string.trim().to_owned())
    };

    Function {
        params,
        code: FunctionCode::new_host(trim_impl),
        closure: None,
    }
}

/* Function applicable to both arrays and strings */

fn len_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let len_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        let len = match tail {
            Value::String(string) => string.len(),
            Value::Array(array) => array.read().expect("Lock poisoned").len(),
            Value::HashMap(map) => map.read().expect("Lock poisoned").len(),
            _ => return Value::Null,
        };

        Value::Int(len as i64)
    };

    Function {
        params,
        code: FunctionCode::new_host(len_impl),
        closure: None,
    }
}

fn replace_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const FROM_PARAM: &str = "from";
    const TO_PARAM: &str = "to";

    let params = FunctionParams::default()
        .with_param(FROM_PARAM, None)
        .with_param(TO_PARAM, None)
        .with_tail(TAIL_PARAM);
    let replace_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();
        let from = environment.get_variable(FROM_PARAM).unwrap_or_default();
        let to = environment.get_variable(TO_PARAM).unwrap_or_default();

        match (tail, from, to) {
            (Value::String(string), Value::String(from), Value::String(to)) => {
                Value::new_string(string.replace(from.as_str(), to.as_str()))
            }
            (Value::Array(array), from, to) => {
                let mut array = array.clone_inner();
                for value in &mut array {
                    if *value == from {
                        *value = to.clone();
                    }
                }

                Value::new_array(array)
            }
            (tail, _, _) => tail,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(replace_impl),
        closure: None,
    }
}

fn split_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const DELIM_PARAM: &str = "delim";

    let params = FunctionParams::default()
        .with_param(DELIM_PARAM, Some(Value::new_string(" ".to_owned())))
        .with_tail(TAIL_PARAM);
    let split_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();
        let delim = environment.get_variable(DELIM_PARAM).unwrap_or_default();

        match (tail, delim) {
            (Value::String(string), Value::String(delim)) => {
                let parts = string
                    .split(&*delim)
                    .map(|part| Value::new_string(part.to_owned()))
                    .collect();
                Value::new_array(parts)
            }
            (Value::Array(array), delim) => {
                let array = array.read().expect("Lock poisoned");
                let chunks = array
                    .split(|value| *value == delim)
                    .filter_map(|chunk| {
                        if chunk.is_empty() {
                            None
                        } else {
                            Some(Value::new_array(chunk.to_vec()))
                        }
                    })
                    .collect();

                Value::new_array(chunks)
            }
            (tail, _) => tail,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(split_impl),
        closure: None,
    }
}
