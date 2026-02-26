use std::collections::HashMap;

use crate::overseer::{Group, error::NexusError};

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
    pub fn workspace_name(&self) -> Result<String, NexusError> {
        let Some(group_name) = &self.opened_group else {
            return Ok(HUB_WORKSPACE.to_owned());
        };

        let workspace_number = self
            .groups
            .get(group_name)
            .map(|group| group.current_workspace)
            .ok_or(NexusError::InvalidState)?;

        Ok(format!("{group_name}-{workspace_number}"))
    }

    // TO MOVE

    pub fn switch_group(&mut self, group_name: impl Into<String>) {
        let group_name = group_name.into();
        self.groups
            .entry(group_name.clone())
            .or_insert_with(|| Group::default());

        self.opened_group = Some(group_name);
    }

    pub fn switch_workspace(&mut self, workspace: u8) {
        if let Some(group_name) = &self.opened_group
            && let Some(group) = self.groups.get_mut(group_name)
        {
            group.current_workspace = workspace;
        }
    }
}
