use crate::EventParseError;

#[derive(Debug)]
pub enum HyprsError {
    Io(std::io::Error),
    EventParsing(EventParseError),
    UnknownHis,
    UnknownRuntimeDir,
}
