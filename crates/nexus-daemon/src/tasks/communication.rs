use std::sync::Arc;

use anyhow::bail;
use nexus_api::{NEXUS_COMMUNICATION_SOCKET, NexusListener, NexusMessage};
use tokio::sync::RwLock;

use crate::core::Overseer;

/// A task that is responsible for communication with the rest of the Nexus ecosystem.
/// It accepts connections to a communication unix socket and performs actions based on the incoming messages.
pub(crate) async fn communication_task(overseer: Arc<RwLock<Overseer>>) -> anyhow::Result<()> {
    let listener = NexusListener::bind(NEXUS_COMMUNICATION_SOCKET)?;

    loop {
        match listener.accept::<NexusMessage>().await {
            Ok(mut stream) => {
                stream.readable().await?;

                match stream.try_read().await {
                    Ok(Some(msg)) => process_message(&overseer, msg).await,
                    Ok(None) => continue,
                    // TODO: Not bail?
                    Err(err) => bail!(err),
                }
            }
            Err(err) => println!("Failed to accept client: {:?}", err),
        }
    }
}

async fn process_message(overseer: &RwLock<Overseer>, message: NexusMessage) {
    if let Ok(action) = overseer.write().await.process_nexus_message(message)
        && let Err(err) = action.dispatch().await
    {
        println!("Hyprland error: {err}");
    }
}
