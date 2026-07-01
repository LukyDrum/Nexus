use nexus_api::{
    Either::{self, Left, Right},
    GroupName, NexusRequest, NexusResponse, WindowSelector, WorkspaceSelector,
};

use crate::{
    core::{
        DEFAULT_GROUP, NexusState,
        error::{InvalidState, NexusResult},
        state::CurrentWorkspace,
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

    /// Processes the message and returns a [`HyprlandAction`] if some needs to be performed or a [`NexusResponse`] if no action is needed.
    /// This action needs yet to be called.
    /// By returning the action instead of calling it directly, we avoid holding a write lock for too long.
    pub fn process_nexus_message(
        &mut self,
        message: NexusRequest,
    ) -> NexusResult<Either<HyprlandAction, NexusResponse>> {
        Ok(match message {
            NexusRequest::SwitchWorkspace(selector) => self.switch_workspace(selector)?,
            NexusRequest::SwitchGroup(name) => Left(self.switch_group(name)?),
            NexusRequest::MoveWindowToWorkspace(window, workspace) => {
                Left(self.move_window_to_workspace(window, workspace)?)
            }
            NexusRequest::FocusWindow(window) => Left(self.focus_window(window)),
            NexusRequest::ListAllClients => Left(self.list_clients()),
            NexusRequest::ListGroups => Right(self.list_groups()),
        })
    }

    /* Bellow lay the implementations of the individual reactions */

    /// Switches to a workspace (identified by number) in the current group.
    fn switch_workspace(
        &mut self,
        workspace: WorkspaceSelector,
    ) -> NexusResult<Either<HyprlandAction, NexusResponse>> {
        match workspace {
            WorkspaceSelector::Focused => Ok(Right(NexusResponse::Empty)),
            WorkspaceSelector::InGroup(number) => {
                // Change the inner state
                let group_name = self.state.last_group().clone();
                let Some(group) = self.state.groups.get_mut(&group_name) else {
                    // The currently opened group is not among the registered groups.
                    // That should not happen.
                    return Err(super::error::NexusError::InvalidState(
                        InvalidState::OpenedGroupNotRegistered,
                    ));
                };

                group.current_number = number;
                self.state
                    .set_current_workspace(CurrentWorkspace::InGroup(group_name));

                // Perform the actual workspace switch in Hyprland
                self.switch_to_current_workspace().map(Left)
            }
            WorkspaceSelector::Named(name) => {
                self.state
                    .set_current_workspace(CurrentWorkspace::Other(name.clone()));
                Ok(Left(HyprlandAction::SwitchWorkspace(name)))
            }
        }
    }

    /// Switches the currently opened group.
    /// If such group does not yet exist, then it will first create it.
    fn switch_group(&mut self, name: GroupName) -> NexusResult<HyprlandAction> {
        // Insert the group in case it does not exist yet and set it as the currently opened.
        self.state.groups.entry(name.clone()).or_default();
        self.state
            .set_current_workspace(CurrentWorkspace::InGroup(name));

        // Switch to the workspace in the newly opened group
        self.switch_to_current_workspace()
    }

    /// A utility function that creates a Hyprland action to switch to the currently opened workspace according to the state.
    fn switch_to_current_workspace(&self) -> NexusResult<HyprlandAction> {
        let new_workspace = self.state.current_workspace_full_name()?;
        Ok(HyprlandAction::SwitchWorkspace(new_workspace))
    }

    /// Moves the window to a workspace given by `WorkspaceSelector`.
    fn move_window_to_workspace(
        &self,
        window: WindowSelector,
        workspace: WorkspaceSelector,
    ) -> NexusResult<HyprlandAction> {
        Ok(match workspace {
            WorkspaceSelector::Focused => {
                let workspace = self.state.current_workspace_full_name()?;
                HyprlandAction::MoveWindowToNamedWorkspace { window, workspace }
            }
            WorkspaceSelector::InGroup(number) => HyprlandAction::MoveWindowToNamedWorkspace {
                window,
                workspace: self.state.name_from_ingroup_workspace(number),
            },
            WorkspaceSelector::Named(name) => HyprlandAction::MoveWindowToNamedWorkspace {
                window,
                workspace: name,
            },
        })
    }

    fn focus_window(&self, window: WindowSelector) -> HyprlandAction {
        HyprlandAction::FocusWindow(window)
    }

    /// List currently active Hyprland clients.
    fn list_clients(&self) -> HyprlandAction {
        HyprlandAction::Clients
    }

    /// List currently active Nexus groups.
    fn list_groups(&self) -> NexusResponse {
        NexusResponse::Groups(self.state.groups.keys().cloned().collect())
    }

    /* Bellow lay the implementation of handlers that react to changes in Hyprland */

    /// Configures the inner state so that it is in sync with change that happened.
    pub(crate) fn on_workspace_changed(&mut self, new_workspace: &str) {
        // The new workspace should be in a format "<group>-<workspace number>"
        let group_and_number = if let Some(group_and_number) =
            new_workspace.rsplit_once('-').and_then(|(group, number)| {
                number
                    .parse()
                    .ok()
                    .map(|number| (GroupName(group.to_owned()), number))
            }) {
            Some(group_and_number)
        }
        // Lets try if it is only a number, in that case we can assume we are in the default group.
        else if let Ok(number) = new_workspace.parse() {
            Some((GroupName(DEFAULT_GROUP.to_owned()), number))
        } else {
            None
        };

        let group_and_number = group_and_number.and_then(|(group_name, number)| {
            self.state
                .groups
                .get_mut(&group_name)
                .map(|group| (group_name, number, group))
        });

        if let Some((group_name, number, group)) = group_and_number {
            group.current_number = number;
            self.state
                .set_current_workspace(CurrentWorkspace::InGroup(group_name));
        } else {
            self.state
                .set_current_workspace(CurrentWorkspace::Other(new_workspace.to_owned()));
        }
    }
}
