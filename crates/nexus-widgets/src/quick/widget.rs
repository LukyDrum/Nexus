use iced::{Task, widget::text};

use crate::{NexusWidget, config::style::WidgetStyle, settings::WidgetSettings};

#[derive(Clone)]
pub(super) struct QuickWidget {
    pub name: String,
    pub style: WidgetStyle,
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

    fn theme(&self) -> iced::Theme {
        self.style.main_theme()
    }
}
