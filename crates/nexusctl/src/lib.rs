mod command;

pub use command::*;
pub use nexus_api;

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

pub fn send_command_blocking(command: NexusCommand) -> anyhow::Result<NexusResponse> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(send_command(command))
}
