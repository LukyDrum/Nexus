use serde::{Deserialize, Serialize};

use crate::types::WorkspaceNumber;

/// The variants of this enum represent the possible messages that will be passed between
/// the parts of the shell using the [`NexusSocket`](crate::NexusSocket).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NexusMessage {
    /// Switches to a workspace defined by a number in the **current group**.
    /// It does not matter if the workspace already exists or not:
    ///     - If it does, then it will just switch to it,
    ///     - If it does not, then it will create it and switch to it.
    ///
    /// This mimics the behaviour of Hyprland.
    SwitchWorkspace(WorkspaceNumber),

    /// Switches to a group defined by a string (the name of the group).
    /// Same as with the workspace, it does not matter if the group already exists or not:
    ///     - If it does, then it will switch to the last used workspace in that group,
    ///     - If it does not, then it will create a new group and switch to workspace #1 in that group.
    SwitchGroup(String),

    /// Switches to the special hub workspace which exists outside of the groups.
    SwitchToHub,
}
