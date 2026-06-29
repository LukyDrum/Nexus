use crate::hyprland::hyprctl_eval;

/// Variants of this enum represent the actions that Nexus could request from Hyprland.
/// It should abstract away the Hyprland specifics and provide an easy interface for [`Overseer`](crate::overseer::Overseer)
/// to perform actions in Hyprland.
pub(crate) enum HyprlandAction {
    /// Performs a switch to a named workspace.
    SwitchWorkspace(String),

    /// Moves the currently active window to a named workspace.
    MoveActiveWindowToWorkspace(String),
}

impl HyprlandAction {
    pub async fn dispatch(self) -> anyhow::Result<()> {
        match self {
            HyprlandAction::SwitchWorkspace(workspace) => {
                let command =
                    format!("hl.dispatch(hl.dsp.focus({{ workspace = \"name:{workspace}\" }}))");
                hyprctl_eval(&command).await?;
            }
            HyprlandAction::MoveActiveWindowToWorkspace(workspace) => {
                let command = format!(
                    "hl.dispatch(hl.dsp.window.move({{ workspace = \"name:{workspace}\" }}))"
                );
                hyprctl_eval(&command).await?;
            }
        }

        Ok(())
    }
}
