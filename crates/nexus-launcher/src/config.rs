use nexus_widgets::{settings::Anchor, style::WidgetAppStyle};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LauncherConfig {
    pub style: WidgetAppStyle,
    pub size: (u32, u32),
    pub anchor: Anchor,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            style: WidgetAppStyle::default_dark(),
            size: (600, 400),
            anchor: Anchor::None,
        }
    }
}
