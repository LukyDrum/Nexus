use std::sync::Arc;

use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{
            CONTENT_PARAM, KEY_PARAM, build_with_style, element_base_params,
            get_style_from_environment,
        },
    },
    elemental::ElementalMessage,
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
    style::{CommonStyle, StyleKey},
    utils::CloneInner,
};

#[derive(Clone, Debug)]
pub struct Button {
    pub key: String,
    pub content: Arc<KoolElement>,
    pub callback: Arc<Function>,
    pub style: Option<CommonStyle>,
}

impl<'a> Element<'a> for Button {
    type IcedElement = iced::widget::Button<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let content = self.content.build(context);
        let widget = iced::widget::Button::new(content)
            .on_press(ElementalMessage::Callback(self.callback.clone()));

        build_with_style(widget, style, key, self.style)
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Button";
        const CALLBACK_PARAM: &str = "onClick";

        let params = element_base_params().with_param(
            CALLBACK_PARAM,
            Some(Value::Function(Arc::new(Function::empty_function()))),
        );
        let function = |environment: &SharedEnvironment| -> Value {
            let key = get_var_or_elem_error!(KEY_PARAM, environment, Value::String(key) => key)
                .clone_inner();
            let content = get_var_or_elem_error!(CONTENT_PARAM, environment, Value::Element(content) => content);
            let callback = get_var_or_elem_error!(CALLBACK_PARAM, environment, Value::Function(callback) => callback);
            let style = get_style_from_environment(environment);

            Value::new_element(KoolElement::Button(Self {
                key,
                content,
                callback,
                style,
            }))
        };

        (
            NAME,
            Function {
                params,
                code: FunctionCode::new_host(function),
                closure: None,
            },
        )
    }
}
