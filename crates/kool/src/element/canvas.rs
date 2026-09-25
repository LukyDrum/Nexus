use crate::element::common::{CONTENT_PARAM, get_style_from_environment, merge_with_default_style};

use crate::language::SharedEnvironment;
use crate::runner::WidgetMessage;
use crate::style::{CommonStyle, WithStyle};
use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{KEY_PARAM, element_base_params},
    },
    get_var_or_elem_error,
    language::{Function, FunctionCode, Value},
};

#[derive(Clone, Debug)]
pub struct Canvas {
    pub style: Option<CommonStyle>,
}

impl<'a> Element<'a> for Canvas {
    type IcedElement = iced::widget::Container<'a, WidgetMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let style = merge_with_default_style(self.style, context.default_style);

        widget.with_style(style)
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Canvas";

        let function = |environment: &SharedEnvironment| -> Value {
            let style = get_style_from_environment(environment);

            Value::new_element(KoolElement::Canvas(Self {
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
