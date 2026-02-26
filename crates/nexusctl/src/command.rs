use nexus_api::{NexusMessage, WorkspaceNumber};

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    #[command(subcommand)]
    Switch(Selector),
}

#[derive(Clone, Debug, clap::Subcommand)]
pub(crate) enum Selector {
    Workspace { number: WorkspaceNumber },
    Group { name: String },
    Hub,
}

impl From<Command> for NexusMessage {
    fn from(command: Command) -> Self {
        match command {
            Command::Switch(Selector::Workspace { number }) => {
                NexusMessage::SwitchWorkspace(number)
            }
            Command::Switch(Selector::Group { name }) => NexusMessage::SwitchGroup(name),
            Command::Switch(Selector::Hub) => NexusMessage::SwitchToHub,
        }
    }
}
