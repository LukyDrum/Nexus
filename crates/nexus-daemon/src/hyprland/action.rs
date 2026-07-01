use nexus_api::WindowSelector;

use crate::hyprland::{HyprlandResponse, ctl::hyprctl, hyprctl_eval};

const JSON_FLAG: &str = "-j";
const CLIENTS_COMMAND: &str = "clients";

/// Variants of this enum represent the actions that Nexus could request from Hyprland.
/// It should abstract away the Hyprland specifics and provide an easy interface for [`Overseer`](crate::overseer::Overseer)
/// to perform actions in Hyprland.
pub(crate) enum HyprlandAction {
    /// Performs a switch to a named workspace.
    SwitchWorkspace(String),

    /// Moves the `window` to a named `workspace`.
    MoveWindowToNamedWorkspace {
        window: WindowSelector,
        workspace: String,
    },

    /// Focuses window.
    FocusWindow(WindowSelector),

    /// List active clients.
    Clients,
}

impl HyprlandAction {
    pub async fn dispatch(self) -> anyhow::Result<HyprlandResponse> {
        match self {
            HyprlandAction::SwitchWorkspace(workspace) => {
                let command =
                    format!("hl.dispatch(hl.dsp.focus({{ workspace = \"name:{workspace}\" }}))");
                hyprctl_eval(&command).await?;
                Ok(HyprlandResponse::None)
            }
            HyprlandAction::MoveWindowToNamedWorkspace { window, workspace } => {
                let window = match window {
                    WindowSelector::Focused => "activewindow".to_owned(),
                    WindowSelector::Pid(pid) => format!("pid:{pid}"),
                };

                let command = format!(
                    "hl.dispatch(hl.dsp.window.move({{ workspace = \"name:{workspace}\", window = \"{window}\" }}))"
                );
                hyprctl_eval(&command).await?;
                Ok(HyprlandResponse::None)
            }
            HyprlandAction::FocusWindow(window) => {
                let window = match window {
                    WindowSelector::Focused => return Ok(HyprlandResponse::None),
                    WindowSelector::Pid(pid) => format!("pid:{pid}"),
                };

                let command = format!("hl.dispatch(hl.dsp.focus({{ window = \"{window}\" }}))");
                hyprctl_eval(&command).await?;
                Ok(HyprlandResponse::None)
            }
            HyprlandAction::Clients => {
                let response = hyprctl(&[JSON_FLAG], CLIENTS_COMMAND, &[]).await?;
                let clients = serde_json::from_str(&response)?;
                Ok(HyprlandResponse::Clients(clients))
            }
        }
    }
}
