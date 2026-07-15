use iced_layershell::reexport::{Anchor as IcedShellAnchor, Layer as IcedShellLayer};
use iced_layershell::settings::LayerShellSettings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WidgetSettings {
    pub id: Option<String>,
    pub anchor: Anchor,
    pub layer: Layer,
    pub exclusive_zone: i32,
    pub size: Size,
    pub margin: (i32, i32, i32, i32),
}

impl Default for WidgetSettings {
    fn default() -> Self {
        Self {
            id: None,
            anchor: Anchor::None,
            layer: Layer::Top,
            exclusive_zone: 0,
            size: Size::Size(100, 100),
            margin: (0, 0, 0, 0),
        }
    }
}

impl From<WidgetSettings> for iced_layershell::Settings {
    fn from(value: WidgetSettings) -> Self {
        let size = match value.size {
            Size::Expand => None,
            Size::Size(x, y) => Some((x, y)),
        };

        let layer_settings = LayerShellSettings {
            anchor: value.anchor.into(),
            layer: value.layer.into(),
            exclusive_zone: value.exclusive_zone,
            size,
            margin: value.margin,
            ..Default::default()
        };

        iced_layershell::Settings {
            id: value.id,
            layer_settings,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Size {
    #[default]
    Expand,
    Size(u32, u32),
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Anchor {
    #[default]
    None,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl From<Anchor> for IcedShellAnchor {
    fn from(value: Anchor) -> Self {
        match value {
            Anchor::None => Self::empty(),
            Anchor::Top => Self::Top,
            Anchor::TopRight => Self::Top | Self::Right,
            Anchor::Right => Self::Right,
            Anchor::BottomRight => Self::Bottom | Self::Right,
            Anchor::Bottom => Self::Bottom,
            Anchor::BottomLeft => Self::Bottom | Self::Left,
            Anchor::Left => Self::Left,
            Anchor::TopLeft => Self::Top | Self::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

impl From<Layer> for IcedShellLayer {
    fn from(value: Layer) -> Self {
        match value {
            Layer::Background => Self::Background,
            Layer::Bottom => Self::Bottom,
            Layer::Top => Self::Top,
            Layer::Overlay => Self::Overlay,
        }
    }
}
