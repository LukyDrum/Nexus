use std::sync::Arc;

use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{
            CONTENT_PARAM, element_base_params, get_style_from_environment,
            merge_with_default_style,
        },
    },
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
    runner::WidgetMessage,
    style::{CommonStyle, WithStyle},
};

#[derive(Clone, Debug)]
pub struct Button {
    pub content: Arc<KoolElement>,
    pub callback: Arc<Function>,
    pub style: Option<CommonStyle>,
}

impl<'a> Element<'a> for Button {
    type IcedElement = iced::widget::Button<'a, WidgetMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let content = self.content.build(context);
        let widget = iced::widget::Button::new(content)
            .on_press(WidgetMessage::Callback(self.callback.clone()));

        let style = merge_with_default_style(self.style, context.default_style);

        widget.with_style(style)
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Button";
        const CALLBACK_PARAM: &str = "onClick";

        let params = element_base_params().with_param(
            CALLBACK_PARAM,
            Some(Value::Function(Arc::new(Function::empty_function()))),
        );
        let function = |environment: &SharedEnvironment| -> Value {
            let content = get_var_or_elem_error!(CONTENT_PARAM, environment, Value::Element(content) => content);
            let callback = get_var_or_elem_error!(CALLBACK_PARAM, environment, Value::Function(callback) => callback);
            let style = get_style_from_environment(environment);

            Value::new_element(KoolElement::Button(Self {
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
