use std::{fmt::Debug, sync::Arc, time::Duration};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use iced::{
    Color, Element, Length, Subscription, Task,
    keyboard::{self, Key, key::Named},
    widget::{
        self, Column, Scrollable, Text, column,
        operation::{AbsoluteOffset, scroll_to},
        text_input,
    },
};
use nexus_widgets::{
    NexusWidget,
    settings::{Size, WidgetSettings},
};

use crate::desktop::{DesktopEntry, read_desktop_entries};

const ROW_HEIGHT: f32 = 40.0;

#[derive(Clone)]
pub struct NexusLauncher {
    input_id: widget::Id,
    input_content: String,
    fuzzy_matcher: Arc<SkimMatcherV2>,
    desktop_entries: Vec<DesktopEntry>,
    list_results: Vec<(String, usize)>,
    selected_result: usize,
    scroll_id: widget::Id,

    mode: Mode,
}

#[derive(Clone, Debug)]
pub enum NavigationDirection {
    Up,
    Down,
}

#[derive(Clone, Debug)]
pub enum LauncherMessage {
    Nothing,
    Exit,
    InputContentChanged(String),
    InputSubmit,
    InputFocus,
    ListNavigation(NavigationDirection),
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    AppRunner,
    NexusGroup,
    ActiveApp,
    Math,
    TerminalCommand,
    QuickAction,
}

impl Mode {
    fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "#" => Self::NexusGroup,
            "@" => Self::ActiveApp,
            "=" => Self::Math,
            ":" => Self::TerminalCommand,
            ">" => Self::QuickAction,
            _ => Self::AppRunner,
        }
    }

    fn as_symbol(&self) -> &str {
        match self {
            Mode::AppRunner => "",
            Mode::NexusGroup => "#",
            Mode::ActiveApp => "@",
            Mode::Math => "=",
            Mode::TerminalCommand => ":",
            Mode::QuickAction => ">",
        }
    }
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

    fn update(&mut self, message: LauncherMessage) -> Task<LauncherMessage> {
        match message {
            LauncherMessage::InputContentChanged(new_content) => {
                self.input_content = new_content;
                // Reset the index of the selected result
                self.selected_result = 0;

                // Update mode
                if let Some((symbol, _rest)) = self.input_content.split_at_checked(1) {
                    self.mode = Mode::from_symbol(symbol);
                } else {
                    self.mode = Mode::AppRunner;
                }

                // Mode specific update
                match self.mode {
                    Mode::AppRunner => {
                        self.list_results = self.search_results();
                    }
                    Mode::NexusGroup => todo!(),
                    Mode::ActiveApp => todo!(),
                    Mode::Math => self.math_update(),
                    Mode::QuickAction => todo!(),
                    _ => {}
                }

                Task::none()
            }
            LauncherMessage::InputSubmit => {
                self.on_submit();
                iced::exit()
            }
            LauncherMessage::InputFocus => widget::operation::focus(self.input_id.clone()),
            LauncherMessage::ListNavigation(direction) => {
                self.selected_result = match direction {
                    NavigationDirection::Up => self.selected_result.saturating_sub(1),
                    NavigationDirection::Down => {
                        (self.selected_result + 1).min(self.list_results.len() - 1)
                    }
                };

                let offset = self.selected_result.saturating_sub(3);
                scroll_to(
                    self.scroll_id.clone(),
                    AbsoluteOffset {
                        x: 0.0,
                        y: offset as f32 * ROW_HEIGHT,
                    },
                )
            }
            LauncherMessage::Exit => iced::exit(),
            LauncherMessage::Nothing => Task::none(),
        }
    }

    fn view(&'_ self) -> impl Into<Element<'_, LauncherMessage>> {
        let input = text_input("Command...", &self.input_content)
            .id(self.input_id.clone())
            .on_input(LauncherMessage::InputContentChanged)
            .on_submit(LauncherMessage::InputSubmit);

        let search_results = {
            let results = self
                .list_results
                .iter()
                .enumerate()
                .map(|(index, (name, _))| {
                    let text = Text::new(name).height(ROW_HEIGHT);
                    if self.selected_result == index {
                        text.color(Color::BLACK).into()
                    } else {
                        text.color(Color::from_rgb8(60, 60, 60)).into()
                    }
                });
            let column = Column::with_children(results);
            Scrollable::new(column)
                .id(self.scroll_id.clone())
                .width(Length::Fill)
        };

        column![input, search_results]
    }

    fn subscription(&self) -> Subscription<LauncherMessage> {
        keyboard::listen().map(|event| match event {
            keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowUp),
                ..
            } => LauncherMessage::ListNavigation(NavigationDirection::Up),
            keyboard::Event::KeyPressed {
                key: Key::Named(Named::ArrowDown),
                ..
            } => LauncherMessage::ListNavigation(NavigationDirection::Down),
            keyboard::Event::KeyPressed {
                key: Key::Named(Named::Escape),
                ..
            } => LauncherMessage::Exit,
            _ => LauncherMessage::Nothing,
        })
    }

    fn startup_task(&self) -> Task<LauncherMessage> {
        // Small hack to make the input focus from the start
        let future = async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            LauncherMessage::InputFocus
        };
        Task::future(future)
    }
}

impl NexusLauncher {
    pub fn new() -> Self {
        let mut launcher = NexusLauncher {
            input_id: widget::Id::unique(),
            input_content: String::new(),
            fuzzy_matcher: Arc::new(SkimMatcherV2::default()),
            desktop_entries: read_desktop_entries(),
            list_results: Vec::new(),
            selected_result: 0,
            scroll_id: widget::Id::unique(),
            mode: Mode::AppRunner,
        };
        // Init the search results
        launcher.list_results = launcher.search_results();

        launcher
    }

    fn search_results(&self) -> Vec<(String, usize)> {
        fn clean(string: &str) -> String {
            string
                .to_lowercase()
                .replace(|c: char| c.is_whitespace(), "")
        }

        let search = clean(&self.input_content);

        let mut scored_entries = self
            .desktop_entries
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| {
                let name = entry.name();
                self.fuzzy_matcher
                    .fuzzy_match(&clean(name), &search)
                    .map(|score| ((name, index), score))
            })
            .collect::<Vec<_>>();

        scored_entries.sort_by_key(|(_, score)| *score);

        scored_entries
            .into_iter()
            .map(|((name, index), _)| (name.to_string(), index))
            .collect()
    }

    fn math_update(&mut self) {
        let expr = self
            .input_content
            .trim_start_matches(Mode::Math.as_symbol());
        let evaluated = calc_this::calc_this(expr, &[])
            .map(|result| format!("= {result}"))
            .unwrap_or("= undef.".to_owned());

        self.list_results.clear();
        self.list_results.push((evaluated, 0));
    }

    fn on_submit(&self) {
        match self.mode {
            Mode::AppRunner => self.run_selected_app(),
            Mode::NexusGroup => todo!("Nexus group"),
            Mode::ActiveApp => todo!("Active app"),
            Mode::TerminalCommand => todo!("Terminal command"),
            Mode::QuickAction => todo!("Quick action"),
            Mode::Math => {}
        }
    }

    fn run_selected_app(&self) {
        let Some((_name, index)) = self.list_results.get(self.selected_result) else {
            return;
        };
        let Some(entry) = self.desktop_entries.get(*index) else {
            return;
        };

        #[expect(clippy::zombie_processes, reason = "We want to run it and forget it.")]
        entry.run().expect("Failed to run desktop entry!");
    }
}
