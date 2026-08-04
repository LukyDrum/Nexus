use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{KEY_VAR, key_function_param},
        kool::Error,
    },
    get_var_or_elem_error,
    language::{Environment, Function, FunctionCode, TAIL_VAR, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Text {
    pub key: String,
    pub content: String,
}

impl<'a> Element<'a> for Text {
    type IcedElement = iced::widget::Text<'a>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Text::new(self.content.clone());

        if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        }
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Text";

        let params = vec![key_function_param()];

        let function = |environment: &mut Environment| -> Value {
            let key = get_var_or_elem_error!(KEY_VAR, environment, Value::String(key) => key);
            let content = get_var_or_elem_error!(TAIL_VAR, environment, value => value);

            Value::Element(Box::new(KoolElement::Text(Self {
                key,
                content: content.to_string(),
            })))
        };

        (
            NAME,
            Function {
                params,
                code: FunctionCode::new_host(function),
            },
        )
    }
}
