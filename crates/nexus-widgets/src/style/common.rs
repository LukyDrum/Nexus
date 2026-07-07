use serde::{Deserialize, Serialize};

use crate::style::Color;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CommonStyle {
    background: Option<Color>,
    color: Option<Color>,
    highlight: Option<Color>,

    opacity: f32,
    width: Length,
    height: Length,

    border: Border,
    /// Going: TL, TR, BR, BL
    padding: [f32; 4],
}

impl Default for CommonStyle {
    fn default() -> Self {
        Self {
            background: None,
            color: None,
            highlight: None,

            opacity: 1.0,
            width: Length::Fill,
            height: Length::Fill,

            border: Border::default(),
            padding: [0.0; 4],
        }
    }
}

impl CommonStyle {
    pub fn background(&self) -> Option<iced::Background> {
        self.background
            .map(|background| iced::Background::Color(background.into()))
    }

    pub fn color(&self) -> Option<iced::Color> {
        self.color.map(Into::into)
    }

    pub fn highlight(&self) -> Option<iced::Color> {
        self.highlight.map(Into::into)
    }

    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    pub fn width(&self) -> iced::Length {
        self.width.into()
    }

    pub fn height(&self) -> iced::Length {
        self.height.into()
    }

    pub fn line_height(&self) -> iced::widget::text::LineHeight {
        self.height.into()
    }

    pub fn border(&self) -> iced::Border {
        let Border {
            color,
            width,
            radius,
        } = self.border;
        let [top_left, top_right, bottom_right, bottom_left] = radius;

        iced::Border::default()
            .color(color)
            .width(width)
            .rounded(iced::border::Radius {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            })
    }

    pub fn padding(&self) -> iced::Padding {
        let [top, right, bottom, left] = self.padding;

        iced::Padding {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Border {
    color: Color,
    width: f32,
    /// Going: TL, TR, BR, BL
    radius: [f32; 4],
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Length {
    Shrink,
    #[default]
    Fill,
    Fixed(f32),
}

impl From<Length> for iced::Length {
    fn from(value: Length) -> Self {
        match value {
            Length::Shrink => Self::Shrink,
            Length::Fill => Self::Fill,
            Length::Fixed(value) => Self::Fixed(value),
        }
    }
}

impl From<Length> for iced::widget::text::LineHeight {
    fn from(value: Length) -> Self {
        match value {
            Length::Fixed(value) => Self::Absolute(value.into()),
            _ => Self::Relative(1.0),
        }
    }
}
