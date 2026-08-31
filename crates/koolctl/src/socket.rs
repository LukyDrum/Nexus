use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
};

use thiserror::Error;

use crate::ControlMessage;

const SOCKET_NAME: &str = "kool.sock";
const STREAM_BUFFER_SIZE: usize = 1024;

/// Returns the path to a Kool control socket located in `$XDG_RUNTIME_DIR`.
/// If the env var is not set, then the it returns a path to a socket located in `/tmp/`.
pub fn get_socket_path() -> PathBuf {
    let dirs = xdg::BaseDirectories::new();
    dirs.place_runtime_file(SOCKET_NAME)
        .ok()
        .unwrap_or_else(|| PathBuf::from(format!("/tmp/{SOCKET_NAME}")))
}

#[derive(Debug, Error)]
pub enum ControlSocketError {
    #[error("Failed to serialize message: {0}")]
    Serialization(postcard::Error),
    #[error("Failed to deserialize message: {0}")]
    Deserialization(postcard::Error),
    #[error("Failed to write bytes: {0:?}")]
    Write(std::io::Error),
    #[error("Failed to read bytes: {0:?}")]
    Read(std::io::Error),
}

pub struct ControlListener {
    listener: UnixListener,
}

impl ControlListener {
    pub fn new() -> std::io::Result<Self> {
        let path = get_socket_path();

        // Remove potential old socket file
        let _ = std::fs::remove_file(&path);

        let listener = UnixListener::bind(path)?;
        Ok(Self { listener })
    }

    pub fn accept(&self) -> std::io::Result<ControlStream> {
        let (stream, _addr) = self.listener.accept()?;
        Ok(ControlStream::new(stream))
    }

    pub fn incoming(&self) -> impl Iterator<Item = Result<ControlStream, std::io::Error>> {
        self.listener
            .incoming()
            .map(|stream| stream.map(ControlStream::new))
    }
}

pub struct ControlStream {
    stream: UnixStream,
    buffer: Vec<u8>,
}

impl ControlStream {
    /// Wraps an already established [`UnixStream`].
    pub(crate) fn new(stream: UnixStream) -> Self {
        Self {
            stream,
            buffer: vec![0; STREAM_BUFFER_SIZE],
        }
    }

    /// Connects to an already created socket.
    pub fn connect() -> std::io::Result<Self> {
        let path = get_socket_path();
        let stream = UnixStream::connect(path)?;

        Ok(Self::new(stream))
    }

    /// Closes the stream in this direction.
    /// The other side will receive a write of length 0.
    pub fn shutdown(self) -> std::io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }

    /// Reads a `ControlMessage` from the socket.
    pub fn read(&mut self) -> Result<Option<ControlMessage>, ControlSocketError> {
        self.buffer.fill(0);

        match self.stream.read(&mut self.buffer) {
            Ok(0) => Ok(None),
            Ok(n) => {
                let message = postcard::from_bytes(&self.buffer[..n])
                    .map_err(ControlSocketError::Deserialization)?;
                Ok(Some(message))
            }
            Err(error) => Err(ControlSocketError::Read(error)),
        }
    }

    /// Writes a `ControlMessage` to the socket.
    pub fn write(&mut self, value: &ControlMessage) -> Result<(), ControlSocketError> {
        let bytes = postcard::to_allocvec(value).map_err(ControlSocketError::Serialization)?;
        let mut written = 0;

        while written < bytes.len() {
            match self.stream.write(&bytes[written..]) {
                Ok(n) => written += n,
                Err(error) => return Err(ControlSocketError::Write(error)),
            }
        }

        Ok(())
    }
}
