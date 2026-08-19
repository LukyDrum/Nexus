use std::sync::Arc;

use crate::element::common::{CONTENT_PARAM, build_with_style, get_style_from_environment};

use crate::language::SharedEnvironment;
use crate::style::CommonStyle;
use crate::utils::CloneInner;
use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{KEY_PARAM, element_base_params},
    },
    elemental::ElementalMessage,
    get_var_or_elem_error,
    language::{Function, FunctionCode, Value},
    style::StyleKey,
};

#[derive(Clone, Debug)]
pub struct Container {
    pub key: String,
    pub content: Arc<KoolElement>,
    pub style: Option<CommonStyle>,
}

impl<'a> Element<'a> for Container {
    type IcedElement = iced::widget::Container<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Container::new(self.content.build(context));

        build_with_style(widget, style, key, self.style)
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Container";

        let function = |environment: &SharedEnvironment| -> Value {
            let key = get_var_or_elem_error!(KEY_PARAM, environment, Value::String(key) => key)
                .clone_inner();
            let content = get_var_or_elem_error!(CONTENT_PARAM, environment, Value::Element(element) => element);
            let style = get_style_from_environment(environment);

            Value::new_element(KoolElement::Container(Self {
                key,
                content,
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
