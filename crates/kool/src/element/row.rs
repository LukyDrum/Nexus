use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{CONTENT_PARAM, KEY_PARAM, element_base_params},
        kool::Error,
    },
    elemental::ElementalMessage,
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
    utils::CloneInner,
};

#[derive(Clone, Debug)]
pub struct Row {
    pub key: String,
    pub content: Vec<KoolElement>,
}

impl<'a> Element<'a> for Row {
    type IcedElement = iced::widget::Row<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let children = self.content.iter().map(|child| child.build(context));
        let widget = iced::widget::Row::new().extend(children);

        if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        }
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Row";

        let function = |environment: &SharedEnvironment| -> Value {
            let key = get_var_or_elem_error!(KEY_PARAM, environment, Value::String(key) => key)
                .clone_inner();

            let content =
                get_var_or_elem_error!(CONTENT_PARAM, environment, Value::Array(array) => array)
                    .clone_inner();
            let content = content
                .into_iter()
                .map(|value| match value {
                    Value::Element(element) => element.clone_inner(),
                    other => {
                        KoolElement::Error(Error::new(format!("expected element, found: {other}")))
                    }
                })
                .collect();

            Value::new_element(KoolElement::Row(Self { key, content }))
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
