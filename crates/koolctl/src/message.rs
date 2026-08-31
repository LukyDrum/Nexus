use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ControlMessage {
    Run { path: PathBuf },
    Close { path: PathBuf },
}
