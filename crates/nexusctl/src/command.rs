use nexus_api::{NexusMessage, WorkspaceNumber};

#[derive(Debug, clap::Subcommand)]
pub enum NexusCommand {
    #[command(subcommand)]
    Switch(SpaceSelector),
    Move {
        window: Window,
        target: String,
    },
    Clients,
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum SpaceSelector {
    Workspace { number: WorkspaceNumber },
    Group { name: String },
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Window {
    Active,
}

impl From<NexusCommand> for NexusMessage {
    fn from(command: NexusCommand) -> Self {
        match command {
            NexusCommand::Switch(SpaceSelector::Workspace { number }) => {
                NexusMessage::SwitchWorkspace(number)
            }
            NexusCommand::Switch(SpaceSelector::Group { name }) => NexusMessage::SwitchGroup(name),
            NexusCommand::Move {
                window: Window::Active,
                target,
            } => {
                if let Ok(number) = target.parse() {
                    NexusMessage::MoveActiveWindowToWorkspace(number)
                } else {
                    NexusMessage::MoveActiveWindowToNamedWorkspace(target)
                }
            }
            NexusCommand::Clients => NexusMessage::ListAllClients,
        }
    }
}
