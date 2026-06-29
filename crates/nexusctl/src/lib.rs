mod command;

pub use command::{NexusCommand, SpaceSelector};

use nexus_api::{NEXUS_COMMUNICATION_SOCKET, NexusRequest, NexusResponse, NexusStream};

pub async fn send_command(command: NexusCommand) -> anyhow::Result<NexusResponse> {
    let mut stream = NexusStream::connect(NEXUS_COMMUNICATION_SOCKET).await?;

    stream.writable().await?;
    let _ = stream.try_write::<NexusRequest>(&command.into()).await?;

    stream.readable().await?;
    let response = stream
        .try_read::<NexusResponse>()
        .await?
        .unwrap_or(NexusResponse::Empty);

    let _ = stream.shutdown().await;

    Ok(response)
}
