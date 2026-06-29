use std::sync::Arc;

use anyhow::bail;
use nexus_api::{
    ActiveClient, NEXUS_COMMUNICATION_SOCKET, NexusListener, NexusMessage, NexusResponse,
};
use tokio::sync::RwLock;

use crate::{
    core::Overseer,
    hyprland::{HyprlandClient, HyprlandResponse},
};

/// A task that is responsible for communication with the rest of the Nexus ecosystem.
/// It accepts connections to a communication unix socket and performs actions based on the incoming messages.
pub(crate) async fn communication_task(overseer: Arc<RwLock<Overseer>>) -> anyhow::Result<()> {
    let listener = NexusListener::bind(NEXUS_COMMUNICATION_SOCKET)?;

    loop {
        match listener.accept().await {
            Ok(mut stream) => {
                stream.readable().await?;
                let Some(response) = (match stream.try_read().await {
                    Ok(Some(msg)) => process_message(&overseer, msg).await,
                    Ok(None) => continue,
                    // TODO: Not bail?
                    Err(err) => bail!(err),
                }) else {
                    let _ = stream.shutdown().await;
                    continue;
                };

                stream.writable().await?;
                stream.try_write(&response).await?;
                let _ = stream.shutdown().await;
            }
            Err(err) => println!("Failed to accept client: {:?}", err),
        }
    }
}

async fn process_message(
    overseer: &RwLock<Overseer>,
    message: NexusMessage,
) -> Option<NexusResponse> {
    let action = match overseer.write().await.process_nexus_message(message) {
        Ok(action) => action,
        Err(err) => {
            println!("Processing error: {err:?}");
            return None;
        }
    };

    let response = match action.dispatch().await {
        Ok(reponse) => reponse,
        Err(err) => {
            println!("Action dispatch error: {err}");
            return None;
        }
    };

    match response {
        HyprlandResponse::None => None,
        HyprlandResponse::Clients(hyprland_clients) => Some(NexusResponse::Clients(
            hyprland_clients
                .into_iter()
                .map(
                    |HyprlandClient {
                         title,
                         initial_title,
                         pid,
                     }| ActiveClient {
                        title,
                        name: initial_title,
                        pid,
                    },
                )
                .collect(),
        )),
    }
}
