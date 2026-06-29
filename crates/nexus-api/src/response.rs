use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NexusResponse {
    /// An empty response.
    Empty,
    /// A list of currently active clients.
    Clients(Vec<ActiveClient>),
    /// A list of names of the currently active Nexus groups.
    Groups(Vec<String>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveClient {
    pub title: String,
    pub name: String,
    pub pid: u64,
}
