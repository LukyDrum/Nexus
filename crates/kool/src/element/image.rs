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
pub struct Image {
    pub key: String,
    pub file: String,
    pub style: Option<CommonStyle>,
}

impl<'a> Element<'a> for Image {
    type IcedElement = iced::widget::Image;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Image::new(&self.file);

        build_with_style(widget, style, key, self.style)
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Image";

        let function = |environment: &SharedEnvironment| -> Value {
            let key = get_var_or_elem_error!(KEY_PARAM, environment, Value::String(key) => key)
                .clone_inner();
            let file =
                get_var_or_elem_error!(CONTENT_PARAM, environment, Value::String(file) => file)
                    .clone_inner();
            let style = get_style_from_environment(environment);

            Value::new_element(KoolElement::Image(Self { key, file, style }))
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
