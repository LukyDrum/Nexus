use serde::{Deserialize, Serialize};

use crate::style::palette::Palette;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Style {
    palette: Palette,
}

impl Style {
    pub fn default_dark() -> Self {
        Self {
            palette: Palette::dark(),
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::custom("custom", self.palette.clone().into())
    }
}
