use std::{process::Command, time::Duration};

use serde::{Deserialize, Serialize};

use crate::{
    KoolWidget,
    element::{BuildContext, KoolBuilder, KoolElement},
    language::{Value, Variables},
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
    pub commands: Vec<RepeatingCommand>,
}

#[derive(Clone, Debug)]
pub enum ElementalMessage {
    Empty,
    RepeatingCommandTick(usize),
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
            ElementalMessage::RepeatingCommandTick(index) => {
                let command = &self.commands[index];
                let output_value = command.run_or_null();
                self.variables.set(&command.variable, output_value);

                iced::Task::none()
            }
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

    fn startup_task(&self) -> iced::Task<ElementalMessage> {
        iced::Task::batch(self.commands.iter().enumerate().map(|(index, _command)| {
            iced::Task::done(ElementalMessage::RepeatingCommandTick(index))
        }))
    }

    fn subscription(&self) -> iced::Subscription<ElementalMessage> {
        iced::Subscription::batch(self.commands.iter().enumerate().filter_map(
            |(index, command)| {
                let period = command.period?;
                Some(
                    iced::time::every(period)
                        .with(index)
                        .map(|(index, _instant)| ElementalMessage::RepeatingCommandTick(index)),
                )
            },
        ))
    }
}

#[derive(Clone, Debug)]
pub struct RepeatingCommand {
    pub period: Option<Duration>,
    pub variable: String,
    pub command: String,
}

impl RepeatingCommand {
    pub fn run_or_null(&self) -> Value {
        let output = Command::new("bash").arg("-c").arg(&self.command).output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8(output.stdout).map_or(Value::Null, Value::String)
                } else {
                    String::from_utf8(output.stderr).map_or(Value::Null, Value::String)
                }
            }
            Err(_) => Value::Null,
        }
    }
}
