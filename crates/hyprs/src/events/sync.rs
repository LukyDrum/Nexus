use std::{io::Read, os::unix::net::UnixStream, path::PathBuf};

use crate::{
    HyprlandEvent, HyprsError,
    consts::{EVENTS_SOCKET_NAME, HIS_VAR_NAME, HYPR_DIR, XDG_RUNTIME_DIR_VAR_NAME},
};

pub struct HyprlandEvents {
    stream: UnixStream,
    buffer: Vec<u8>,
}

impl HyprlandEvents {
    pub fn new() -> Result<Self, HyprsError> {
        let his = std::env::var(HIS_VAR_NAME).map_err(|_| HyprsError::UnknownHis)?;
        let runtime_dir =
            std::env::var(XDG_RUNTIME_DIR_VAR_NAME).map_err(|_| HyprsError::UnknownRuntimeDir)?;
        let path = PathBuf::from(runtime_dir)
            .join(HYPR_DIR)
            .join(his)
            .join(EVENTS_SOCKET_NAME);

        let stream = UnixStream::connect(path).map_err(HyprsError::Io)?;

        Ok(Self {
            stream,
            buffer: vec![0; 1024],
        })
    }

    pub fn read(&mut self) -> Result<Vec<HyprlandEvent>, HyprsError> {
        let n = self
            .stream
            .read(&mut self.buffer[..])
            .map_err(HyprsError::Io)?;
        let read = &self.buffer[..n];
        let read = String::from_utf8_lossy(read);

        let mut events = Vec::new();
        for event_with_data in read.lines() {
            let event = HyprlandEvent::parse_from_line(event_with_data)
                .map_err(HyprsError::EventParsing)?;
            events.push(event);
        }

        Ok(events)
    }
}
