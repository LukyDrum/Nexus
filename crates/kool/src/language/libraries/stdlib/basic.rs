use chrono::{Datelike, Timelike};

use crate::{
    language::{
        Expression, Function, FunctionCode, FunctionParams, Library, Operator, SharedEnvironment,
        Value,
    },
    utils::CloneInner,
};

pub(super) fn basic_functions() -> Library {
    Library::from([
        ("print", print_function()),
        ("sum", sum_function()),
        ("exec", exec_function()),
        ("date_str", date_str_function()),
        ("date", date_function()),
    ])
}

fn print_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let print_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        println!("{tail}");

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(print_impl),
        closure: None,
    }
}

fn sum_function() -> Function {
    const SEP_PARAM: &str = "sep";
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default()
        .with_param(SEP_PARAM, None)
        .with_tail(TAIL_PARAM);

    let sum_impl = |environment: &SharedEnvironment| -> Value {
        let sep = environment.get_variable(SEP_PARAM).unwrap_or(Value::Null);
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or(Value::Null);

        let array = match tail {
            Value::Array(array) => array,
            single => return single,
        };

        // The only time reduce returns `None` is when the array was empty, then it makes sense to return an empty array as well.
        array
            .clone_inner()
            .into_iter()
            .reduce(|sum, value| {
                // If there is a separator than add the sum and the sep first
                let sum = if sep.is_null() {
                    sum
                } else {
                    let expr = Expression::Binary {
                        operator: Operator::Add,
                        left: Box::new(Expression::Value(sum)),
                        right: Box::new(Expression::Value(sep.clone())),
                    };

                    expr.evaluate_or_null(environment)
                };

                let expr = Expression::Binary {
                    operator: Operator::Add,
                    left: Box::new(Expression::Value(sum)),
                    right: Box::new(Expression::Value(value)),
                };
                expr.evaluate_or_null(environment)
            })
            .unwrap_or(Value::new_array(Vec::new()))
    };

    Function {
        params,
        code: FunctionCode::new_host(sum_impl),
        closure: None,
    }
}

fn exec_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let exec_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        let mut parts = match tail {
            Value::String(command) => split_command(&command),
            Value::Array(array) => array
                .read()
                .expect("Lock poisoned")
                .iter()
                .map(|value| value.to_string())
                .collect(),
            _ => return Value::Null,
        };

        if parts.is_empty() {
            return Value::Null;
        }

        let program = parts.remove(0);
        let Ok(output) = std::process::Command::new(program).args(parts).output() else {
            return Value::Null;
        };
        let output = String::from_utf8_lossy(&output.stdout).to_string();

        Value::new_string(output)
    };

    Function {
        params,
        code: FunctionCode::new_host(exec_impl),
        closure: None,
    }
}

fn split_command(command: &str) -> Vec<String> {
    let mut start = 0;
    let mut end = 0;
    let mut quoted = false;
    let mut parts = Vec::new();

    for c in command.chars() {
        if c == ' ' && !quoted {
            parts.push(command[start..end].trim_matches(is_quote).to_owned());
            start = end + 1;
        } else if is_quote(c) {
            quoted = !quoted;
        }

        end += 1;
    }
    parts.push(command[start..end].trim_matches(is_quote).to_owned());

    parts
}

fn is_quote(c: char) -> bool {
    c == '"' || c == '\''
}

fn date_str_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let date_str_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        let now = chrono::Local::now();
        let formatted = if let Value::String(format) = tail {
            now.format(&format).to_string()
        } else {
            now.to_rfc3339()
        };

        Value::new_string(formatted)
    };

    Function {
        params,
        code: FunctionCode::new_host(date_str_impl),
        closure: None,
    }
}

fn date_function() -> Function {
    let params = FunctionParams::default();
    let date_impl = |_: &SharedEnvironment| -> Value {
        let now = chrono::Local::now();
        let date_obj_values = [
            ("year", Value::Int(now.year() as i64)),
            ("month", Value::Int(now.month() as i64)),
            ("day", Value::Int(now.day() as i64)),
            ("hour", Value::Int(now.hour() as i64)),
            ("minute", Value::Int(now.minute() as i64)),
            ("second", Value::Int(now.second() as i64)),
            ("weekday", Value::new_string(now.weekday().to_string())),
        ];

        Value::new_hash_map(
            date_obj_values
                .into_iter()
                .map(|(key, value)| (Value::new_string(key.to_owned()), value))
                .collect(),
        )
    };

    Function {
        params,
        code: FunctionCode::new_host(date_impl),
        closure: None,
    }
}
