use tokio::net::UnixListener;

use crate::{runtime_dir, socket::NexusStream};

pub struct NexusListener {
    listener: UnixListener,
}

impl NexusListener {
    /// Creates a new [`NexusListener`] bounded to a socket of name [`name`] inside Nexuses runtime directory.
    pub fn bind(name: &str) -> std::io::Result<Self> {
        let path = runtime_dir().join(name);

        // Remove potential old socket file
        let _ = std::fs::remove_file(&path);
        // Create the parent dir in case it does not exist
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let listener = UnixListener::bind(path)?;
        Ok(Self { listener })
    }

    /// Accepts a new incoming connection and returns a [`NexusStream`](crate::NexusStream) of type [`T`].
    pub async fn accept(&self) -> std::io::Result<NexusStream> {
        let (stream, _addr) = self.listener.accept().await?;
        Ok(NexusStream::new(stream))
    }
}
