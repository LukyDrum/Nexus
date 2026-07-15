use serde::{Deserialize, Serialize};

use crate::style::Color;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CommonStyle {
    #[serde(flatten)]
    inheritable: InheritableStyle,

    width: Length,
    height: Length,

    border: Border,
    /// Going: TL, TR, BR, BL
    padding: [f32; 4],

    spacing: f32,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct InheritableStyle {
    background: Option<Color>,
    color: Option<Color>,
    highlight: Option<Color>,
    opacity: Option<f32>,

    text_size: Option<f32>,
    line_height: Option<f32>,
    align_x: Option<Align>,
    align_y: Option<Align>,
}

impl Default for CommonStyle {
    fn default() -> Self {
        Self {
            inheritable: InheritableStyle::default(),

            width: Length::Fill,
            height: Length::Fill,

            border: Border::default(),
            padding: [0.0; 4],

            spacing: 0.0,
        }
    }
}

impl CommonStyle {
    pub const DEFAULT_TEXT_SIZE: f32 = 17.0;

    pub fn background(&self) -> Option<iced::Background> {
        self.inheritable
            .background
            .map(|background| iced::Background::Color(background.into()))
    }

    pub fn color(&self) -> Option<iced::Color> {
        self.inheritable.color.map(Into::into)
    }

    pub fn highlight(&self) -> Option<iced::Color> {
        self.inheritable.highlight.map(Into::into)
    }

    pub fn opacity(&self) -> f32 {
        self.inheritable.opacity.unwrap_or(1.0)
    }

    pub fn text_size(&self) -> f32 {
        self.inheritable
            .text_size
            .unwrap_or(Self::DEFAULT_TEXT_SIZE)
    }

    pub fn line_height(&self) -> iced::widget::text::LineHeight {
        self.inheritable
            .line_height
            .map(|line_height| iced::widget::text::LineHeight::Absolute(line_height.into()))
            .unwrap_or_default()
    }

    pub fn align_x(&self) -> iced::Alignment {
        self.inheritable
            .align_x
            .map_or(iced::Alignment::Start, Into::into)
    }

    pub fn align_y(&self) -> iced::Alignment {
        self.inheritable
            .align_y
            .map_or(iced::Alignment::Start, Into::into)
    }

    pub fn width(&self) -> iced::Length {
        self.width.into()
    }

    pub fn height(&self) -> iced::Length {
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

    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    pub fn inherit(&self, parent: &Self) -> Self {
        Self {
            inheritable: self.inheritable.inherit(&parent.inheritable),
            ..*self
        }
    }
}

impl InheritableStyle {
    pub fn inherit(&self, parent: &Self) -> Self {
        Self {
            background: self.background.or(parent.background),
            color: self.color.or(parent.color),
            highlight: self.highlight.or(parent.highlight),
            opacity: self.opacity.or(parent.opacity),
            text_size: self.text_size.or(parent.text_size),
            line_height: self.line_height.or(parent.line_height),
            align_x: self.align_x.or(parent.align_x),
            align_y: self.align_y.or(parent.align_y),
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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

impl From<Align> for iced::Alignment {
    fn from(value: Align) -> Self {
        match value {
            Align::Start => Self::Start,
            Align::Center => Self::Center,
            Align::End => Self::End,
        }
    }
}

impl From<Align> for iced::widget::text::Alignment {
    fn from(value: Align) -> Self {
        match value {
            Align::Start => Self::Left,
            Align::Center => Self::Center,
            Align::End => Self::Right,
        }
    }
}
