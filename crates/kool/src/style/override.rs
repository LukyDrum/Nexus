use crate::style::CommonStyle;

pub struct ElementWithStyleOverride<T> {
    element: T,
    override_style: CommonStyle,
}

impl<T> ElementWithStyleOverride<T> {
    pub fn into_element(self) -> T {
        self.element
    }

    pub fn override_style(&self) -> &CommonStyle {
        &self.override_style
    }
}

impl<'a, Msg, T> From<ElementWithStyleOverride<T>> for iced::Element<'a, Msg>
where
    iced::Element<'a, Msg>: From<T>,
{
    fn from(value: ElementWithStyleOverride<T>) -> Self {
        value.into_element().into()
    }
}

pub trait WithStyleOverride {
    fn with_style_override(self, override_style: CommonStyle) -> ElementWithStyleOverride<Self>
    where
        Self: Sized,
    {
        ElementWithStyleOverride {
            element: self,
            override_style,
        }
    }
}
impl<T> WithStyleOverride for T {}
