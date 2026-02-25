use std::marker::PhantomData;

use anyhow::bail;
use serde::{Deserialize, Serialize};
use tokio::net::UnixStream;

use crate::runtime_dir::runtime_dir;

const BUFFER_SIZE: usize = 1024;

/// An abstraction over a tokio unix socket to make implementing the communication easier.
pub struct NexusStream<T> {
    stream: UnixStream,
    buffer: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T> NexusStream<T> {
    /// Wraps an already established [`UnixStream`].
    pub(crate) fn new(stream: UnixStream) -> Self {
        Self {
            stream,
            buffer: vec![0; BUFFER_SIZE],
            _phantom: PhantomData,
        }
    }

    /// Connects to an already created socket.
    /// It is expected that this function will be used by the various clients wanting to communicate with the daemon.
    pub async fn connect(name: &str) -> std::io::Result<Self> {
        let path = runtime_dir().join(name);
        let stream = UnixStream::connect(path).await?;

        Ok(Self::new(stream))
    }

    /// Waits for the socket to become readable.
    pub async fn readable(&self) -> std::io::Result<()> {
        self.stream.readable().await
    }

    /// Waits for the socket to become writeable.
    pub async fn writable(&self) -> std::io::Result<()> {
        self.stream.writable().await
    }

    /// Tries to read a value of type [`T`] from the socket.
    /// You should first wait using [`Self::readable`] before calling this.
    ///
    /// It requires a `&mut self` because we write the read bytes into an internal buffer that used re-used across reads.
    ///
    /// We return a `Result<Option<_>>` because the [`Self::readable`] method can sometimes return a false-positive.
    /// If it in fact happens, that the socket is not yet readable, we simply return `Ok(None)`.
    pub async fn try_read<'de>(&'de mut self) -> anyhow::Result<Option<T>>
    where
        T: Deserialize<'de>,
    {
        self.buffer.fill(0);

        match self.stream.try_read(&mut self.buffer) {
            // Ok(0) => Ok(None),
            Ok(n) => {
                let data = &self.buffer[0..n];
                let value = serde_json::from_slice::<T>(data)?;
                Ok(Some(value))
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(err) => bail!(err),
        }
    }

    /// Tries to write a value of type [`T`] to the socket.
    /// You should first wait using [`Self::writeable`] before calling this.
    ///
    /// We return a `Result<Option<_>>` because the [`Self::writeable`] method can sometimes return a false-positive.
    /// If it in fact happens, that the socket is not yet writeable, we simply return `Ok(None)`.
    /// Otherwise we return `Ok(Some(json))` where `json` is the JSON representation of the send message.
    pub async fn try_write(&self, value: &T) -> anyhow::Result<Option<String>>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(value)?;

        match self.stream.try_write(json.as_bytes()) {
            Ok(_) => Ok(Some(json)),
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
            Err(err) => bail!(err),
        }
    }
}
