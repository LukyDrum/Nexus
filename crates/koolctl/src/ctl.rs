use std::path::PathBuf;

use koolctl::ControlMessage;

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct KoolCtl {
    #[command(subcommand)]
    pub command: ControlCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum ControlCommand {
    Run { path: String },
    Close { path: String },
}

impl From<ControlCommand> for ControlMessage {
    fn from(value: ControlCommand) -> Self {
        match value {
            ControlCommand::Run { path } => {
                let path = PathBuf::from(path);
                let path = std::path::absolute(&path).unwrap_or(path);

                Self::Run { path }
            }
            ControlCommand::Close { path } => {
                let path = PathBuf::from(path);
                let path = std::path::absolute(&path).unwrap_or(path);

                Self::Close { path }
            }
        }
    }
}
