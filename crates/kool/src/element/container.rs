use crate::{
    element::{BuildContext, Element, error::ErrorElement},
    elemental::ElementalMessage,
    eval_or_return_error,
    language::{Expression, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Container {
    pub key: String,
    pub content: Expression,
}

impl<'a> Element<'a> for Container {
    type IcedElement = iced::widget::Container<'a, ElementalMessage>;

    fn build(&self, context: &'a BuildContext) -> Result<Self::IcedElement, ErrorElement> {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let content =
            eval_or_return_error!(&self.content, &context.variables, Value::Element(elem) => elem);
        let widget = iced::widget::Container::new(content.build(context));

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        Ok(widget)
    }
}
