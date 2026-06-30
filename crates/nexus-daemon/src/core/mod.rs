mod error;
mod group;
mod overseer;
mod state;

pub(crate) use error::NexusResult;
pub(crate) use group::Group;
pub(crate) use overseer::Overseer;
pub(crate) use state::NexusState;

/// An unnamed group
const DEFAULT_GROUP: &str = "";
