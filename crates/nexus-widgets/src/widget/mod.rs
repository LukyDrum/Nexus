mod message;
mod runner;
pub mod settings;

use iced::Element;

pub(crate) use message::LayerShellAppMessage;

pub use runner::NexusWidgetRunner;

/// This trait should be implemented for iced apps that you want to run with [`NexusWidgetRunner`].
pub trait NexusWidget<Message> {
    fn name(&self) -> String;

    fn update(&mut self, message: Message);

    fn view(&'_ self) -> impl Into<Element<'_, Message>>;
}
