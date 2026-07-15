use crate::{
    element::{BuildContext, KoolBuilder},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Text {
    pub key: String,
    pub content: String,
}

impl KoolBuilder for Text {
    type IcedElement = iced::widget::Text<'static>;

    fn build(&self, context: BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style;

        let widget = iced::widget::Text::new(self.content.clone());

        if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        }
    }
}
