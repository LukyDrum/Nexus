use crate::EventParseError;

pub mod sync;
pub mod tokio;

#[derive(Debug)]
pub enum HyprlandEventsError {
    Io(std::io::Error),
    EventParsing(EventParseError),
    UnknownHis,
    UnknownRuntimeDir,
}
