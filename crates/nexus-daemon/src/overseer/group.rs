use nexus_api::WorkspaceNumber;

#[derive(Clone, Debug)]
pub(crate) struct Group {
    pub(super) current_workspace: WorkspaceNumber,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            current_workspace: 1,
        }
    }
}
