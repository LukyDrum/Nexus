use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use crate::{
    language::{FunctionParams, SharedEnvironment, Value},
    style::CommonStyle,
    utils::CloneInner,
};

pub(super) const KEY_PARAM: &str = "key";
pub(super) const STYLE_PARAM: &str = "style";
/// Name of the tail parameter in basic element function params.
pub(super) const CONTENT_PARAM: &str = "content";

/// Returns basic `FunctionParams` with a `key` parameter and a tail parameter named `content`.
pub(super) fn element_base_params() -> FunctionParams {
    FunctionParams::default()
        .with_param(KEY_PARAM.to_owned(), Some(Value::new_string(String::new())))
        .with_param(STYLE_PARAM.to_owned(), None)
        .with_tail(CONTENT_PARAM)
}

#[macro_export]
macro_rules! get_var_or_elem_error {
    ($var:expr, $env:expr, $pat:pat => $value:ident) => {{
        use $crate::element::kool::Error;

        let Some(var) = $env.get_variable($var) else {
            return Value::new_element(KoolElement::Error(Error::new(format!(
                "`{}` variable not found",
                $var
            ))));
        };
        #[allow(irrefutable_let_patterns)]
        let $pat = var else {
            return Value::new_element(KoolElement::Error(Error::new(format!(
                "`{}` is of unexpected type",
                $var
            ))));
        };

        $value.to_owned()
    }};
}

pub(super) fn merge_with_default_style(
    style: Option<CommonStyle>,
    default: CommonStyle,
) -> CommonStyle {
    style.map_or(default, |style| style.merge(&default))
}

pub(super) fn get_style_from_environment(environment: &SharedEnvironment) -> Option<CommonStyle> {
    let value = environment.get_variable(STYLE_PARAM)?;

    kool_value_to_toml(&value)?.try_into().ok()
}

#[expect(clippy::mutable_key_type)]
fn kool_map_to_table(map: &HashMap<Value, Value>) -> Option<toml::Table> {
    map.iter()
        .map(|(key, value)| {
            let Value::String(key) = key else {
                return None;
            };

            kool_value_to_toml(value)
                .map(|value| (CamelToSnakeCase(key.as_str()).to_string(), value))
        })
        .collect()
}

fn kool_value_to_toml(value: &Value) -> Option<toml::Value> {
    let value = match value {
        Value::Bool(bool) => toml::Value::Boolean(*bool),
        Value::Int(int) => toml::Value::Integer(*int),
        Value::String(string) => toml::Value::String(string.clone_inner()),
        Value::Array(array) => {
            let array = array.read().expect("Lock poisoned");
            let array = array
                .iter()
                .map(kool_value_to_toml)
                .collect::<Option<Vec<_>>>()?;
            toml::Value::Array(array)
        }
        Value::HashMap(map) => {
            let map = map.read().expect("Lock poisoned");
            toml::Value::Table(kool_map_to_table(&map)?)
        }
        _ => return None,
    };

    Some(value)
}

struct CamelToSnakeCase<T>(T)
where
    T: AsRef<str>;

impl<T> Display for CamelToSnakeCase<T>
where
    T: AsRef<str>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let camel = self.0.as_ref();
        for c in camel.chars() {
            if c.is_uppercase() {
                f.write_char('_')?;
                f.write_char(c.to_ascii_lowercase())?;
                continue;
            }

            f.write_char(c)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::element::common::CamelToSnakeCase;

    #[test]
    fn camel_to_snake() {
        let camel = "alignX";
        let snake = CamelToSnakeCase(camel).to_string();
        assert_eq!(snake, "align_x")
    }
}
