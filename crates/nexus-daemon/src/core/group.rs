use nexus_api::WorkspaceNumber;

const DEFAULT_WORKSPACE: WorkspaceNumber = 1;

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
