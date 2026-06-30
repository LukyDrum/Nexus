use serde::{Deserialize, Serialize};

use crate::types::WorkspaceNumber;

/// The variants of this enum represent the possible requests that will be send to the daemon.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NexusRequest {
    /// Switches to a workspace defined by a [`WorkspaceSelector`].
    /// If the workspace does not exists, then it will be created in the current group.
    SwitchWorkspace(WorkspaceSelector),

    /// Switches to a group defined by a string (the name of the group).
    /// Same as with the workspace, it does not matter if the group already exists or not:
    ///     - If it does, then it will switch to the last used workspace in that group,
    ///     - If it does not, then it will create a new group and switch to workspace #1 in that group.
    SwitchGroup(String),

    /// Moves a window to a workspace.
    MoveWindowToWorkspace(WindowSelector, WorkspaceSelector),

    /// Lists all the currently active clients of the WM.
    ListAllClients,

    /// Lists all currently active groups (as in Nexus group).
    ListGroups,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WorkspaceSelector {
    Focused,
    InGroup(WorkspaceNumber),
    Named(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WindowSelector {
    Focused,
    Pid(u64),
}
