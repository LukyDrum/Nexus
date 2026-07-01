use std::collections::HashMap;

use nexus_api::{GroupName, WorkspaceNumber};

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
    pub(super) groups: HashMap<GroupName, Group>,
    /// The current workspace we are in. This is identified either by a group name (and the number information is hold in the group itself),
    /// or by a full name of the workspace if this workspace if outside of groups.
    current_workspace: CurrentWorkspace,
    /// The name of the current group we are in, or of the last group we were in if a switch to a workspace outside of groups was performed.
    last_group: GroupName,
}

impl NexusState {
    pub fn new() -> Self {
        let default_group_name = GroupName(DEFAULT_GROUP.to_owned());
        Self {
            groups: HashMap::from([(default_group_name.clone(), Group::default())]),
            current_workspace: CurrentWorkspace::InGroup(default_group_name.clone()),
            last_group: default_group_name,
        }
    }

    /// Returns the full name for the current workspace (according to Nexus).
    /// This should be the same as the actual workspace.
    pub fn current_workspace_full_name(&self) -> NexusResult<String> {
        let group_name = match &self.current_workspace {
            CurrentWorkspace::InGroup(group_name) => group_name,
            CurrentWorkspace::Other(name) => return Ok(name.clone()),
        };

        let workspace_number = self
            .groups
            .get(group_name)
            .map(|group| group.current_number)
            .ok_or(NexusError::InvalidState(
                InvalidState::OpenedGroupNotRegistered,
            ))?;

        Ok(if group_name.is_empty() {
            workspace_number.to_string()
        } else {
            format!("{}-{workspace_number}", &**group_name)
        })
    }

    pub fn name_from_ingroup_workspace(&self, number: WorkspaceNumber) -> String {
        if self.last_group.is_empty() {
            number.to_string()
        } else {
            format!("{}-{number}", &*self.last_group)
        }
    }

    pub fn last_group(&self) -> &GroupName {
        &self.last_group
    }

    pub fn set_current_workspace(&mut self, new_workspace: CurrentWorkspace) {
        match &new_workspace {
            CurrentWorkspace::InGroup(group_name) => self.last_group = group_name.clone(),
            CurrentWorkspace::Other(_) => {}
        }

        self.current_workspace = new_workspace;
    }
}

pub enum CurrentWorkspace {
    InGroup(GroupName),
    Other(String),
}
