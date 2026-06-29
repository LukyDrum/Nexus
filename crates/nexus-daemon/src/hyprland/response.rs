use serde::Deserialize;

/// A response returned by `hyprctl` after calling some command on it.
#[derive(Debug)]
pub(crate) enum HyprlandResponse {
    None,
    Clients(Vec<HyprlandClient>),
}

/// Information from Hyprland about a running app with window.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HyprlandClient {
    pub title: String,
    pub initial_title: String,
    pub pid: u64,
}
