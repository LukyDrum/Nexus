use std::str::FromStr;

use nexus_api::{GroupName, NexusRequest, WindowSelector, WorkspaceNumber, WorkspaceSelector};

#[derive(Debug, clap::Subcommand)]
pub enum NexusCommand {
    #[command(subcommand)]
    Switch(SwitchTarget),

    Move {
        window: Window,
        target: String,
    },

    #[command(subcommand)]
    List(ListTarget),
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum SwitchTarget {
    Workspace { number: WorkspaceNumber },
    Group { name: GroupName },
    Window { window: Window },
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum ListTarget {
    Clients,
    Groups,
}

#[derive(Clone, Debug)]
pub enum Window {
    Active,
    Pid(u64),
}

impl From<NexusCommand> for NexusRequest {
    fn from(command: NexusCommand) -> Self {
        match command {
            NexusCommand::Switch(SwitchTarget::Workspace { number }) => {
                NexusRequest::SwitchWorkspace(WorkspaceSelector::InGroup(number))
            }
            NexusCommand::Switch(SwitchTarget::Group { name }) => NexusRequest::SwitchGroup(name),
            NexusCommand::Switch(SwitchTarget::Window { window }) => {
                NexusRequest::FocusWindow(window.into())
            }
            NexusCommand::Move { window, target } => {
                let target = if let Ok(number) = target.parse() {
                    WorkspaceSelector::InGroup(number)
                } else {
                    WorkspaceSelector::Named(target)
                };

                let window = match window {
                    Window::Active => WindowSelector::Focused,
                    Window::Pid(pid) => WindowSelector::Pid(pid),
                };

                NexusRequest::MoveWindowToWorkspace(window, target)
            }
            NexusCommand::List(ListTarget::Clients) => NexusRequest::ListAllClients,
            NexusCommand::List(ListTarget::Groups) => NexusRequest::ListGroups,
        }
    }
}

impl FromStr for Window {
    type Err = clap::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.eq_ignore_ascii_case("active") {
            return Ok(Self::Active);
        }

        if let Ok(pid) = string.parse() {
            return Ok(Self::Pid(pid));
        }

        Err(clap::Error::new(clap::error::ErrorKind::InvalidValue))
    }
}

impl From<Window> for WindowSelector {
    fn from(value: Window) -> Self {
        match value {
            Window::Active => Self::Focused,
            Window::Pid(pid) => Self::Pid(pid),
        }
    }
}
