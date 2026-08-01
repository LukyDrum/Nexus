use std::{process::Command, rc::Rc, time::Duration};

use crate::{
    Element, KoolWidget,
    element::{BuildContext, KoolElement, kool},
    language::{Environment, Function, Value},
    settings::WidgetSettings,
    style::WidgetStyle,
};

const VIEW_FUNCTION_NAME: &str = "view";

#[derive(Clone, Debug)]
pub struct ElementalWidget {
    name: String,
    settings: WidgetSettings,
    style: WidgetStyle,

    root: KoolElement,
    runtime: Environment,
    view_function: Rc<Function>,
    commands: Vec<RepeatingCommand>,
}

#[derive(Clone, Debug)]
pub enum ElementalMessage {
    Empty,
    RepeatingCommandTick(usize),
}

impl ElementalWidget {
    pub fn new(
        name: String,
        settings: WidgetSettings,
        style: WidgetStyle,
        mut runtime: Environment,
    ) -> Self {
        let view_function = runtime
            .get_function(VIEW_FUNCTION_NAME)
            .expect("No view function found");
        let root = Self::get_root(&view_function, &mut runtime);

        Self {
            name,
            settings,
            style,

            root,
            runtime,
            view_function,
            commands: Vec::new(),
        }
    }

    fn get_root(view_function: &Function, environment: &mut Environment) -> KoolElement {
        match view_function.call_with_default_args(environment) {
            Ok(Value::Element(element)) => *element,
            Ok(value) => KoolElement::Error(kool::Error::new(format!(
                "view function returned non-element value: {value}"
            ))),
            Err(error) => KoolElement::Error(kool::Error::new(format!(
                "view function call ended with error: {error:?}"
            ))),
        }
    }
}

impl KoolWidget<ElementalMessage> for ElementalWidget {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn settings(&self) -> WidgetSettings {
        self.settings.clone()
    }

    fn update(&mut self, message: ElementalMessage) -> iced::Task<ElementalMessage> {
        self.root = Self::get_root(&self.view_function, &mut self.runtime);

        match message {
            ElementalMessage::Empty => iced::Task::none(),
            ElementalMessage::RepeatingCommandTick(index) => {
                let command = &self.commands[index];
                let output_value = command.run_or_null();
                // self.build_context
                //     .variables
                //     .set(&command.variable, output_value);

                iced::Task::none()
            }
        }
    }

    fn view<'a>(&'a self) -> impl Into<iced::Element<'a, ElementalMessage>> {
        let context = BuildContext {
            style: self.style.style_tree(),
        };

        self.root.build(&context)
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
