use crate::style::StyleKey;

pub struct ElementWithStyleId<T> {
    element: T,
    id: StyleKey,
}

impl<T> ElementWithStyleId<T> {
    pub fn style_id(&self) -> &StyleKey {
        &self.id
    }

    pub fn element(self) -> T {
        self.element
    }
}

impl<'a, Msg, T> From<ElementWithStyleId<T>> for iced::Element<'a, Msg>
where
    iced::Element<'a, Msg>: From<T>,
{
    fn from(value: ElementWithStyleId<T>) -> Self {
        value.element().into()
    }
}

pub trait WithStyleId {
    fn with_style_id(self, id: impl Into<StyleKey>) -> ElementWithStyleId<Self>
    where
        Self: Sized;
}

impl<T> WithStyleId for T {
    fn with_style_id(self, id: impl Into<StyleKey>) -> ElementWithStyleId<Self>
    where
        Self: Sized,
    {
        let id = id.into();
        let id = id.as_str().trim_start_matches('#');

        ElementWithStyleId {
            element: self,
            id: StyleKey::new(format!("#{id}")),
        }
    }
}
