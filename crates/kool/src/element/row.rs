use crate::{
    element::{BuildContext, Element, KoolElement},
    elemental::ElementalMessage,
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Row {
    pub key: String,
    pub content: Vec<KoolElement>,
}

impl Element for Row {
    type IcedElement = iced::widget::Row<'static, ElementalMessage>;

    fn build(&self, context: BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Row::new().extend(
            self.content
                .iter()
                .map(|child| child.build(context.clone())),
        );

        if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        }
    }
}
