use std::{fmt::Debug, process::Command, sync::Arc, time::Duration};

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
use nexusctl::{
    ListTarget, NexusCommand, SwitchTarget,
    nexus_api::{GroupName, NexusResponse},
    send_command_blocking,
};

use crate::desktop::{DesktopEntry, read_desktop_entries};

const ROW_HEIGHT: f32 = 40.0;

#[derive(Clone)]
pub struct NexusLauncher {
    input_id: widget::Id,
    input_content: String,
    scroll_id: widget::Id,

    mode: Mode,

    fuzzy_matcher: Arc<SkimMatcherV2>,

    /// Index into `list_results`
    selected_result: usize,
    /// Pairs of the text to show and an index into a vec of actual values based on the current mode.
    search_results: Vec<(String, usize)>,

    desktop_entries: Vec<DesktopEntry>,
    current_groups: Vec<GroupName>,
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

    fn all_symbols() -> &'static [char] {
        &['#', '@', '=', ':', '>']
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

                self.search_results = self.make_search_results();

                // Mode specific update
                match self.mode {
                    Mode::Math => self.math_update(),
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
                        (self.selected_result + 1).min(self.search_results.len() - 1)
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
                .search_results
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
        let groups_response =
            nexusctl::send_command_blocking(NexusCommand::List(ListTarget::Groups));
        let groups = match groups_response {
            Ok(NexusResponse::Groups(groups)) => groups,
            _ => Vec::new(),
        };

        let mut launcher = NexusLauncher {
            input_id: widget::Id::unique(),
            input_content: String::new(),
            fuzzy_matcher: Arc::new(SkimMatcherV2::default()),
            desktop_entries: read_desktop_entries(),
            search_results: Vec::new(),
            selected_result: 0,
            scroll_id: widget::Id::unique(),
            current_groups: groups,
            mode: Mode::AppRunner,
        };
        // Init the search results
        launcher.search_results = launcher.make_search_results();

        launcher
    }

    fn clean_str(string: &str) -> String {
        string
            .to_lowercase()
            .trim_start_matches(|c| Mode::all_symbols().contains(&c))
            .replace(|c: char| c.is_whitespace(), "")
    }

    fn make_search_results(&self) -> Vec<(String, usize)> {
        let search = Self::clean_str(&self.input_content);

        let items: Vec<_> = match self.mode {
            Mode::AppRunner => self
                .desktop_entries
                .iter()
                .map(DesktopEntry::name)
                .collect(),
            Mode::NexusGroup => self
                .current_groups
                .iter()
                .map(GroupName::display_name)
                .collect(),
            Mode::ActiveApp => todo!(),
            Mode::QuickAction => todo!(),
            Mode::Math | Mode::TerminalCommand => return Vec::new(),
        };

        self.match_search_results(&search, items.into_iter())
    }

    fn match_search_results<'a>(
        &'a self,
        search: &str,
        items: impl Iterator<Item = &'a str>,
    ) -> Vec<(String, usize)> {
        let mut scored_items = items
            .enumerate()
            .filter_map(|(index, item)| {
                self.fuzzy_matcher
                    .fuzzy_match(&Self::clean_str(item), search)
                    .map(|score| ((item, index), score))
            })
            .collect::<Vec<_>>();

        scored_items.sort_by_key(|(_, score)| *score);

        scored_items
            .into_iter()
            .map(|((item, index), _)| (item.to_string(), index))
            .collect()
    }

    fn math_update(&mut self) {
        let expr = self
            .input_content
            .trim_start_matches(Mode::Math.as_symbol());
        let evaluated = calc_this::calc_this(expr, &[])
            .map(|result| format!("= {result}"))
            .unwrap_or("= undef.".to_owned());

        self.search_results.clear();
        self.search_results.push((evaluated, 0));
    }

    fn on_submit(&self) {
        match self.mode {
            Mode::AppRunner => self.run_selected_app(),
            Mode::NexusGroup => self.switch_to_selected_group(),
            Mode::ActiveApp => todo!("Active app"),
            Mode::TerminalCommand => todo!("Terminal command"),
            Mode::QuickAction => todo!("Quick action"),
            Mode::Math => self.result_to_clipboard(),
        }
    }

    /* On submit handlers */

    fn run_selected_app(&self) {
        let Some((_name, index)) = self.search_results.get(self.selected_result) else {
            return;
        };
        let Some(entry) = self.desktop_entries.get(*index) else {
            return;
        };

        #[expect(clippy::zombie_processes, reason = "We want to run it and forget it.")]
        entry.run().expect("Failed to run desktop entry!");
    }

    fn switch_to_selected_group(&self) {
        let group_name = self
            .search_results
            .get(self.selected_result)
            .and_then(|(_display_name, index)| self.current_groups.get(*index))
            .map_or_else(
                || {
                    GroupName(
                        self.input_content
                            .trim_start_matches(Mode::NexusGroup.as_symbol())
                            .trim()
                            .to_owned(),
                    )
                },
                GroupName::clone,
            );

        let _ = send_command_blocking(NexusCommand::Switch(SwitchTarget::Group {
            name: group_name.clone(),
        }));
    }

    fn result_to_clipboard(&self) {
        let Some((value, _)) = self.search_results.first() else {
            return;
        };
        let value = value.trim_start_matches(Mode::Math.as_symbol()).trim();

        let _ = Command::new("wl-copy").arg(value).spawn();
    }
}
