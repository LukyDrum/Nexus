use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NexusResponse {
    Empty,
    Clients(Vec<ActiveClient>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveClient {
    pub title: String,
    pub name: String,
    pub pid: u64,
}
