use std::rc::Rc;

use crate::{elemental::ElementalMessage, language::Variables, style::StyleTree};

mod column;
mod container;
mod error;
mod image;
mod row;
mod stack;
mod text;
pub(crate) mod util;

pub use error::ErrorElement;

pub mod kool {
    pub use super::column::Column;
    pub use super::container::Container;
    pub use super::image::Image;
    pub use super::row::Row;
    pub use super::stack::Stack;
    pub use super::text::Text;
}

/// The context in which an element is buidl through `KoolBuilder`.
#[derive(Clone, Debug)]
pub struct BuildContext {
    pub variables: Variables,
    pub style: Rc<StyleTree>,
}

/// Defines the basic behaviour of a custom element.
pub trait Element<'a> {
    type IcedElement;

    fn build(&self, context: &'a BuildContext) -> Result<Self::IcedElement, ErrorElement>;

    fn type_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// The base building block of Kool.
/// Each variant roughly corresponds to an `iced` widget/element.
#[derive(Clone, Debug)]
pub enum KoolElement {
    /* BASIC */
    Text(kool::Text),
    Container(kool::Container),
    Image(kool::Image),

    /* LAYOUT */
    Column(kool::Column),
    Row(kool::Row),
    Stack(kool::Stack),
}

macro_rules! build {
    ($elem:ident, $context:expr) => {
        match $elem.build($context) {
            Ok(elem) => elem.into(),
            Err(error) => error.build($context).into(),
        }
    };
}

impl KoolElement {
    pub fn build<'a>(&self, context: &'a BuildContext) -> iced::Element<'a, ElementalMessage> {
        match self {
            KoolElement::Text(text) => build!(text, context),
            KoolElement::Container(container) => build!(container, context),
            KoolElement::Image(image) => build!(image, context),
            KoolElement::Column(column) => build!(column, context),
            KoolElement::Row(row) => build!(row, context),
            KoolElement::Stack(stack) => build!(stack, context),
        }
    }
}

impl PartialEq for KoolElement {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for KoolElement {}
