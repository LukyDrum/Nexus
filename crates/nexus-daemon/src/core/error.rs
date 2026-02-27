use hyprland::error::HyprError;

pub(crate) type NexusResult<T> = Result<T, NexusError>;

#[derive(Debug)]
pub(crate) enum NexusError {
    InvalidState(InvalidState),
    HyprlandError(HyprError),
}

#[derive(Debug)]
pub(crate) enum InvalidState {
    OpenedGroupNotRegistered,
}
