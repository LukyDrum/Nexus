use std::sync::Arc;

use anyhow::bail;
use nexus_api::{NEXUS_COMMUNICATION_SOCKET, NexusListener, NexusMessage};
use tokio::sync::RwLock;

use crate::overseer::Overseer;

/// A task that is responsible for communication with the rest of the Nexus ecosystem.
/// It accepts connections to a communication unix socket and performs actions based on the incoming messages.
pub(crate) async fn communication_task(overseer: Arc<RwLock<Overseer>>) -> anyhow::Result<()> {
    let listener = NexusListener::bind(NEXUS_COMMUNICATION_SOCKET)?;

    loop {
        match listener.accept::<NexusMessage>().await {
            Ok(mut stream) => {
                stream.readable().await?;

                match stream.try_read().await {
                    Ok(Some(msg)) => {
                        // Spawn task so that we can read more messages while the current one is being processed.
                        tokio::spawn(process_message(overseer.clone(), msg));
                    }
                    Ok(None) => continue,
                    Err(err) => bail!(err),
                }
            }
            Err(e) => println!("Failed to accept client: {:?}", e),
        }
    }
}

async fn process_message(overseer: Arc<RwLock<Overseer>>, message: NexusMessage) {
    overseer.write().await.process_nexus_message(message);
}
