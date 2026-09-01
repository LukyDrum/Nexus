use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::{LazyLock, Mutex},
};

use strum::EnumString;

use crate::{
    language::Value,
    style::Color,
    utils::{CloneInner, SimpleCase},
};

static FONT_CACHE: LazyLock<Mutex<HashSet<&'static str>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct FontName(&'static str);

impl FontName {
    pub fn new(string: String) -> Self {
        let mut cache = FONT_CACHE.lock().unwrap();

        if let Some(&static_str) = cache.get(string.as_str()) {
            Self(static_str)
        } else {
            let leaked = string.leak();
            cache.insert(leaked);
            Self(leaked)
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Border {
    color: Color,
    width: f32,
    /// Going: TL, TR, BR, BL
    radius: [f32; 4],
}

#[derive(Clone, Copy, Debug, Default, EnumString)]
#[strum(ascii_case_insensitive)]
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

#[derive(Clone, Copy, Debug, Default, EnumString)]
#[strum(ascii_case_insensitive)]
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

impl From<Value> for CommonStyle {
    fn from(value: Value) -> Self {
        let mut style = CommonStyle::default();

        let Value::HashMap(map) = value else {
            return style;
        };
        let map = map.read().expect("Lock poisoned");
        let map = map
            .iter()
            .filter_map(|(key, value)| {
                if let Value::String(string) = key {
                    Some((SimpleCase::new(string).to_string(), value))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        if let Some(value) = map.get("background") {
            style.background = Some(Color::from(*value));
        }
        if let Some(value) = map.get("color") {
            style.color = Some(Color::from(*value));
        }
        if let Some(value) = map.get("highlight") {
            style.highlight = Some(Color::from(*value));
        }
        if let Some(_opacity) = map.get("opacity") {
            unimplemented!("opacity from float");
        }

        if let Some(Value::String(string)) = map.get("font") {
            style.font = Some(FontName::new(string.clone_inner()));
        }
        if let Some(Value::Int(text_size)) = map.get("textsize") {
            style.text_size = Some(*text_size as f32);
        }
        if let Some(Value::Int(line_height)) = map.get("lineheight") {
            style.line_height = Some(*line_height as f32);
        }

        if let Some(align_x) = map.get("alignx").and_then(|align| {
            if let Value::String(align) = align {
                Align::from_str(align).ok()
            } else {
                None
            }
        }) {
            style.align_x = Some(align_x);
        }
        if let Some(align_y) = map.get("aligny").and_then(|align| {
            if let Value::String(align) = align {
                Align::from_str(align).ok()
            } else {
                None
            }
        }) {
            style.align_y = Some(align_y);
        }
        if let Some(Value::Int(spacing)) = map.get("spacing") {
            style.spacing = Some(*spacing as f32);
        }

        if let Some(width) = map.get("width") {
            match width {
                Value::Int(width) => style.width = Some(Length::Fixed(*width as f32)),
                Value::String(width) => style.width = Length::from_str(width).ok(),
                _ => {}
            }
        }
        if let Some(height) = map.get("height") {
            match height {
                Value::Int(height) => style.height = Some(Length::Fixed(*height as f32)),
                Value::String(height) => style.height = Length::from_str(height).ok(),
                _ => {}
            }
        }

        if let Some(Value::HashMap(map)) = map.get("border") {
            let map = map.read().expect("Lock poisoned");
            let map = map
                .iter()
                .filter_map(|(key, value)| {
                    if let Value::String(string) = key {
                        Some((SimpleCase::new(string).to_string(), value))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>();
            let mut border = Border::default();

            if let Some(value) = map.get("color") {
                border.color = Color::from(*value);
            }
            if let Some(Value::Int(width)) = map.get("width") {
                border.width = *width as f32;
            }
            if let Some(Value::Array(radius)) = map.get("radius") {
                let radius = radius.read().expect("Lock poisoned");
                if let Some(Value::Int(tl)) = radius.first()
                    && let Some(Value::Int(tr)) = radius.get(1)
                    && let Some(Value::Int(br)) = radius.get(2)
                    && let Some(Value::Int(bl)) = radius.get(3)
                {
                    border.radius = [*tl as f32, *tr as f32, *br as f32, *bl as f32];
                }
            }

            style.border = Some(border);
        }

        if let Some(Value::Array(padding)) = map.get("padding") {
            let padding = padding.read().expect("Lock poisoned");
            if let Some(Value::Int(top)) = padding.first()
                && let Some(Value::Int(right)) = padding.get(1)
                && let Some(Value::Int(bottom)) = padding.get(2)
                && let Some(Value::Int(left)) = padding.get(3)
            {
                style.padding = Some([*top as f32, *right as f32, *bottom as f32, *left as f32]);
            }
        }

        style
    }
}
