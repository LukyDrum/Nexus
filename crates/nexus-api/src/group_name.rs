use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupName(pub String);

impl GroupName {
    /// Either the actual group name or the "unnamed" string in case of an empty (default) group name.
    pub fn display_name(&self) -> &str {
        if self.0.is_empty() {
            &self.0
        } else {
            "unnamed"
        }
    }
}

impl Deref for GroupName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for GroupName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
