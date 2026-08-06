use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{CONTENT_PARAM, KEY_PARAM, element_base_params},
    },
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
    utils::CloneInner,
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

        if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        }
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Image";

        let function = |environment: &SharedEnvironment| -> Value {
            let key = get_var_or_elem_error!(KEY_PARAM, environment, Value::String(key) => key)
                .clone_inner();
            let file =
                get_var_or_elem_error!(CONTENT_PARAM, environment, Value::String(file) => file)
                    .clone_inner();

            Value::new_element(KoolElement::Image(Self { key, file }))
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
