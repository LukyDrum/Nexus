mod message;
mod runner;
pub mod settings;

use iced::{Element, Subscription, Task};

pub(crate) use message::LayerShellAppMessage;

pub use runner::NexusWidgetRunner;

use crate::settings::WidgetSettings;

/// This trait should be implemented for iced apps that you want to run with [`NexusWidgetRunner`].
pub trait NexusWidget<Message> {
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
}
