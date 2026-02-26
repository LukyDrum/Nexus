use nexus_api::{NexusMessage, WorkspaceNumber};

use crate::{
    hyprland::HyprlandAction,
    overseer::{
        NexusState,
        error::{InvalidState, NexusError, NexusResult},
    },
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
    pub async fn process_nexus_message(&mut self, message: NexusMessage) -> NexusResult<()> {
        match message {
            NexusMessage::SwitchWorkspace(number) => self.switch_workspace(number).await,
            NexusMessage::SwitchGroup(name) => todo!(),
            NexusMessage::SwitchToHub => todo!(),
        }
    }

    /* Bellow lay the implementations of the individual reactions */

    /// Switches to a workspace (identified by number) in the current group.
    /// Calling this when on the *Hub (default) workspace* will do **nothing**.
    async fn switch_workspace(&mut self, workspace: WorkspaceNumber) -> NexusResult<()> {
        // Change the inner state
        {
            let Some(group) = &self.state.opened_group else {
                // Looks like we are in the Hub workspace, so we do nothing.
                return Ok(());
            };
            let Some(group) = self.state.groups.get_mut(group) else {
                // The currently opened group is not among the registered groups.
                // That should not happen.
                return Err(super::error::NexusError::InvalidState(
                    InvalidState::OpenedGroupNotRegistered,
                ));
            };

            group.current_workspace = workspace;
        }

        // Perform the actual workspace switch in Hyprland
        let new_workspace = self.state.workspace_name()?;
        HyprlandAction::SwitchWorkspace(new_workspace)
            .dispatch()
            .await
            .map_err(NexusError::HyprlandError)
    }

    fn switch_group(&mut self, name: String) {}

    fn switch_to_hub(&mut self) {}
}
