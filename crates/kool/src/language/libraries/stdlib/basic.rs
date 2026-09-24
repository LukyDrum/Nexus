use std::time::Duration;

use chrono::{Datelike, Timelike};

use crate::{
    language::{
        DuplexChannel, Expression, Function, FunctionCode, FunctionParams, Library, Operator,
        SharedEnvironment, Value,
    },
    utils::CloneInner,
};

const MILLIS_IN_SECONDS: u32 = 1000;
const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;

pub(super) fn basic_functions() -> Library {
    Library::from([
        ("print", print_function()),
        ("clone", clone_function()),
        ("sum", sum_function()),
        ("exec", exec_function()),
        ("date_str", date_str_function()),
        ("date", date_function()),
        ("timeEvents", time_events_function()),
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

fn clone_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let clone_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        match tail {
            Value::String(string) => Value::new_string(string.clone_inner()),
            Value::Element(kool_element) => Value::new_element(kool_element.clone_inner()),
            Value::Array(array) => Value::new_array(array.read().expect("Lock poisoned").clone()),
            Value::HashMap(map) => Value::new_hash_map(map.read().expect("Lock poisoned").clone()),
            // Other values can either be easily copied, or it makes no sense to clone them
            value => value,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(clone_impl),
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
    let date_impl = |_: &SharedEnvironment| -> Value { current_time_value() };

    Function {
        params,
        code: FunctionCode::new_host(date_impl),
        closure: None,
    }
}

/// Produces a time event on every whole second/minute/hour base on `onEvery` param.
fn time_events_function() -> Function {
    const ON_EVERY_PARAM: &str = "onEvery";

    let params =
        FunctionParams::default().with_param(ON_EVERY_PARAM, Some(Value::new_string("second")));
    let time_events = |environment: &SharedEnvironment| -> Value {
        let Some(Value::String(on_every)) = environment.get_variable(ON_EVERY_PARAM) else {
            return Value::Null;
        };
        // Pre-check
        match on_every.as_str() {
            "second" | "minute" | "hour" => {}
            _ => return Value::Null,
        }

        let channel = DuplexChannel::default();
        let channel_value = Value::Channel(channel.clone());

        std::thread::spawn(move || {
            // We want millisecond precision here
            loop {
                let now = chrono::Local::now();

                let millis_to_sleep = match on_every.as_str() {
                    "second" => {
                        let millis = now.timestamp_subsec_millis();
                        MILLIS_IN_SECONDS.saturating_sub(millis)
                    }
                    "minute" => {
                        let millis =
                            now.second() * MILLIS_IN_SECONDS + now.timestamp_subsec_millis();
                        (SECONDS_IN_MINUTE * MILLIS_IN_SECONDS).saturating_sub(millis)
                    }
                    "hour" => {
                        let millis = now.minute() * SECONDS_IN_MINUTE * MILLIS_IN_SECONDS
                            + now.second() * MILLIS_IN_SECONDS
                            + now.timestamp_subsec_millis();
                        (MINUTES_IN_HOUR * SECONDS_IN_MINUTE * MILLIS_IN_SECONDS)
                            .saturating_sub(millis)
                    }
                    _ => unreachable!("We checked for these variants beforehand."),
                };

                std::thread::sleep(Duration::from_millis(millis_to_sleep.into()));

                let _ = channel.send(current_time_value());
            }
        });

        channel_value
    };

    Function {
        params,
        code: FunctionCode::new_host(time_events),
        closure: None,
    }
}

fn current_time_value() -> Value {
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
}
