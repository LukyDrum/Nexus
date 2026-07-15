use crate::{
    element::{BuildContext, KoolBuilder, KoolElement},
    elemental::ElementalMessage,
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Column {
    pub key: String,
    pub content: Vec<KoolElement>,
}

impl KoolBuilder for Column {
    type IcedElement = iced::widget::Column<'static, ElementalMessage>;

    fn build(&self, context: BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Column::new().extend(
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
