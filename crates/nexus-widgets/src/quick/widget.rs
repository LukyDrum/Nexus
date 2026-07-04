use iced::{Task, widget::text};

use crate::{NexusWidget, settings::WidgetSettings, style::WidgetAppStyle};

#[derive(Clone)]
pub(super) struct QuickWidget {
    pub name: String,
    pub style: WidgetAppStyle,
}

#[derive(Clone, Debug)]
pub(super) enum QuickMessage {}

impl NexusWidget<QuickMessage> for QuickWidget {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn settings(&self) -> WidgetSettings {
        WidgetSettings::default()
    }

    fn update(&mut self, _message: QuickMessage) -> Task<QuickMessage> {
        Task::none()
    }

    fn view(&'_ self) -> impl Into<iced::Element<'_, QuickMessage>> {
        text("Hello there!")
    }

    fn style(&self) -> WidgetAppStyle {
        self.style.clone()
    }
}
