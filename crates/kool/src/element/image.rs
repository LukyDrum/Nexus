use crate::{
    element::{BuildContext, Element},
    style::{StyleKey, WithStyle, WithStyleKey},
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

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        widget
    }
}
