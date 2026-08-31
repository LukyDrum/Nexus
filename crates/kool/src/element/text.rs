use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{
            CONTENT_PARAM, KEY_PARAM, element_base_params, get_style_from_environment,
            merge_with_default_style,
        },
    },
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
    style::{CommonStyle, WithStyle},
    utils::CloneInner,
};

#[derive(Clone, Debug)]
pub struct Text {
    pub key: String,
    pub content: String,
    pub style: Option<CommonStyle>,
}

impl<'a> Element<'a> for Text {
    type IcedElement = iced::widget::Text<'a>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let widget = iced::widget::Text::new(self.content.clone());
        let style = merge_with_default_style(self.style, context.default_style);

        widget.with_style(style)
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Text";

        let function = |environment: &SharedEnvironment| -> Value {
            let key = get_var_or_elem_error!(KEY_PARAM, environment, Value::String(key) => key)
                .clone_inner();
            let content = get_var_or_elem_error!(CONTENT_PARAM, environment, value => value);
            let style = get_style_from_environment(environment);

            Value::new_element(KoolElement::Text(Self {
                key,
                content: content.to_string(),
                style,
            }))
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
