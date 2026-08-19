use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{
            CONTENT_PARAM, KEY_PARAM, build_with_style, element_base_params,
            get_style_from_environment,
        },
    },
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
    style::{CommonStyle, StyleKey},
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
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Text::new(self.content.clone());

        build_with_style(widget, style, key, self.style)
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
