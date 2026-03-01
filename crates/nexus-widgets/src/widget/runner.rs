use std::{fmt::Debug, marker::PhantomData};

use iced::Element;
use iced_layershell::{
    Settings,
    reexport::{Anchor, Layer},
    settings::{LayerShellSettings, StartMode},
};

use crate::{LayerShellAppMessage, NexusWidget};

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
        let settings = Settings {
            layer_settings: LayerShellSettings {
                start_mode: StartMode::Active,
                layer: Layer::Top,
                anchor: Anchor::all(),
                size: Some((600, 400)),
                ..Default::default()
            },
            ..Default::default()
        };

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
        .settings(settings)
        .run()
    }

    fn update(&mut self, message: LayerShellAppMessage<Message>) {
        match message {
            LayerShellAppMessage::AppMessage(message) => self.widget.update(message),
            _ => {}
        }
    }

    fn view(&'_ self) -> Element<'_, LayerShellAppMessage<Message>> {
        let element = self.widget.view().into();
        element.map(|message| LayerShellAppMessage::AppMessage(message))
    }
}
