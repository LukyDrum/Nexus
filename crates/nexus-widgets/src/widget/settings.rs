pub use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::LayerShellSettings;

#[derive(Clone, Debug)]
pub enum Size {
    Expand,
    Size(u32, u32),
}

#[derive(Clone, Debug)]
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
            anchor: Anchor::empty(),
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
            anchor: value.anchor,
            layer: value.layer,
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
