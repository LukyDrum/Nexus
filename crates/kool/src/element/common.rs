use crate::language::{FunctionParams, Value};

pub(super) const KEY_PARAM: &str = "key";
/// Name of the tail parameter in basic element function params.
pub(super) const CONTENT_PARAM: &str = "content";

/// Returns basic `FunctionParams` with a `key` parameter and a tail parameter named `content`.
pub(super) fn element_base_params() -> FunctionParams {
    FunctionParams::default()
        .with_param(KEY_PARAM.to_owned(), Some(Value::String(String::new())))
        .with_tail(CONTENT_PARAM)
}

#[macro_export]
macro_rules! get_var_or_elem_error {
    ($var:expr, $env:expr, $pat:pat => $value:ident) => {{
        use $crate::element::kool::Error;

        let Some(var) = $env.get_variable($var) else {
            return Value::Element(Box::new(KoolElement::Error(Error::new(format!(
                "`{}` variable not found",
                $var
            )))));
        };
        #[allow(irrefutable_let_patterns)]
        let $pat = var else {
            return Value::Element(Box::new(KoolElement::Error(Error::new(format!(
                "`{}` is of unexpected type",
                $var
            )))));
        };

        $value.to_owned()
    }};
}
