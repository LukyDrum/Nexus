mod message;
mod runner;

use iced::Element;

pub use message::LayerShellAppMessage;
pub use runner::NexusWidgetRunner;

pub trait NexusWidget<Message> {
    fn name(&self) -> String;

    fn update(&mut self, message: Message);

    fn view(&'_ self) -> impl Into<Element<'_, Message>>;
}
