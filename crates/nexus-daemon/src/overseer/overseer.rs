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
            NexusMessage::SwitchGroup(name) => self.switch_group(name).await,
            NexusMessage::SwitchToHub => self.switch_to_hub().await,
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
        self.switch_to_opened_workspace().await
    }

    /// Switches the currently opened group.
    /// If such group does not yet exist, then it will first create it.
    async fn switch_group(&mut self, name: String) -> NexusResult<()> {
        // Insert the group in case it does not exist yet and set it as the currently opened.
        self.state.groups.entry(name.clone()).or_default();
        self.state.opened_group = Some(name);

        // Switch to the workspace in the newly opened group
        self.switch_to_opened_workspace().await
    }

    /// Switches to the *Hub* group/workspace.
    async fn switch_to_hub(&mut self) -> NexusResult<()> {
        self.state.opened_group = None;

        self.switch_to_opened_workspace().await
    }

    /// A utility function that dispatches a Hyprland action to switch to the currently opened workspace according to the state.
    async fn switch_to_opened_workspace(&self) -> NexusResult<()> {
        let new_workspace = self.state.opened_workspace()?;
        HyprlandAction::SwitchWorkspace(new_workspace)
            .dispatch()
            .await
            .map_err(NexusError::HyprlandError)
    }
}
