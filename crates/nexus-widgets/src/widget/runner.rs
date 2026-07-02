use std::{fmt::Debug, marker::PhantomData};

use iced::{Element, Subscription, Task};

use crate::NexusWidget;
use crate::widget::LayerShellAppMessage;

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
    pub fn run(widget: Widget) -> Result<(), iced_layershell::Error> {
        let name = widget.name();
        let settings = widget.settings();
        let theme = widget.theme();

        iced_layershell::application(
            move || {
                let task = widget.startup_task().map(LayerShellAppMessage::AppMessage);
                (
                    Self {
                        widget: widget.clone(),
                        _phantom: PhantomData,
                    },
                    task,
                )
            },
            move || name.clone(),
            Self::update,
            Self::view,
        )
        .settings(settings.into())
        .subscription(Self::subscription)
        .theme(theme)
        .run()
    }

    fn update(
        &mut self,
        message: LayerShellAppMessage<Message>,
    ) -> Task<LayerShellAppMessage<Message>> {
        if let LayerShellAppMessage::AppMessage(message) = message {
            self.widget
                .update(message)
                .map(LayerShellAppMessage::AppMessage)
        } else {
            Task::none()
        }
    }

    fn view(&'_ self) -> Element<'_, LayerShellAppMessage<Message>> {
        let element = self.widget.view().into();
        element.map(LayerShellAppMessage::AppMessage)
    }

    fn subscription(&self) -> Subscription<LayerShellAppMessage<Message>> {
        self.widget
            .subscription()
            .map(LayerShellAppMessage::AppMessage)
    }
}
