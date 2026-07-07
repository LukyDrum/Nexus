use nexus_widgets::{settings::Anchor, style::WidgetStyle};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherConfig {
    pub size: (u32, u32),
    pub anchor: Anchor,
    pub bg_image: Option<String>,
    pub visual: WidgetStyle,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            size: (600, 400),
            anchor: Anchor::None,
            bg_image: None,
            visual: WidgetStyle::default_dark(),
        }
    }
}
