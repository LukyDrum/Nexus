use crate::{
    element::{BuildContext, Element},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Text {
    pub key: String,
    pub content: String,
}

impl<'a> Element<'a> for Text {
    type IcedElement = iced::widget::Text<'a>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Text::new(self.content.clone());

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        widget
    }
}
