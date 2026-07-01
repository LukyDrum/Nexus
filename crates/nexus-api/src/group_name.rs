use std::{convert::Infallible, fmt::Display, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GroupName(pub String);

impl GroupName {
    /// Either the actual group name or the "unnamed" string in case of an empty (default) group name.
    pub fn display_name(&self) -> &str {
        if self.0.is_empty() {
            "unnamed"
        } else {
            &self.0
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

impl FromStr for GroupName {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}
