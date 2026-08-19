use std::{
    collections::HashMap,
    fmt::{Display, Write},
    rc::Rc,
};

use crate::{
    language::{FunctionParams, SharedEnvironment, Value},
    style::{CommonStyle, StyleKey, StyleTree, WithStyle, WithStyleKey, WithStyleOverride},
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

pub(super) fn build_with_style<Widget>(
    widget: Widget,
    tree: Rc<StyleTree>,
    key: StyleKey,
    style_override: Option<CommonStyle>,
) -> Widget
where
    Widget: WithStyle + WithStyleKey + WithStyleOverride,
{
    let mut final_tree = tree;

    if !key.is_empty() {
        let id_tree = StyleTree::subtree(&final_tree, key);
        let base_tree = Widget::base_tree(&final_tree);

        let mut adhoc_tree = StyleTree::default();
        adhoc_tree.set_style(base_tree.style().inherit(&final_tree.style()));
        adhoc_tree.nest(Widget::BASE_KEY, id_tree);

        final_tree = Rc::new(adhoc_tree);
    }

    if let Some(override_style) = style_override {
        let base_tree = Widget::base_tree(&final_tree);
        let merged_style = override_style.inherit(&base_tree.style());

        let mut adhoc_tree = final_tree.clone_inner();

        let mut overridden_base_tree = base_tree.clone_inner();
        overridden_base_tree.set_style(merged_style);

        adhoc_tree.nest(Widget::BASE_KEY, Rc::new(overridden_base_tree));

        final_tree = Rc::new(adhoc_tree);
    }

    widget.with_style(final_tree)
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
