use std::path::PathBuf;

use tokio::{io::AsyncReadExt, net::UnixStream};

use crate::{HyprlandEvent, events::HyprlandEventsError};

const HIS_VAR: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const XDG_RUNTIME_DIR_VAR: &str = "XDG_RUNTIME_DIR";
const HYPR_DIR: &str = "hypr";
const SOCKET_2_NAME: &str = ".socket2.sock";

pub struct HyprlandEvents {
    stream: UnixStream,
    buffer: Vec<u8>,
}

impl HyprlandEvents {
    pub async fn new() -> Result<Self, HyprlandEventsError> {
        let his = std::env::var(HIS_VAR).map_err(|_| HyprlandEventsError::UnknownHis)?;
        let runtime_dir = std::env::var(XDG_RUNTIME_DIR_VAR)
            .map_err(|_| HyprlandEventsError::UnknownRuntimeDir)?;
        let path = PathBuf::from(runtime_dir)
            .join(HYPR_DIR)
            .join(his)
            .join(SOCKET_2_NAME);

        let stream = UnixStream::connect(path)
            .await
            .map_err(HyprlandEventsError::Io)?;

        Ok(Self {
            stream,
            buffer: vec![0; 1024],
        })
    }

    pub async fn read(&mut self) -> Result<Vec<HyprlandEvent>, HyprlandEventsError> {
        self.stream
            .readable()
            .await
            .map_err(HyprlandEventsError::Io)?;

        let n = self
            .stream
            .read(&mut self.buffer[..])
            .await
            .map_err(HyprlandEventsError::Io)?;
        let read = &self.buffer[..n];
        let read = String::from_utf8_lossy(read);

        let mut events = Vec::new();
        for event_with_data in read.lines() {
            let event = HyprlandEvent::parse_from_line(event_with_data)
                .map_err(HyprlandEventsError::EventParsing)?;
            events.push(event);
        }

        Ok(events)
    }
}
