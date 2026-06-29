use nexus_api::{NexusRequest, WorkspaceNumber};

#[derive(Debug, clap::Subcommand)]
pub enum NexusCommand {
    #[command(subcommand)]
    Switch(SpaceSelector),

    Move {
        window: Window,
        target: String,
    },

    #[command(subcommand)]
    List(ListTarget),
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum SpaceSelector {
    Workspace { number: WorkspaceNumber },
    Group { name: String },
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum ListTarget {
    Clients,
    Groups,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Window {
    Active,
}

impl From<NexusCommand> for NexusRequest {
    fn from(command: NexusCommand) -> Self {
        match command {
            NexusCommand::Switch(SpaceSelector::Workspace { number }) => {
                NexusRequest::SwitchWorkspace(number)
            }
            NexusCommand::Switch(SpaceSelector::Group { name }) => NexusRequest::SwitchGroup(name),
            NexusCommand::Move {
                window: Window::Active,
                target,
            } => {
                if let Ok(number) = target.parse() {
                    NexusRequest::MoveActiveWindowToWorkspace(number)
                } else {
                    NexusRequest::MoveActiveWindowToNamedWorkspace(target)
                }
            }
            NexusCommand::List(ListTarget::Clients) => NexusRequest::ListAllClients,
            NexusCommand::List(ListTarget::Groups) => NexusRequest::ListGroups,
        }
    }
}
