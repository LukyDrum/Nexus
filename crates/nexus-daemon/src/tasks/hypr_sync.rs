use std::{env, path::PathBuf, sync::Arc};

use tokio::{io::AsyncReadExt, net::UnixStream, sync::RwLock};

use crate::core::Overseer;

const HIS_VAR: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const RUNTIME_DIR_VAR: &str = "XDG_RUNTIME_DIR";

const CREATE_WORKSPACE: &str = "createworkspace";
const CHANGE_WORKSPACE: &str = "workspace";

/// Task that listens for Hyprland events and updates [`Overseer`](crate::core::Overseer).
pub(crate) async fn hypr_sync_task(overseer: Arc<RwLock<Overseer>>) -> anyhow::Result<()> {
    let runtime_dir = env::var(RUNTIME_DIR_VAR)?;
    let his = env::var(HIS_VAR)?;

    let path = PathBuf::from(runtime_dir)
        .join("hypr")
        .join(his)
        .join(".socket2.sock");
    let mut stream = UnixStream::connect(path).await?;
    let mut buffer = vec![0; 1024];

    loop {
        let Ok(n) = stream.read(&mut buffer).await else {
            continue;
        };

        let received = &buffer[0..n];
        for event in received.split(|c| *c == b'\n') {
            let event = String::from_utf8_lossy(event);
            let Some((event, data)) = event.split_once(">>") else {
                continue;
            };

            match event {
                CREATE_WORKSPACE => { /* For now we have no use */ }
                CHANGE_WORKSPACE => {
                    overseer.write().await.on_workspace_changed(data);
                }

                _ => {}
            }
        }
    }
}
