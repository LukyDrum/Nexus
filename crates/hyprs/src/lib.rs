pub mod control;
mod error;
mod event;
pub mod events;
pub mod types;

pub use error::HyprsError;
pub use event::{EventParseError, HyprlandEvent};

pub mod consts {
    pub const HIS_VAR_NAME: &str = "HYPRLAND_INSTANCE_SIGNATURE";
    pub const XDG_RUNTIME_DIR_VAR_NAME: &str = "XDG_RUNTIME_DIR";
    pub const HYPR_DIR: &str = "hypr";
    pub const CONTROL_SOCKET_NAME: &str = ".socket.sock";
    pub const EVENTS_SOCKET_NAME: &str = ".socket2.sock";
}
