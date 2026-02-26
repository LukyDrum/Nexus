mod hyprland;
mod overseer;
mod tasks;

use std::sync::Arc;
use tokio::{select, sync::RwLock};

use crate::{
    overseer::{NexusState, Overseer},
    tasks::communication_task,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = NexusState::new();
    let overseer = Arc::new(RwLock::new(Overseer::new(state)));

    select! {
        result = communication_task(overseer) => {
            println!("Communication task exited!");
            result
        }
    }
}
