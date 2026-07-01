mod command;

pub use command::*;
pub use nexus_api;

use nexus_api::{NEXUS_COMMUNICATION_SOCKET, NexusRequest, NexusResponse, NexusStream};

pub async fn send_command(command: NexusCommand) -> anyhow::Result<NexusResponse> {
    let mut stream = NexusStream::connect(NEXUS_COMMUNICATION_SOCKET).await?;

    let response = send_command_through_stream(command, &mut stream).await;

    let _ = stream.shutdown().await;

    response
}

pub async fn send_multiple_commands(
    commands: impl Iterator<Item = NexusCommand>,
) -> anyhow::Result<Vec<NexusResponse>> {
    let mut stream = NexusStream::connect(NEXUS_COMMUNICATION_SOCKET).await?;

    let mut responses = Vec::new();
    for command in commands {
        let response = send_command_through_stream(command, &mut stream).await?;
        responses.push(response);
    }

    let _ = stream.shutdown().await;

    Ok(responses)
}

pub fn send_command_blocking(command: NexusCommand) -> anyhow::Result<NexusResponse> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(send_command(command))
}

pub fn send_multiple_commands_blocking(
    commands: impl Iterator<Item = NexusCommand>,
) -> anyhow::Result<Vec<NexusResponse>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    rt.block_on(send_multiple_commands(commands))
}

async fn send_command_through_stream(
    command: NexusCommand,
    stream: &mut NexusStream,
) -> anyhow::Result<NexusResponse> {
    let _ = stream.write::<NexusRequest>(&command.into()).await?;

    let response = stream
        .read::<NexusResponse>()
        .await?
        .unwrap_or(NexusResponse::Empty);

    Ok(response)
}
