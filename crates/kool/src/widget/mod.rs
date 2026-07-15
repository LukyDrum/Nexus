mod message;
mod runner;
pub mod settings;

use iced::{Element, Subscription, Task};

pub(crate) use message::LayerShellAppMessage;
pub use runner::KoolWidgetRunner;

use crate::{style::WidgetStyle, widget::settings::WidgetSettings};

pub trait KoolWidget<Message> {
    fn name(&self) -> String;

    fn settings(&self) -> WidgetSettings;

    fn update(&mut self, message: Message) -> Task<Message>;

    fn view(&'_ self) -> impl Into<Element<'_, Message>>;

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn startup_task(&self) -> Task<Message> {
        Task::none()
    }

    fn style(&self) -> &WidgetStyle;
}
