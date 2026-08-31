use crate::{
    KoolElement,
    element::{
        BuildContext, Element,
        common::{
            CONTENT_PARAM, KEY_PARAM, element_base_params, get_style_from_environment,
            merge_with_default_style,
        },
        kool::Error,
    },
    get_var_or_elem_error,
    language::{Function, FunctionCode, SharedEnvironment, Value},
    runner::WidgetMessage,
    style::{CommonStyle, WithStyle},
    utils::CloneInner,
};

#[derive(Clone, Debug)]
pub struct Stack {
    pub key: String,
    pub content: Vec<KoolElement>,
    pub style: Option<CommonStyle>,
}

impl<'a> Element<'a> for Stack {
    type IcedElement = iced::widget::Stack<'a, WidgetMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let children = self.content.iter().map(|child| child.build(context));
        let widget = iced::widget::Stack::new().extend(children);

        let style = merge_with_default_style(self.style, context.default_style);

        widget.with_style(style)
    }

    fn kool_function() -> (&'static str, Function) {
        const NAME: &str = "Stack";

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
            let style = get_style_from_environment(environment);

            Value::new_element(KoolElement::Stack(Self {
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
