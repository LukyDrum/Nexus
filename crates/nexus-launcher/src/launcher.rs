use std::{fmt::Debug, sync::Arc};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use iced::{
    Element,
    widget::{Column, Scrollable, Text, column, text_input},
};
use nexus_widgets::{
    NexusWidget,
    settings::{Size, WidgetSettings},
};

use crate::desktop::{DesktopEntry, read_desktop_entries};

#[derive(Clone)]
pub struct NexusLauncher {
    input_content: String,
    fuzzy_matcher: Arc<SkimMatcherV2>,
    desktop_entries: Vec<DesktopEntry>,
}

#[derive(Clone, Debug)]
pub enum LauncherMessage {
    InputContentChanged(String),
    InputSubmit,
}

impl NexusWidget<LauncherMessage> for NexusLauncher {
    fn name(&self) -> String {
        "Nexus Menu App".to_owned()
    }

    fn settings(&self) -> WidgetSettings {
        WidgetSettings {
            size: Size::Size(500, 300),
            ..Default::default()
        }
    }

    fn update(&mut self, message: LauncherMessage) {
        match message {
            LauncherMessage::InputContentChanged(new_content) => self.input_content = new_content,
            LauncherMessage::InputSubmit => todo!("Process submitted input"),
        }
    }

    fn view(&'_ self) -> impl Into<Element<'_, LauncherMessage>> {
        let input = text_input("Command...", &self.input_content)
            .on_input(LauncherMessage::InputContentChanged)
            .on_submit(LauncherMessage::InputSubmit);

        let search_results = {
            let results = self
                .search_results()
                .into_iter()
                .map(|result| Text::new(result).into());
            let column = Column::with_children(results);
            Scrollable::new(column)
        };

        column![input, search_results]
    }
}

impl NexusLauncher {
    pub fn new() -> Self {
        NexusLauncher {
            input_content: String::new(),
            fuzzy_matcher: Arc::new(SkimMatcherV2::default()),
            desktop_entries: read_desktop_entries(),
        }
    }

    fn search_results(&self) -> Vec<&str> {
        fn clean(string: &str) -> String {
            string
                .to_lowercase()
                .replace(|c: char| c.is_whitespace(), "")
        }

        let search = clean(&self.input_content);

        let mut scored_entries = self
            .desktop_entries
            .iter()
            .filter_map(|entry| {
                let name = entry.name();
                self.fuzzy_matcher
                    .fuzzy_match(&clean(name), &search)
                    .map(|score| (name, score))
            })
            .collect::<Vec<_>>();

        scored_entries.sort_by_key(|(_, score)| *score);

        scored_entries.into_iter().map(|(name, _)| name).collect()
    }
}
