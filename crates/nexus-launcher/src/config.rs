use std::collections::HashMap;

use kool::{settings::Anchor, style::WidgetStyle};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherConfig {
    pub size: (u32, u32),
    pub anchor: Anchor,
    pub bg_image: Option<String>,
    pub terminal_cmd: Option<String>,
    pub actions: HashMap<String, String>,
    pub visual: WidgetStyle,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            size: (600, 400),
            anchor: Anchor::None,
            bg_image: None,
            terminal_cmd: None,
            actions: HashMap::new(),
            visual: WidgetStyle::default_dark(),
        }
    }
}
