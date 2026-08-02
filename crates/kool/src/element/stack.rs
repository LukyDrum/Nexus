use std::rc::Rc;

use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{KEY_VAR, key_function_param},
        kool::Error,
    },
    elemental::ElementalMessage,
    get_var_or_elem_error,
    language::{Environment, Function, FunctionCode, TAIL_VAR, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Stack {
    pub key: String,
    pub content: Vec<KoolElement>,
}

impl<'a> Element<'a> for Stack {
    type IcedElement = iced::widget::Stack<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let children = self.content.iter().map(|child| child.build(context));
        let widget = iced::widget::Stack::new().extend(children);

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        widget
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Stack";

        let params = vec![key_function_param()];

        let code = |environment: &mut Environment| -> Value {
            let key = get_var_or_elem_error!(KEY_VAR, environment, Value::String(key) => key);

            let tail = get_var_or_elem_error!(TAIL_VAR, environment, Value::Array(array) => array);
            let content = tail
                .into_iter()
                .map(|value| match value {
                    Value::Element(element) => *element,
                    other => {
                        KoolElement::Error(Error::new(format!("expected element, found: {other}")))
                    }
                })
                .collect();

            Value::Element(Box::new(KoolElement::Stack(Self { key, content })))
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
