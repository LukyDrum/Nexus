mod command;

use clap::Parser;
use nexus_api::{NEXUS_COMMUNICATION_SOCKET, NexusMessage, NexusStream};

use crate::command::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let stream = NexusStream::<NexusMessage>::connect(NEXUS_COMMUNICATION_SOCKET).await?;

    stream.writable().await?;
    let command = cli.command;
    let json = stream.try_write(&command.into()).await?;
    dbg!(json);

    Ok(())
}
