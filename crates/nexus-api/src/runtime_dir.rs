use std::path::PathBuf;

const XDG_RUNTIME_DIR_ENV: &str = "XDG_RUNTIME_DIR";
const DEFAULT_DIR: &str = "/tmp/";
const NEXUS_DIR: &str = "nexus";
/// Name of the default Nexus communication socket.
pub const NEXUS_COMMUNICATION_SOCKET: &str = "nexus.sock";

pub fn runtime_dir() -> PathBuf {
    let runtime_dir = PathBuf::from(
        std::env::var(XDG_RUNTIME_DIR_ENV).unwrap_or_else(|_| DEFAULT_DIR.to_owned()),
    );

    runtime_dir.join(NEXUS_DIR)
}
