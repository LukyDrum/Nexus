use crate::{
    element::{BuildContext, Element, error::ErrorElement},
    eval_or_return_error,
    language::{Expression, Value},
    style::{StyleKey, WithStyle, WithStyleKey},
};

#[derive(Clone, Debug)]
pub struct Image {
    pub key: String,
    pub file: Expression,
}

impl<'a> Element<'a> for Image {
    type IcedElement = iced::widget::Image;

    fn build(&self, context: &'a BuildContext) -> Result<Self::IcedElement, ErrorElement> {
        let key = StyleKey::new(self.key.clone());
        let style = context.style.clone();

        let file =
            eval_or_return_error!(&self.file, &context.variables, Value::String(file) => file);
        let widget = iced::widget::Image::new(file);

        let widget = if key.is_empty() {
            widget.with_style(style)
        } else {
            widget.with_style_key(key).with_style(style).element()
        };

        Ok(widget)
    }
}
