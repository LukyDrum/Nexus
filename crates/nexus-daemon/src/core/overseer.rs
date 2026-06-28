use nexus_api::{NexusMessage, WorkspaceNumber};

use crate::{
    core::{
        DEFAULT_GROUP, NexusState,
        error::{InvalidState, NexusError, NexusResult},
    },
    hyprland::HyprlandAction,
};

/// [`Overseer`] acts as the middle man between the tasks that listen for incoming messages, for Hyprland event and so on.
/// It is responsible for reacting to any events and keeping the state of Nexus in coherent state.
pub(crate) struct Overseer {
    state: NexusState,
}

impl Overseer {
    pub fn new(state: NexusState) -> Self {
        Self { state }
    }

    /// Processes the message and returns a [`HyprlandAction`] if some needs to be performed.
    /// This action needs yet to be called.
    /// By returning the action instead of calling it directly, we avoid holding a write lock for too long.
    pub fn process_nexus_message(&mut self, message: NexusMessage) -> NexusResult<HyprlandAction> {
        Ok(match message {
            NexusMessage::SwitchWorkspace(number) => self.switch_workspace(number)?,
            NexusMessage::SwitchGroup(name) => self.switch_group(name)?,
            NexusMessage::MoveActiveWindowToWorkspace(number) => {
                self.move_active_window(self.state.name_for_current_workspace(number))
            }
            NexusMessage::MoveActiveWindowToNamedWorkspace(named_workspace) => {
                self.move_active_window(named_workspace)
            }
        })
    }

    /* Bellow lay the implementations of the individual reactions */

    /// Switches to a workspace (identified by number) in the current group.
    fn switch_workspace(&mut self, workspace: WorkspaceNumber) -> NexusResult<HyprlandAction> {
        // Change the inner state
        let Some(group) = self.state.groups.get_mut(&self.state.current_group) else {
            // The currently opened group is not among the registered groups.
            // That should not happen.
            return Err(super::error::NexusError::InvalidState(
                InvalidState::OpenedGroupNotRegistered,
            ));
        };

        group.current_workspace = workspace;

        // Perform the actual workspace switch in Hyprland
        self.switch_to_current_workspace()
    }

    /// Switches the currently opened group.
    /// If such group does not yet exist, then it will first create it.
    fn switch_group(&mut self, name: String) -> NexusResult<HyprlandAction> {
        // Insert the group in case it does not exist yet and set it as the currently opened.
        self.state.groups.entry(name.clone()).or_default();
        self.state.current_group = name;

        // Switch to the workspace in the newly opened group
        self.switch_to_current_workspace()
    }

    /// Moves the currently active window to a workspace in the current group with number `target_workspace`.
    fn move_active_window(&mut self, target_workspace: String) -> HyprlandAction {
        HyprlandAction::MoveActiveWindowToWorkspace(target_workspace)
    }

    /// A utility function that creates a Hyprland action to switch to the currently opened workspace according to the state.
    fn switch_to_current_workspace(&self) -> NexusResult<HyprlandAction> {
        let new_workspace = self.state.current_workspace()?;
        Ok(HyprlandAction::SwitchWorkspace(new_workspace))
    }

    /* Bellow lay the implementation of handlers that react to changes in Hyprland */

    /// Configures the inner state so that it reacted to the workspace change.
    pub(crate) fn on_workspace_changed(&mut self, new_workspace: &str) -> NexusResult<()> {
        // The new workspace should be in a format "<group>-<workspace number>"
        let (group, number) = if let Some(group_and_number) = new_workspace
            .rsplit_once('-')
            .and_then(|(group, number)| number.parse().ok().map(|number| (group, number)))
        {
            group_and_number
        } else {
            // Lets try if it is only a number, in that case we can assume we are in the default group.
            let Ok(number) = new_workspace.parse() else {
                return Err(NexusError::SyncFailed);
            };
            (DEFAULT_GROUP, number)
        };

        self.state.current_group = group.to_owned();
        if let Some(group) = self.state.groups.get_mut(group) {
            group.current_workspace = number;
        }

        Ok(())
    }
}
