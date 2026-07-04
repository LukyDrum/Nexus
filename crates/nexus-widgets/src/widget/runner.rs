use std::{fmt::Debug, marker::PhantomData};

use iced::widget::{Container, container};
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
        let theme = widget.style().theme();

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
        .style(|_, theme| iced::theme::Style {
            background_color: iced::Color::TRANSPARENT,
            text_color: theme.palette().text,
        })
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
        let style = self.widget.style();

        Container::new(element.map(LayerShellAppMessage::AppMessage))
            .padding(style.widget.padding())
            .style(move |_| container::Style {
                text_color: style
                    .widget
                    .color()
                    .or_else(|| Some(style.palette.text.into())),
                background: style
                    .widget
                    .background()
                    .or_else(|| Some(style.palette.background.into())),
                border: style.widget.border(),
                ..Default::default()
            })
            .into()
    }

    fn subscription(&self) -> Subscription<LayerShellAppMessage<Message>> {
        self.widget
            .subscription()
            .map(LayerShellAppMessage::AppMessage)
    }
}
