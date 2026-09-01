use crate::{
    language::{FunctionParams, SharedEnvironment, Value},
    style::CommonStyle,
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

    Some(CommonStyle::from(value))
}
