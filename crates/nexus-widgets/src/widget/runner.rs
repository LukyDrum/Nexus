use std::{fmt::Debug, marker::PhantomData};

use iced::Element;

use crate::widget::LayerShellAppMessage;
use crate::{NexusWidget, settings::WidgetSettings};

pub struct NexusWidgetRunner<Widget, Message>
where
    Widget: NexusWidget<Message>,
{
    widget: Widget,
    _phantom: PhantomData<Message>,
}

impl<Widget, Message> NexusWidgetRunner<Widget, Message>
where
    Widget: NexusWidget<Message> + Clone + 'static,
    Message: Clone + Debug + Send + 'static,
{
    pub fn run(widget: Widget, settings: WidgetSettings) -> Result<(), iced_layershell::Error> {
        let name = widget.name();

        iced_layershell::application(
            move || Self {
                widget: widget.clone(),
                _phantom: PhantomData,
            },
            move || name.clone(),
            Self::update,
            Self::view,
        )
        .settings(settings.into())
        .run()
    }

    fn update(&mut self, message: LayerShellAppMessage<Message>) {
        if let LayerShellAppMessage::AppMessage(message) = message {
            self.widget.update(message);
        }
    }

    fn view(&'_ self) -> Element<'_, LayerShellAppMessage<Message>> {
        let element = self.widget.view().into();
        element.map(|message| LayerShellAppMessage::AppMessage(message))
    }
}
