use crate::language::{FunctionParam, Value};

pub(super) const KEY_VAR: &str = "key";

pub(super) fn key_function_param() -> FunctionParam {
    FunctionParam {
        name: "key".to_owned(),
        default: Some(Value::String("".to_owned())),
    }
}

#[macro_export]
macro_rules! get_var_or_elem_error {
    ($var:expr, $env:expr, $pat:pat => $value:ident) => {{
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
