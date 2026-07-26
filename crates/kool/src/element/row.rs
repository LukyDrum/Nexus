use crate::{
    element::{BuildContext, Element, ErrorElement, util::into_child_elements},
    elemental::ElementalMessage,
    eval_or_return_error,
    language::{Expression, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Row {
    pub key: String,
    pub content: Expression,
}

impl<'a> Element<'a> for Row {
    type IcedElement = iced::widget::Row<'a, ElementalMessage>;

    fn build(&self, context: &'a BuildContext) -> Result<Self::IcedElement, ErrorElement> {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let content =
            eval_or_return_error!(&self.content, &context.variables, Value::Array(array) => array);
        let children = into_child_elements(content, context);

        let widget = iced::widget::Row::new().extend(children);

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        Ok(widget)
    }
}
