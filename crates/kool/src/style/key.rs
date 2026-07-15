use serde::{Deserialize, Serialize};

const DELIMETER: &str = ".";

#[derive(Clone, Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyleKey(String);

impl StyleKey {
    pub fn new(key: String) -> Self {
        Self(key)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn join(self, other: &StyleKey) -> Self {
        if other.as_str().is_empty() {
            return self;
        }

        Self(self.0 + DELIMETER + other.as_str())
    }

    pub fn is_base(&self) -> bool {
        !self.0.contains(DELIMETER)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn base_and_rest(&self) -> (StyleKey, Option<StyleKey>) {
        if let Some((base, rest)) = self.0.split_once(DELIMETER) {
            (Self::from(base), Some(Self::from(rest)))
        } else {
            (self.clone(), None)
        }
    }
}

impl From<&str> for StyleKey {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl<T> From<T> for StyleKey
where
    T: AsStyleKey,
{
    fn from(value: T) -> Self {
        value.as_style_key()
    }
}

pub trait AsStyleKey {
    fn as_style_key(&self) -> StyleKey;
}

impl AsStyleKey for iced::widget::button::Status {
    fn as_style_key(&self) -> StyleKey {
        let key = match self {
            iced::widget::button::Status::Active => "active",
            iced::widget::button::Status::Hovered => "hovered",
            iced::widget::button::Status::Pressed => "pressed",
            iced::widget::button::Status::Disabled => "disabled",
        };

        key.into()
    }
}

impl AsStyleKey for iced::widget::text_input::Status {
    fn as_style_key(&self) -> StyleKey {
        let key = match self {
            iced::widget::text_input::Status::Active => "active",
            iced::widget::text_input::Status::Hovered => "hovered",
            iced::widget::text_input::Status::Focused { .. } => "focused",
            iced::widget::text_input::Status::Disabled => "disabled",
        };

        key.into()
    }
}

impl AsStyleKey for iced::widget::scrollable::Status {
    fn as_style_key(&self) -> StyleKey {
        let key = match self {
            iced::widget::scrollable::Status::Active { .. } => "active",
            iced::widget::scrollable::Status::Hovered { .. } => "hovered",
            iced::widget::scrollable::Status::Dragged { .. } => "dragged",
        };

        key.into()
    }
}
