mod core;
mod hyprland;
mod tasks;

use std::sync::Arc;
use tokio::{select, sync::RwLock};

use crate::{
    core::{NexusState, Overseer},
    tasks::{communication_task, hypr_sync_task},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = NexusState::new();
    let overseer = Arc::new(RwLock::new(Overseer::new(state)));

    select! {
        result = communication_task(overseer.clone()) => {
            println!("Communication task exited!");
            result
        },
        result = hypr_sync_task(overseer) => {
            println!("Hyprland sync task exited!");
            result
        }
    }
}
