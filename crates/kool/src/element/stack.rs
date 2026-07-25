use crate::{
    KoolElement,
    element::{BuildContext, Element},
    elemental::ElementalMessage,
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Stack {
    pub key: String,
    pub content: Vec<KoolElement>,
}

impl Element for Stack {
    type IcedElement = iced::widget::Stack<'static, ElementalMessage>;

    fn build(&self, context: BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let widget = iced::widget::Stack::new().extend(
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
