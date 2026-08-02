use std::rc::Rc;

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
pub struct Image {
    pub key: String,
    pub file: String,
}

impl<'a> Element<'a> for Image {
    type IcedElement = iced::widget::Image;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Image::new(&self.file);

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        widget
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Image";

        let params = vec![key_function_param()];

        let code = |environment: &mut Environment| -> Value {
            let key = get_var_or_elem_error!(KEY_VAR, environment, Value::String(key) => key);
            let file = get_var_or_elem_error!(TAIL_VAR, environment, Value::String(file) => file);

            Value::Element(Box::new(KoolElement::Image(Self { key, file })))
        };

        (
            NAME,
            Function {
                params,
                code: FunctionCode::Host(Rc::new(code)),
            },
        )
    }
}
