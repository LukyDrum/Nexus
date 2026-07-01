use std::sync::Arc;

use nexus_api::{
    ActiveClient, Either, NEXUS_COMMUNICATION_SOCKET, NexusListener, NexusRequest, NexusResponse,
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
                loop {
                    let response = match stream.read().await {
                        Ok(Some(msg)) => process_message(&overseer, msg).await,
                        Ok(None) => break,
                        Err(_err) => break,
                    };

                    if stream.write(&response).await.is_err() {
                        break;
                    }
                }

                let _ = stream.shutdown().await;
            }
            Err(err) => println!("Failed to accept client: {:?}", err),
        }
    }
}

async fn process_message(overseer: &RwLock<Overseer>, message: NexusRequest) -> NexusResponse {
    let action_or_response = match overseer.write().await.process_nexus_message(message) {
        Ok(action) => action,
        Err(err) => {
            println!("Processing error: {err:?}");
            return NexusResponse::Empty;
        }
    };

    let action = match action_or_response {
        Either::Left(action) => action,
        Either::Right(response) => {
            return response;
        }
    };

    let response = match action.dispatch().await {
        Ok(reponse) => reponse,
        Err(err) => {
            println!("Action dispatch error: {err}");
            return NexusResponse::Empty;
        }
    };

    match response {
        HyprlandResponse::None => NexusResponse::Empty,
        HyprlandResponse::Clients(hyprland_clients) => NexusResponse::Clients(
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
        ),
    }
}
