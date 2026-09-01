use std::collections::HashMap;
use std::str::FromStr;

use iced_exwlshell::reexport::{Anchor as IcedShellAnchor, Layer as IcedShellLayer};

use iced_exwlshell::settings::LayerShellSettings;

use strum::EnumString;

use crate::language::Value;

#[derive(Clone, Debug)]
pub struct WidgetSettings {
    pub name: Option<String>,
    pub anchors: Vec<Anchor>,
    pub layer: Layer,
    pub exclusive_zone: i32,
    pub size: Option<(u32, u32)>,
    pub margin: (i32, i32, i32, i32),
    pub click_through: bool,
}

impl Default for WidgetSettings {
    fn default() -> Self {
        Self {
            name: None,
            anchors: Vec::new(),
            layer: Layer::Top,
            exclusive_zone: 0,
            size: None,
            margin: (0, 0, 0, 0),
            click_through: false,
        }
    }
}

impl From<WidgetSettings> for iced_exwlshell::Settings {
    fn from(value: WidgetSettings) -> Self {
        let id = value.name.clone();
        let layer_settings = value.into();

        iced_exwlshell::Settings {
            id,
            layer_settings,
            ..Default::default()
        }
    }
}

impl From<WidgetSettings> for LayerShellSettings {
    fn from(value: WidgetSettings) -> Self {
        LayerShellSettings {
            anchor: value
                .anchors
                .into_iter()
                .fold(IcedShellAnchor::empty(), |acc, anchor| acc | anchor.into()),
            layer: value.layer.into(),
            exclusive_zone: value.exclusive_zone,
            size: value.size,
            margin: value.margin,
            events_transparent: value.click_through,
            ..Default::default()
        }
    }
}

impl From<WidgetSettings> for iced_exwlshell::reexport::NewLayerShellSettings {
    fn from(value: WidgetSettings) -> Self {
        iced_exwlshell::reexport::NewLayerShellSettings {
            size: value.size,
            layer: value.layer.into(),
            anchor: value
                .anchors
                .into_iter()
                .fold(IcedShellAnchor::empty(), |acc, anchor| acc | anchor.into()),
            exclusive_zone: Some(value.exclusive_zone),
            margin: Some(value.margin),
            events_transparent: value.click_through,
            ..Default::default()
        }
    }
}

impl From<Value> for WidgetSettings {
    fn from(value: Value) -> Self {
        let mut settings = WidgetSettings::default();

        let Value::HashMap(value_map) = value else {
            return settings;
        };
        let value_map = value_map.read().expect("Lock poisoned");
        let map = value_map
            .iter()
            .filter_map(|(key, value)| {
                if let Value::String(key) = key {
                    Some((key.as_str(), value))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        if let Some(anchors) = map.get("anchor") {
            let anchors = match anchors {
                Value::String(anchor) => Anchor::from_str(anchor).ok().map(|anchor| vec![anchor]),
                Value::Array(anchors) => {
                    let anchors = anchors.read().expect("Lock poisoned");
                    anchors
                        .iter()
                        .map(|anchor| match anchor {
                            Value::String(anchor) => Anchor::from_str(anchor).ok(),
                            _ => None,
                        })
                        .collect()
                }
                _ => None,
            };

            settings.anchors = anchors.unwrap_or_default();
        }

        if let Some(Value::String(layer)) = map.get("layer")
            && let Ok(layer) = Layer::from_str(layer)
        {
            settings.layer = layer;
        }

        if let Some(Value::Int(exclusive_zone)) = map.get("exclusiveZone") {
            settings.exclusive_zone = *exclusive_zone as i32;
        }

        if let Some(Value::Array(size)) = map.get("size") {
            let size = size.read().expect("Lock poisoned");
            if let Some(Value::Int(size_x)) = size.first()
                && let Some(Value::Int(size_y)) = size.get(1)
            {
                settings.size = Some((*size_x as u32, *size_y as u32));
            }
        }

        if let Some(Value::Array(margin)) = map.get("margin") {
            let size = margin.read().expect("Lock poisoned");
            if let Some(Value::Int(maring_top)) = size.first()
                && let Some(Value::Int(margin_right)) = size.get(1)
                && let Some(Value::Int(margin_bottom)) = size.get(2)
                && let Some(Value::Int(margin_left)) = size.get(3)
            {
                settings.margin = (
                    *maring_top as i32,
                    *margin_right as i32,
                    *margin_bottom as i32,
                    *margin_left as i32,
                );
            }
        }

        if let Some(Value::Bool(click_through)) = map.get("clickThrough") {
            settings.click_through = *click_through;
        }

        settings
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Anchor {
    Top,
    Right,
    Bottom,
    Left,
}

impl From<Anchor> for IcedShellAnchor {
    fn from(value: Anchor) -> Self {
        match value {
            Anchor::Top => Self::Top,
            Anchor::Right => Self::Right,
            Anchor::Bottom => Self::Bottom,
            Anchor::Left => Self::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Rendering {
    Cpu,
    #[default]
    Gpu,
}
