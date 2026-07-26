use crate::{BuildContext, element::ErrorElement, elemental::ElementalMessage, language::Value};

#[macro_export]
macro_rules! eval_or_return_error {
    ($expr:expr, $variables:expr, $pat:pat => $value:ident) => {{
        let value = $expr.evaluate_or_null($variables);
        match value {
            $pat => $value,
            other => {
                return Err(ErrorElement::new(format!(
                    "[{}] invalid value: {other}",
                    Self::type_name()
                )));
            }
        }
    }};
}

pub(crate) fn into_child_elements<'a>(
    array: Vec<Value>,
    context: &'a BuildContext,
) -> impl Iterator<Item = iced::Element<'a, ElementalMessage>> {
    array.into_iter().map(|child| match child {
        Value::Element(elem) => elem.build(context),
        other => {
            let error = ErrorElement::new(format!("invalid value: {other}"));
            error.build(context).into()
        }
    })
}
