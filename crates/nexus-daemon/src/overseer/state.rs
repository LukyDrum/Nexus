use std::collections::HashMap;

use crate::overseer::{
    Group,
    error::{InvalidState, NexusError, NexusResult},
};

const HUB_WORKSPACE: &str = "hub";

/// [`NexusState`] exists independent of Hyprland.
/// It does not care itself if it is in coherent state with Hyprland.
/// Keeping it that way is the responsibility of [`Overseer`].
pub(crate) struct NexusState {
    /// All the currently active groups.
    /// No matter if they are opened or not.
    pub(super) groups: HashMap<String, Group>,
    pub(super) opened_group: Option<String>,
}

impl NexusState {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            opened_group: None,
        }
    }

    /// Returns the name of the current workspace according to the state.
    /// Can return a [`NexusError::InvalidState`] if the *opened group* is not among the *registered groups*.
    pub fn workspace_name(&self) -> NexusResult<String> {
        let Some(group_name) = &self.opened_group else {
            return Ok(HUB_WORKSPACE.to_owned());
        };

        let workspace_number = self
            .groups
            .get(group_name)
            .map(|group| group.current_workspace)
            .ok_or(NexusError::InvalidState(
                InvalidState::OpenedGroupNotRegistered,
            ))?;

        Ok(format!("{group_name}-{workspace_number}"))
    }
}
