mod command;

pub use command::{NexusCommand, SpaceSelector};

use nexus_api::{NEXUS_COMMUNICATION_SOCKET, NexusMessage, NexusStream};

pub async fn send_command(command: NexusCommand) -> anyhow::Result<()> {
    let stream = NexusStream::<NexusMessage>::connect(NEXUS_COMMUNICATION_SOCKET).await?;

    stream.writable().await?;
    let _ = stream.try_write(&command.into()).await?;

    Ok(())
}
