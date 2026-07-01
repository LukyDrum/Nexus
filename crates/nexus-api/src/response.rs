use serde::{Deserialize, Serialize};

use crate::GroupName;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NexusResponse {
    /// An empty response.
    Empty,
    /// A list of currently active clients.
    Clients(Vec<ActiveClient>),
    /// A list of names of the currently active Nexus groups.
    Groups(Vec<GroupName>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveClient {
    pub title: String,
    pub name: String,
    pub pid: u64,
}
