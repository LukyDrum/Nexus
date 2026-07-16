use serde::{Deserialize, Serialize};

use crate::{
    KoolWidget,
    element::{BuildContext, KoolBuilder, KoolElement, environment::Variables},
    settings::WidgetSettings,
    style::WidgetStyle,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElementalConfig {
    /// Name of the widget.
    #[serde(default = "default_name")]
    pub name: String,
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

#[derive(Clone, Debug)]
pub struct ElementalWidget {
    pub name: String,
    pub settings: WidgetSettings,
    pub style: WidgetStyle,
    pub root: KoolElement,
    pub variables: Variables,
}

#[derive(Clone, Debug)]
pub enum ElementalMessage {
    Empty,
}

impl KoolWidget<ElementalMessage> for ElementalWidget {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn settings(&self) -> WidgetSettings {
        self.settings.clone()
    }

    fn update(&mut self, message: ElementalMessage) -> iced::Task<ElementalMessage> {
        match message {
            ElementalMessage::Empty => iced::Task::none(),
        }
    }

    fn view(&'_ self) -> impl Into<iced::Element<'_, ElementalMessage>> {
        self.root.build(BuildContext {
            style: self.style.style_tree(),
            variables: &self.variables,
        })
    }

    fn style(&self) -> &crate::style::WidgetStyle {
        &self.style
    }
}
