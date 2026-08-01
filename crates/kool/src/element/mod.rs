use std::rc::Rc;

use crate::{elemental::ElementalMessage, style::StyleTree};

mod column;
mod container;
mod error;
mod image;
mod row;
mod stack;
mod text;

pub mod kool {
    pub use super::column::Column;
    pub use super::container::Container;
    pub use super::error::Error;
    pub use super::image::Image;
    pub use super::row::Row;
    pub use super::stack::Stack;
    pub use super::text::Text;
}

/// The context in which an element is buidl through `KoolBuilder`.
#[derive(Clone, Debug)]
pub struct BuildContext {
    pub style: Rc<StyleTree>,
}

/// Defines the basic behaviour of a custom element.
pub trait Element<'a> {
    type IcedElement;

    fn build(&self, context: &BuildContext) -> Self::IcedElement;
}

/// The base building block of Kool.
/// Each variant roughly corresponds to an `iced` widget/element.
#[derive(Clone, Debug)]
pub enum KoolElement {
    /* SPECIAL */
    Error(kool::Error),

    /* BASIC */
    Text(kool::Text),
    Container(kool::Container),
    Image(kool::Image),

    /* LAYOUT */
    Column(kool::Column),
    Row(kool::Row),
    Stack(kool::Stack),
}

impl<'a> Element<'a> for KoolElement {
    type IcedElement = iced::Element<'a, ElementalMessage>;

    fn build(&self, context: &BuildContext) -> Self::IcedElement {
        match self {
            KoolElement::Error(error) => error.build(context).into(),
            KoolElement::Text(text) => text.build(context).into(),
            KoolElement::Container(container) => container.build(context).into(),
            KoolElement::Image(image) => image.build(context).into(),
            KoolElement::Column(column) => column.build(context).into(),
            KoolElement::Row(row) => row.build(context).into(),
            KoolElement::Stack(stack) => stack.build(context).into(),
        }
    }
}

impl KoolElement {
    pub fn element_type(&self) -> &'static str {
        match self {
            KoolElement::Error(_) => "Error",
            KoolElement::Text(_) => "Text",
            KoolElement::Container(_) => "Container",
            KoolElement::Image(_) => "Image",
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
