use serde::{Deserialize, Serialize};

mod common;
mod palette;

pub use common::*;
pub use palette::{Color, Palette};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WidgetAppStyle {
    #[serde(default = "Palette::dark")]
    pub palette: Palette,
    pub widget: CommonStyle,
}

impl WidgetAppStyle {
    pub fn default_dark() -> Self {
        Self {
            palette: Palette::dark(),
            widget: CommonStyle::default(),
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::custom("custom", self.palette.clone().into())
    }
}
