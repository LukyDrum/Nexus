use std::{io::Write, os::unix::net::UnixStream, path::PathBuf};

use crate::{
    HyprsError,
    consts::{CONTROL_SOCKET_NAME, HIS_VAR_NAME, HYPR_DIR, XDG_RUNTIME_DIR_VAR_NAME},
    control::CommandParts,
};

pub struct ControlSocket;

impl ControlSocket {
    pub fn send_command(parts: &CommandParts) -> Result<bool, HyprsError> {
        let command = parts.to_string();
        let len = command.len();

        Self::write(command.as_bytes()).map(|send| send == len)
    }

    pub(super) fn write(data: &[u8]) -> Result<usize, HyprsError> {
        let his = std::env::var(HIS_VAR_NAME).map_err(|_| HyprsError::UnknownHis)?;
        let runtime_dir =
            std::env::var(XDG_RUNTIME_DIR_VAR_NAME).map_err(|_| HyprsError::UnknownRuntimeDir)?;
        let path = PathBuf::from(runtime_dir)
            .join(HYPR_DIR)
            .join(his)
            .join(CONTROL_SOCKET_NAME);

        let mut stream = UnixStream::connect(path).map_err(HyprsError::Io)?;

        stream.write(data).map_err(HyprsError::Io)
    }
}
