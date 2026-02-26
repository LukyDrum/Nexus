pub(crate) type NexusResult<T> = Result<T, NexusError>;

#[derive(Debug)]
pub(crate) enum NexusError {
    InvalidState(InvalidState),
    HyprlandError(hyprland::shared::HyprError),
}

#[derive(Debug)]
pub(crate) enum InvalidState {
    OpenedGroupNotRegistered,
}
