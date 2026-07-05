use std::path::PathBuf;

const XDG_RUNTIME_DIR_ENV: &str = "XDG_RUNTIME_DIR";
const DEFAULT_RUNTIME_DIR: &str = "/tmp/";
const HOME_DIR_ENV: &str = "HOME";
const CONFIG_DIR: &str = ".config";

const NEXUS_DIR: &str = "nexus";
/// Name of the default Nexus communication socket.
pub const NEXUS_COMMUNICATION_SOCKET: &str = "nexus.sock";

pub fn runtime_dir() -> PathBuf {
    let runtime_dir = PathBuf::from(
        std::env::var(XDG_RUNTIME_DIR_ENV).unwrap_or_else(|_| DEFAULT_RUNTIME_DIR.to_owned()),
    );

    runtime_dir.join(NEXUS_DIR)
}

pub fn config_dir() -> PathBuf {
    let home_dir = PathBuf::from(std::env::var(HOME_DIR_ENV).unwrap_or_else(|_| "~".to_owned()));

    home_dir.join(CONFIG_DIR).join(NEXUS_DIR)
}
