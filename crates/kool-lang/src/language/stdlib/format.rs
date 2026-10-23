use std::fmt::Display;

use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};

pub(super) fn format_library() -> Library {
    Library::from([("format", format_function())])
}

/// Supports:
///     - Variable interpolation from current environment - {varname} and positional
///     - Number formatting
///         - pre-padding
///         - binary
///         - hex
fn format_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let format_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        let formatted = match tail {
            Value::String(string) => format_string_with_args(string.as_str(), &[], environment),
            Value::Array(array) => {
                let array = array.read().expect("Lock poisoned");
                let Some(Value::String(string)) = array.first() else {
                    return Value::Null;
                };
                let Some(args) = array.get(1..) else {
                    return Value::Null;
                };

                format_string_with_args(string.as_str(), args, environment)
            }
            _ => return Value::Null,
        };

        Value::new_string(formatted)
    };

    Function {
        params,
        code: FunctionCode::new_host(format_impl),
        closure: None,
    }
}

#[derive(Debug)]
enum FormatFragment<'a> {
    Raw(&'a str),
    Value { value: Value, format: ValueFormat },
}

#[derive(Debug)]
enum ValueFormat {
    None,
    Invalid,

    /// Pads with `N` 0's in the front.
    ZeroPadding(usize),
    Binary,
    Hex,
    HexCapital,
}

impl<'a> Display for FormatFragment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatFragment::Raw(raw) => write!(f, "{raw}"),
            FormatFragment::Value { value, format } => match (value, format) {
                (value, ValueFormat::None) => write!(f, "{value}"),
                (Value::Int(int), ValueFormat::Binary) => write!(f, "{int:b}"),
                (Value::Int(int), ValueFormat::Hex) => write!(f, "{int:x}"),
                (Value::Int(int), ValueFormat::HexCapital) => write!(f, "{int:X}"),
                (Value::Int(int), ValueFormat::ZeroPadding(zeroes)) => write!(f, "{int:0zeroes$}"),
                _ => write!(f, "invalid"),
            },
        }
    }
}

fn format_string_with_args(
    format: &str,
    args: &[Value],
    environment: &SharedEnvironment,
) -> String {
    const START: char = '{';
    const END: char = '}';
    const FORMAT_DELIM: char = ':';

    let mut args_start = 0;

    let mut fragments = Vec::new();
    let mut start = 0;
    let mut escape_next = false;
    for (end, c) in format.char_indices() {
        if escape_next {
            escape_next = true;
            continue;
        }

        match c {
            START if end != start => {
                fragments.push(FormatFragment::Raw(&format[start..end]));
                start = end + 1;
            }
            END => {
                let section = &format[start..end];
                let (var_name, value_format) =
                    section.split_once(FORMAT_DELIM).unwrap_or((section, ""));
                let var_name = var_name.trim_start_matches(START);

                let maybe_value = if var_name.is_empty() {
                    let maybe_value = args.get(args_start);
                    args_start += 1;
                    maybe_value.cloned()
                } else {
                    environment.get_variable(var_name)
                };
                let value = maybe_value.unwrap_or_default();

                let value_format = match value_format.chars().next() {
                    Some('0') => match value_format.get(1..) {
                        Some("b") => ValueFormat::Binary,
                        Some("x") => ValueFormat::Hex,
                        Some("X") => ValueFormat::HexCapital,
                        Some(num) => num
                            .parse()
                            .ok()
                            .map(ValueFormat::ZeroPadding)
                            .unwrap_or(ValueFormat::Invalid),
                        None => ValueFormat::Invalid,
                    },
                    Some(_) => ValueFormat::Invalid,
                    None => ValueFormat::None,
                };

                fragments.push(FormatFragment::Value {
                    value,
                    format: value_format,
                });
                start = end + 1;
            }
            _ => {}
        }
    }
    fragments.push(FormatFragment::Raw(&format[start..]));

    fragments
        .into_iter()
        .map(|fragment| format!("{fragment}"))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::language::{
        Environment, SharedEnvironment, Value, libraries::stdlib::format::format_string_with_args,
    };

    #[test]
    fn fill_in_from_environment() {
        let mut environment = Environment::default();
        environment.define_variable(
            "question".to_owned(),
            Value::new_string("life, universe and everything".to_owned()),
        );
        environment.define_variable("answer".to_owned(), Value::Int(42));
        let environment = environment.into_shared();

        let formatted = format_string_with_args(
            "The answer to the question of {question} is {answer} (aka {answer:0b} or {answer:0X})",
            &[],
            &environment,
        );

        assert_eq!(
            formatted,
            "The answer to the question of life, universe and everything is 42 (aka 101010 or 2A)",
        );
    }

    #[test]
    fn padding() {
        let formatted = format_string_with_args(
            "{:02}:{:02}",
            &[Value::Int(7), Value::Int(7)],
            &SharedEnvironment::default(),
        );

        assert_eq!(formatted, "07:07",);
    }
}
