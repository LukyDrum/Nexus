pub(crate) type NexusResult<T> = Result<T, NexusError>;

#[derive(Debug)]
pub(crate) enum NexusError {
    InvalidState(InvalidState),
    SyncFailed,
}

#[derive(Debug)]
pub(crate) enum InvalidState {
    OpenedGroupNotRegistered,
}
