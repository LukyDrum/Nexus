use serde::{Deserialize, Serialize};

use crate::{settings::WidgetSettings, style::WidgetStyle};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElementalConfig {
    /// Name of the widget.
    #[serde(default = "default_name")]
    pub name: String,
    /// Path to the Kool source code.
    pub kool: String,
    /// Settings for this widget.
    #[serde(default, flatten)]
    pub settings: WidgetSettings,
    /// The style of this widget.
    #[serde(default)]
    pub visual: WidgetStyle,
}

fn default_name() -> String {
    "Kool widget".to_owned()
}
