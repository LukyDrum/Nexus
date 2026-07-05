use serde::{Deserialize, Serialize};

use crate::style::Color;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CommonStyle {
    background: Option<Color>,
    color: Option<Color>,
    highlight: Option<Color>,

    border: Border,
    /// Going: TL, TR, BR, BL
    padding: [f32; 4],
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

    pub fn border(&self) -> iced::Border {
        let Border {
            color,
            width,
            radius,
        } = self.border;

        iced::Border::default()
            .color(color)
            .width(width)
            .rounded(iced::border::Radius {
                top_left: radius[0],
                top_right: radius[1],
                bottom_right: radius[2],
                bottom_left: radius[3],
            })
    }

    pub fn padding(&self) -> iced::Padding {
        let padding = self.padding;

        iced::Padding {
            top: padding[0],
            right: padding[1],
            bottom: padding[2],
            left: padding[3],
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
