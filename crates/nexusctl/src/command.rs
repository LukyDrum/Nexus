use nexus_api::{NexusMessage, WorkspaceNumber};

#[derive(Debug, clap::Subcommand)]
pub enum NexusCommand {
    #[command(subcommand)]
    Switch(Selector),
}

#[derive(Clone, Debug, clap::Subcommand)]
pub enum Selector {
    Workspace { number: WorkspaceNumber },
    Group { name: String },
    Hub,
}

impl From<NexusCommand> for NexusMessage {
    fn from(command: NexusCommand) -> Self {
        match command {
            NexusCommand::Switch(Selector::Workspace { number }) => {
                NexusMessage::SwitchWorkspace(number)
            }
            NexusCommand::Switch(Selector::Group { name }) => NexusMessage::SwitchGroup(name),
            NexusCommand::Switch(Selector::Hub) => NexusMessage::SwitchToHub,
        }
    }
}
