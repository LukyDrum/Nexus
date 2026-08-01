use crate::{
    KoolElement,
    element::{BuildContext, Element},
    elemental::ElementalMessage,
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Container {
    pub key: String,
    pub content: Box<KoolElement>,
}

impl<'a> Element<'a> for Container {
    type IcedElement = iced::widget::Container<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Container::new(self.content.build(context));

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        widget
    }
}
