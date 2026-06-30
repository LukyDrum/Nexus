use std::ops::Deref;

use nexus_api::WorkspaceNumber;

const DEFAULT_WORKSPACE: WorkspaceNumber = 1;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct GroupName(pub String);

impl Deref for GroupName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Group {
    pub(super) current_number: WorkspaceNumber,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            current_number: DEFAULT_WORKSPACE,
        }
    }
}
