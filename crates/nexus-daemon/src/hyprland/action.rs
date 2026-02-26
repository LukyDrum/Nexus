use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};

/// Variants of this enum represent the actions that Nexus could request from Hyprland.
/// It should abstract away the Hyprland specifics and provide an easy interface for [`Overseer`](crate::overseer::Overseer)
/// to perform actions in Hyprland.
pub(crate) enum HyprlandAction {
    /// Performs a switch to a named workspace.
    SwitchWorkspace(String),
}

impl HyprlandAction {
    pub async fn dispatch(self) -> hyprland::Result<()> {
        match self {
            HyprlandAction::SwitchWorkspace(workspace) => {
                let workspace = WorkspaceIdentifierWithSpecial::Name(&workspace);
                let dispatch_type = DispatchType::Workspace(workspace);
                Dispatch::call_async(dispatch_type).await
            }
        }
    }
}
