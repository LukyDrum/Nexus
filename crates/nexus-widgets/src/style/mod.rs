use serde::{Deserialize, Serialize};

mod palette;
mod style;

pub use palette::{Color, Palette};
pub use style::Style;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WidgetAppStyle {
    pub main: Style,
}

impl WidgetAppStyle {
    pub fn default_dark() -> Self {
        Self {
            main: Style::default_dark(),
        }
    }

    pub fn main_theme(&self) -> iced::Theme {
        self.main.theme()
    }
}
