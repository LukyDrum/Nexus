use anyhow::bail;
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncWriteExt, net::UnixStream};

use crate::runtime_dir::runtime_dir;

const BUFFER_SIZE: usize = 1024;

/// An abstraction over a tokio unix socket to make implementing the communication easier.
pub struct NexusStream {
    stream: UnixStream,
    buffer: Vec<u8>,
}

impl NexusStream {
    /// Wraps an already established [`UnixStream`].
    pub(crate) fn new(stream: UnixStream) -> Self {
        Self {
            stream,
            buffer: vec![0; BUFFER_SIZE],
        }
    }

    /// Connects to an already created socket.
    /// It is expected that this function will be used by the various clients wanting to communicate with the daemon.
    pub async fn connect(name: &str) -> std::io::Result<Self> {
        let path = runtime_dir().join(name);
        let stream = UnixStream::connect(path).await?;

        Ok(Self::new(stream))
    }

    /// Closes the stream in this direction.
    /// The other side will receive a write of length 0.
    pub async fn shutdown(mut self) -> std::io::Result<()> {
        self.stream.shutdown().await
    }

    /// Tries to read a value of type [`T`] from the socket.
    /// If the socket got closed, returns `Ok(None)`.
    pub async fn read<'de, T>(&'de mut self) -> anyhow::Result<Option<T>>
    where
        T: Deserialize<'de>,
    {
        loop {
            self.buffer.fill(0);

            self.stream.readable().await?;
            match self.stream.try_read(&mut self.buffer) {
                Ok(0) => return Ok(None),
                Ok(n) => {
                    let data = &self.buffer[0..n];
                    let value = serde_json::from_slice::<T>(data)?;
                    return Ok(Some(value));
                }
                Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(err) => bail!(err),
            }
        }
    }

    /// Writes a value of type [`T`] to the socket.
    pub async fn write<T>(&self, value: &T) -> anyhow::Result<String>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(value)?;
        let bytes = json.as_bytes();
        let mut written = 0;

        while written < bytes.len() {
            self.stream.writable().await?;
            match self.stream.try_write(&bytes[written..]) {
                Ok(n) => written += n,
                Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(err) => bail!(err),
            }
        }

        Ok(json)
    }
}
