use crate::{
    language::{Function, Library},
    runner::WidgetMessage,
    style::CommonStyle,
};

mod button;
mod canvas;
mod column;
mod common;
mod container;
mod error;
mod image;
mod row;
mod stack;
mod text;

pub mod kool {
    pub use super::button::Button;
    pub use super::canvas::Canvas;
    pub use super::column::Column;
    pub use super::container::Container;
    pub use super::error::Error;
    pub use super::image::Image;
    pub use super::row::Row;
    pub use super::stack::Stack;
    pub use super::text::Text;
}

/// The context in which an element is build.
#[derive(Clone, Debug)]
pub struct BuildContext {
    pub default_style: CommonStyle,
}

/// Defines the basic behaviour of a custom element.
pub trait Element<'a> {
    type IcedElement;

    fn build(&self, context: &BuildContext) -> Self::IcedElement;

    fn kool_function() -> (&'static str, Function);
}

/// The base building block of Kool.
/// Each variant roughly corresponds to an `iced` widget/element.
#[derive(Clone, Debug)]
pub enum KoolElement {
    /* SPECIAL */
    Error(kool::Error),
    Canvas(kool::Canvas),

    /* BASIC */
    Text(kool::Text),
    Container(kool::Container),
    Image(kool::Image),
    Button(kool::Button),

    /* LAYOUT */
    Column(kool::Column),
    Row(kool::Row),
    Stack(kool::Stack),
}

impl KoolElement {
    pub fn build<'a>(&self, context: &BuildContext) -> iced::Element<'a, WidgetMessage> {
        match self {
            KoolElement::Error(error) => error.build(context).into(),
            KoolElement::Canvas(canvas) => canvas.build(context).into(),
            KoolElement::Text(text) => text.build(context).into(),
            KoolElement::Container(container) => container.build(context).into(),
            KoolElement::Image(image) => image.build(context).into(),
            KoolElement::Button(button) => button.build(context).into(),
            KoolElement::Column(column) => column.build(context).into(),
            KoolElement::Row(row) => row.build(context).into(),
            KoolElement::Stack(stack) => stack.build(context).into(),
        }
    }

    pub fn element_type(&self) -> &'static str {
        match self {
            KoolElement::Error(_) => "Error",
            KoolElement::Canvas(_) => "Canvas",
            KoolElement::Text(_) => "Text",
            KoolElement::Container(_) => "Container",
            KoolElement::Image(_) => "Image",
            KoolElement::Button(_) => "Button",
            KoolElement::Column(_) => "Column",
            KoolElement::Row(_) => "Row",
            KoolElement::Stack(_) => "Stack",
        }
    }
}

impl PartialEq for KoolElement {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for KoolElement {}

pub fn element_library() -> Library {
    Library::from([
        kool::Error::kool_function(),
        kool::Canvas::kool_function(),
        kool::Text::kool_function(),
        kool::Container::kool_function(),
        kool::Image::kool_function(),
        kool::Button::kool_function(),
        kool::Column::kool_function(),
        kool::Row::kool_function(),
        kool::Stack::kool_function(),
    ])
}
