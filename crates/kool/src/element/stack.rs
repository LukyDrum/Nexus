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

impl<'a> Element<'a> for Stack {
    type IcedElement = iced::widget::Stack<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let children = self.content.iter().map(|child| child.build(context));
        let widget = iced::widget::Stack::new().extend(children);

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        widget
    }
}
