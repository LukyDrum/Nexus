use std::rc::Rc;

use crate::{elemental::ElementalMessage, style::StyleTree};

mod column;
mod container;
pub(crate) mod environment;
mod image;
mod output;
mod row;
mod stack;
mod text;

pub mod kool {
    pub use super::column::Column;
    pub use super::container::Container;
    pub use super::image::Image;
    pub use super::output::Output;
    pub use super::row::Row;
    pub use super::stack::Stack;
    pub use super::text::Text;
}

/// The context in which an element is buidl through `KoolBuilder`.
#[derive(Clone, Debug)]
pub struct BuildContext {
    pub style: Rc<StyleTree>,
}

/// Defines the transformation from a custom element (or any type really) to a `iced` widget/element.
pub trait KoolBuilder {
    type IcedElement;

    fn build(&self, context: BuildContext) -> Self::IcedElement;
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

    /* SPECIAL */
    Output(kool::Output),
}

impl KoolBuilder for KoolElement {
    type IcedElement = iced::Element<'static, ElementalMessage>;

    fn build(&self, context: BuildContext) -> Self::IcedElement {
        match self {
            KoolElement::Text(text) => text.build(context).into(),
            KoolElement::Container(container) => container.build(context).into(),
            KoolElement::Image(image) => image.build(context).into(),
            KoolElement::Column(column) => column.build(context).into(),
            KoolElement::Row(row) => row.build(context).into(),
            KoolElement::Stack(stack) => stack.build(context).into(),
            KoolElement::Output(output) => output.build(context).into(),
        }
    }
}
