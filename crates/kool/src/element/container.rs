use crate::element::kool::Error;
use std::rc::Rc;

use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{KEY_VAR, key_function_param},
    },
    elemental::ElementalMessage,
    get_var_or_elem_error,
    language::{Environment, Function, FunctionCode, TAIL_VAR, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Container {
    pub key: String,
    pub content: Box<KoolElement>,
}

impl<'a> Element<'a> for Container {
    type IcedElement = iced::widget::Container<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Container::new(self.content.build(context));

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        widget
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Container";

        let params = vec![key_function_param()];

        let code = |environment: &mut Environment| -> Value {
            let key = get_var_or_elem_error!(KEY_VAR, environment, Value::String(key) => key);
            let content =
                get_var_or_elem_error!(TAIL_VAR, environment, Value::Element(element) => element);

            Value::Element(Box::new(KoolElement::Container(Self { key, content })))
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
