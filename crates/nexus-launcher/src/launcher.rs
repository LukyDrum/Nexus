use std::{fmt::Debug, process::Command, sync::Arc, time::Duration};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use iced::{
    Element, Length, Subscription, Task,
    keyboard::{self, Key, key::Named},
    widget::{
        self, Column, Image, Scrollable, Space, Text, column, container,
        operation::{AbsoluteOffset, scroll_to},
        stack, text_input,
    },
};
use nexus_widgets::{
    NexusWidget,
    settings::{Size, WidgetSettings},
    style::{WidgetStyle, WithStyle, WithStyleId},
};
use nexusctl::{
    ListTarget, NexusCommand, SwitchTarget, Window,
    nexus_api::{ActiveClient, GroupName, NexusResponse},
    send_command_blocking, send_multiple_commands_blocking,
};

use crate::{
    config::LauncherConfig,
    desktop::{DesktopEntry, read_desktop_entries},
    history::History,
};

const SELECTED_ID: &str = "selected";
const NOT_SELECTED_ID: &str = "unselected";
const CMD_PATTERN: &str = "{CMD}";

#[derive(Clone)]
pub struct NexusLauncher {
    config: LauncherConfig,
    history: History,

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
    active_apps: Vec<ActiveClient>,
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
            size: Size::Size(self.config.size.0, self.config.size.1),
            anchor: self.config.anchor,
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

                Task::none()
            }
            LauncherMessage::InputSubmit => {
                self.on_submit();
                iced::exit()
            }
            LauncherMessage::InputFocus => widget::operation::focus(self.input_id.clone()),
            LauncherMessage::ListNavigation(direction) => {
                // No point in navigating no items
                if self.search_results.is_empty() {
                    return Task::none();
                }

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
                        y: offset as f32 * self.get_row_height(),
                    },
                )
            }
            LauncherMessage::Exit => iced::exit(),
            LauncherMessage::Nothing => Task::none(),
        }
    }

    fn view(&'_ self) -> impl Into<Element<'_, LauncherMessage>> {
        let style_tree = self.config.visual.style_tree();

        let input = text_input("Command...", &self.input_content)
            .id(self.input_id.clone())
            .with_style(style_tree.clone())
            .on_input(LauncherMessage::InputContentChanged)
            .on_submit(LauncherMessage::InputSubmit);

        let search_results = {
            let results = self
                .search_results
                .iter()
                .enumerate()
                .map(|(index, (name, _))| {
                    let text = Text::new(name);
                    let text = if self.selected_result == index {
                        text.with_style_id(SELECTED_ID)
                    } else {
                        text.with_style_id(NOT_SELECTED_ID)
                    };

                    text.with_style(style_tree.clone()).element().into()
                });

            let column = Column::with_children(results);
            Scrollable::new(column)
                .id(self.scroll_id.clone())
                .with_style(style_tree.clone())
                .width(Length::Fill)
        };

        let gap = Space::new().height(20);

        let column = column![input, gap, search_results];
        let mut stack = stack!(column);

        if let Some(bg_image) = &self.config.bg_image {
            let image = Image::new(&bg_image).with_style(style_tree);
            let image_container = container(image)
                .center_x(self.config.size.0)
                .center_y(self.config.size.1);

            stack = stack.push_under(image_container);
        }

        stack
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

    fn style(&self) -> &WidgetStyle {
        &self.config.visual
    }
}

impl NexusLauncher {
    pub fn new(config: LauncherConfig, history: History) -> Self {
        let mut current_groups = Vec::new();
        let mut active_apps = Vec::new();

        let send_commands_result = send_multiple_commands_blocking(
            [
                NexusCommand::List(ListTarget::Groups),
                NexusCommand::List(ListTarget::Clients),
            ]
            .into_iter(),
        );
        if let Ok(responses) = send_commands_result {
            for response in responses {
                match response {
                    NexusResponse::Clients(active_clients) => active_apps = active_clients,
                    NexusResponse::Groups(group_names) => current_groups = group_names,
                    _ => {}
                }
            }
        }

        let mut launcher = NexusLauncher {
            config,
            history,
            input_id: widget::Id::unique(),
            input_content: String::new(),
            fuzzy_matcher: Arc::new(SkimMatcherV2::default()),
            desktop_entries: read_desktop_entries(),
            search_results: Vec::new(),
            selected_result: 0,
            scroll_id: widget::Id::unique(),
            current_groups,
            active_apps,
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
                .map(|entry| entry.name().to_owned())
                .collect(),
            Mode::NexusGroup => self
                .current_groups
                .iter()
                .map(|group_name| group_name.display_name().to_owned())
                .collect(),
            Mode::ActiveApp => self
                .active_apps
                .iter()
                .map(|app| format!("{} - {}", app.name, app.title))
                .collect(),
            Mode::Math => return vec![(self.eval_math_expr(), 0)],
            Mode::QuickAction => self.config.actions.keys().cloned().collect(),
            Mode::TerminalCommand => {
                if let Some(terminal_cmd) = &self.config.terminal_cmd {
                    return vec![(
                        terminal_cmd.replace(
                            CMD_PATTERN,
                            &self
                                .input_content
                                .trim_start_matches(Mode::TerminalCommand.as_symbol()),
                        ),
                        0,
                    )];
                } else {
                    return vec![("Terminal command not configured!".to_owned(), 0)];
                }
            }
        };

        self.match_search_results(&search, items.into_iter())
    }

    fn match_search_results(
        &self,
        search: &str,
        items: impl Iterator<Item = String>,
    ) -> Vec<(String, usize)> {
        let mut scored_items = items
            .enumerate()
            .filter_map(|(index, item)| {
                let occurrences = self.history.get(&item);
                self.fuzzy_matcher
                    .fuzzy_match(&Self::clean_str(&item), search)
                    .map(|score| ((item, index), (score, occurrences)))
            })
            .collect::<Vec<_>>();

        scored_items.sort_by_key(|(_, rank)| *rank);

        scored_items
            .into_iter()
            .rev()
            .map(|((item, index), _)| (item, index))
            .collect()
    }

    fn eval_math_expr(&self) -> String {
        let expr = self
            .input_content
            .trim_start_matches(Mode::Math.as_symbol());
        calc_this::calc_this(expr, &[])
            .map(|result| format!("= {result}"))
            .unwrap_or("= undef.".to_owned())
    }

    fn on_submit(&self) {
        match self.mode {
            Mode::AppRunner => self.run_selected_app(),
            Mode::NexusGroup => self.switch_to_selected_group(),
            Mode::ActiveApp => self.switch_to_selected_app(),
            Mode::TerminalCommand => self.run_terminal_command(),
            Mode::QuickAction => self.run_quick_action(),
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

        let mut history = self.history.clone();
        history.add_record(entry.name().to_owned());
        let _ = history.write();
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

    fn switch_to_selected_app(&self) {
        let Some(app) = self
            .search_results
            .get(self.selected_result)
            .and_then(|(_title, index)| self.active_apps.get(*index))
        else {
            return;
        };

        let _ = send_command_blocking(NexusCommand::Switch(SwitchTarget::Window {
            window: Window::Pid(app.pid),
        }));
    }

    fn result_to_clipboard(&self) {
        let Some((value, _)) = self.search_results.first() else {
            return;
        };
        let value = value.trim_start_matches(Mode::Math.as_symbol()).trim();

        let _ = Command::new("wl-copy").arg(value).spawn();
    }

    fn run_terminal_command(&self) {
        let Some(terminal_cmd) = &self.config.terminal_cmd else {
            return;
        };

        let cmd = self
            .input_content
            .trim_start_matches(Mode::TerminalCommand.as_symbol())
            .trim();

        let (program, args) = split_command(&terminal_cmd);
        let args = args
            .into_iter()
            .map(|arg| arg.trim_matches(is_quote).replace(CMD_PATTERN, cmd));

        let _ = Command::new(program)
            .args(args)
            .spawn()
            .expect("Failed to run terminal command.");
    }

    fn run_quick_action(&self) {
        let Some((action, _)) = self.search_results.get(self.selected_result) else {
            return;
        };
        let Some(cmd) = self.config.actions.get(action) else {
            return;
        };

        let (program, args) = split_command(cmd);

        let _ = Command::new(program)
            .args(args)
            .spawn()
            .expect("Failed to run action.");
    }

    fn get_row_height(&self) -> f32 {
        let text_style = self.config.visual.style_tree().get(Text::BASE_KEY);

        text_style
            .line_height()
            .to_absolute(text_style.text_size().into())
            .0
    }
}

fn split_command(command: &str) -> (&str, Vec<&str>) {
    let mut start = 0;
    let mut end = 0;
    let mut quoted = false;
    let mut parts = Vec::new();

    for c in command.chars() {
        if c == ' ' && !quoted {
            parts.push(&command[start..end]);
            start = end + 1;
        } else if is_quote(c) {
            quoted = !quoted;
        }

        end += 1;
    }
    parts.push(&command[start..end]);

    (parts.remove(0), parts)
}

fn is_quote(c: char) -> bool {
    c == '"' || c == '\''
}
