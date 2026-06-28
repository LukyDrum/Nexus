use std::collections::HashMap;

use nexus_api::WorkspaceNumber;

use crate::core::{
    DEFAULT_GROUP, Group,
    error::{InvalidState, NexusError, NexusResult},
};

/// [`NexusState`] exists independent of Hyprland.
/// It does not care itself if it is in coherent state with Hyprland.
/// Keeping it that way is the responsibility of [`Overseer`].
pub(crate) struct NexusState {
    /// All the currently active groups.
    /// No matter if they are opened or not.
    pub(super) groups: HashMap<String, Group>,
    pub(super) current_group: String,
}

impl NexusState {
    pub fn new() -> Self {
        Self {
            groups: HashMap::from([(DEFAULT_GROUP.to_owned(), Group::default())]),
            current_group: DEFAULT_GROUP.to_owned(),
        }
    }

    /// Returns the name of the currently opened workspace according to the state.
    /// Can return a [`NexusError::InvalidState`] if the *opened group* is not among the *registered groups*.
    pub fn current_workspace(&self) -> NexusResult<String> {
        let group_name = &self.current_group;
        let workspace_number = self
            .groups
            .get(group_name)
            .map(|group| group.current_workspace)
            .ok_or(NexusError::InvalidState(
                InvalidState::OpenedGroupNotRegistered,
            ))?;

        Ok(self.name_for_current_workspace(workspace_number))
    }

    pub fn name_for_current_workspace(&self, workspace: WorkspaceNumber) -> String {
        if self.current_group.is_empty() {
            workspace.to_string()
        } else {
            format!("{}-{workspace}", self.current_group)
        }
    }
}
