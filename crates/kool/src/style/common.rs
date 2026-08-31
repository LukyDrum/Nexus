use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::style::Color;

static FONT_CACHE: LazyLock<Mutex<HashSet<&'static str>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CommonStyle {
    background: Option<Color>,
    color: Option<Color>,
    highlight: Option<Color>,
    opacity: Option<f32>,

    font: Option<FontName>,
    text_size: Option<f32>,
    line_height: Option<f32>,

    align_x: Option<Align>,
    align_y: Option<Align>,
    spacing: Option<f32>,

    width: Option<Length>,
    height: Option<Length>,

    border: Option<Border>,
    /// Going: top, right, bottom, left
    padding: Option<[f32; 4]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
struct FontName(&'static str);

impl<'de> Deserialize<'de> for FontName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        let mut cache = FONT_CACHE.lock().unwrap();

        if let Some(&static_str) = cache.get(string.as_str()) {
            Ok(Self(static_str))
        } else {
            let leaked = string.leak();
            cache.insert(leaked);
            Ok(Self(leaked))
        }
    }
}

impl CommonStyle {
    pub const DEFAULT_TEXT_SIZE: f32 = 17.0;

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
        self.opacity.unwrap_or(1.0)
    }

    pub fn font(&self) -> Option<iced::font::Font> {
        self.font
            .map(|FontName(name)| iced::font::Font::with_name(name))
    }

    pub fn text_size(&self) -> f32 {
        self.text_size.unwrap_or(Self::DEFAULT_TEXT_SIZE)
    }

    pub fn line_height(&self) -> iced::widget::text::LineHeight {
        self.line_height
            .map(|line_height| iced::widget::text::LineHeight::Absolute(line_height.into()))
            .unwrap_or_default()
    }

    pub fn align_x(&self) -> iced::Alignment {
        self.align_x.map_or(iced::Alignment::Start, Into::into)
    }

    pub fn align_y(&self) -> iced::Alignment {
        self.align_y.map_or(iced::Alignment::Start, Into::into)
    }

    pub fn width(&self) -> iced::Length {
        self.width.unwrap_or_default().into()
    }

    pub fn height(&self) -> iced::Length {
        self.height.unwrap_or_default().into()
    }

    pub fn border(&self) -> iced::Border {
        let Border {
            color,
            width,
            radius,
        } = self.border.unwrap_or_default();
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
        let [top, right, bottom, left] = self.padding.unwrap_or_default();

        iced::Padding {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn spacing(&self) -> f32 {
        self.spacing.unwrap_or_default()
    }

    /// Fills missing properties with values from `other`.
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            background: self.background.or(other.background),
            color: self.color.or(other.color),
            highlight: self.highlight.or(other.highlight),
            opacity: self.opacity.or(other.opacity),
            font: self.font.or(other.font),
            text_size: self.text_size.or(other.text_size),
            line_height: self.line_height.or(other.line_height),
            align_x: self.align_x.or(other.align_x),
            align_y: self.align_y.or(other.align_y),
            border: self.border.or(other.border),
            width: self.width.or(other.width),
            height: self.height.or(other.height),
            padding: self.padding.or(other.padding),
            spacing: self.spacing.or(other.spacing),
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

#[derive(Clone, Copy, Debug, Default)]
pub enum Length {
    Shrink,
    #[default]
    Fill,
    Fixed(f32),
}

impl<'de> Deserialize<'de> for Length {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Keyword(String),
            Fixed(f32),
        }

        match Helper::deserialize(deserializer)? {
            Helper::Keyword(s) => match s.as_str() {
                "Fill" => Ok(Length::Fill),
                "Shrink" => Ok(Length::Shrink),
                _ => Err(serde::de::Error::custom(format!(
                    "expected 'Fill', 'Shrink', or a number, found '{s}'"
                ))),
            },
            Helper::Fixed(n) => Ok(Length::Fixed(n)),
        }
    }
}

impl Serialize for Length {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Length::Shrink => serializer.serialize_str("Shrink"),
            Length::Fill => serializer.serialize_str("Fill"),
            Length::Fixed(n) => serializer.serialize_f32(*n),
        }
    }
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
