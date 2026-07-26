use crate::{
    element::{BuildContext, Element, ErrorElement},
    language::Expression,
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Text {
    pub key: String,
    pub content: Expression,
}

impl<'a> Element<'a> for Text {
    type IcedElement = iced::widget::Text<'a>;

    fn build(&self, context: &'a BuildContext) -> Result<Self::IcedElement, ErrorElement> {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let content = self.content.evaluate_or_null(&context.variables);
        let widget = iced::widget::Text::new(content.to_string());

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        Ok(widget)
    }
}
