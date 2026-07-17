use std::{fmt::Debug, marker::PhantomData};

use iced::widget::Container;
use iced::{Element, Subscription, Task};

use crate::settings::Rendering;
use crate::style::WithStyle;
use crate::widget::KoolWidget;
use crate::widget::LayerShellAppMessage;

pub struct KoolWidgetRunner<Widget, Message>
where
    Widget: KoolWidget<Message>,
{
    widget: Widget,
    _phantom: PhantomData<Message>,
}

impl<Widget, Message> KoolWidgetRunner<Widget, Message>
where
    Widget: KoolWidget<Message> + Clone + 'static,
    Message: Clone + Debug + Send + 'static,
{
    pub fn run(widget: Widget) -> Result<(), iced_layershell::Error> {
        let name = widget.name();
        let settings = widget.settings();
        let theme = widget.style().theme();

        set_renderer(settings.rendering);

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

    fn view<'a>(&'a self) -> Element<'a, LayerShellAppMessage<Message>> {
        let element = self.widget.view().into();

        Container::new(element.map(LayerShellAppMessage::AppMessage))
            .with_style(self.widget.style().style_tree())
            .into()
    }

    fn subscription(&self) -> Subscription<LayerShellAppMessage<Message>> {
        self.widget
            .subscription()
            .map(LayerShellAppMessage::AppMessage)
    }
}

fn set_renderer(renderer: Rendering) {
    let renderer = match renderer {
        Rendering::Cpu => "tiny-skia",
        Rendering::Gpu => "wgpu",
    };

    // Trust me bro
    unsafe {
        std::env::set_var("ICED_BACKEND", renderer);
    }
}
