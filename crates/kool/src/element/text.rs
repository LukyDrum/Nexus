use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{CONTENT_PARAM, KEY_PARAM, element_base_params},
    },
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
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

        let function = |environment: &SharedEnvironment| -> Value {
            let key = get_var_or_elem_error!(KEY_PARAM, environment, Value::String(key) => key);
            let content = get_var_or_elem_error!(CONTENT_PARAM, environment, value => value);

            Value::Element(Box::new(KoolElement::Text(Self {
                key,
                content: content.to_string(),
            })))
        };

        (
            NAME,
            Function {
                params: element_base_params(),
                code: FunctionCode::new_host(function),
                closure: None,
            },
        )
    }
}
