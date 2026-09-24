use serde::{Deserialize, Serialize};

use crate::types::{
    Floating, Fullscreen, IgnoreGroupLock, KeyboardName, LayoutName, LockGroups, Minimized,
    MonitorDescription, MonitorId, MonitorName, Namespace, PinState, ScreenCastName,
    ScreenCastOwner, ScreenCastState, SubMapName, ToggleStatus, WindowAddres, WindowClass,
    WindowTitle, WorkspaceId, WorkspaceName,
};

const DELIMITER: &str = ">>";
const DATA_SEPARATOR: char = ',';

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum HyprlandEvent {
    Workspace {
        name: WorkspaceName,
    },
    WorkspaceV2 {
        id: WorkspaceId,
        name: WorkspaceName,
    },
    FocusedMonitor {
        monitor: MonitorName,
        workspace: WorkspaceName,
    },
    FocusedMonitorV2 {
        monitor: MonitorName,
        workspace_id: WorkspaceId,
    },
    ActiveWindow {
        class: WindowClass,
        title: WindowTitle,
    },
    ActiveWindowV2 {
        address: WindowAddres,
    },
    Fullscreen {
        fullscreen: Fullscreen,
    },
    MonitorRemoved {
        name: MonitorName,
    },
    MonitorRemovedV2 {
        id: MonitorId,
        name: MonitorName,
        description: MonitorDescription,
    },
    MonitorAdded {
        name: MonitorName,
    },
    MonitorAddedV2 {
        id: MonitorId,
        name: MonitorName,
        description: MonitorDescription,
    },
    CreateWorkspace {
        name: WorkspaceName,
    },
    CreateWorkspaceV2 {
        id: WorkspaceId,
        name: WorkspaceName,
    },
    DestroyWorkspace {
        name: WorkspaceName,
    },
    DestroyWorkspaceV2 {
        id: WorkspaceId,
        name: WorkspaceName,
    },
    MoveWorkspace {
        workspace: WorkspaceName,
        monitor: MonitorName,
    },
    MoveWorkspaceV2 {
        workspace_id: WorkspaceId,
        workspace: WorkspaceName,
        monitor: MonitorName,
    },
    ActiveSpecial {
        workspace: WorkspaceName,
        monitor: MonitorName,
    },
    ActiveLayout {
        keyboard: KeyboardName,
        layout: LayoutName,
    },
    OpenWindow {
        address: WindowAddres,
        workspace: WorkspaceName,
        class: WindowClass,
        title: WindowTitle,
    },
    CloseWindow {
        address: WindowAddres,
    },
    Kill {
        address: WindowAddres,
    },
    MoveWindow {
        address: WindowAddres,
        workspace: WorkspaceName,
    },
    MoveWindowV2 {
        address: WindowAddres,
        workspace_id: WorkspaceId,
        workspace: WorkspaceName,
    },
    OpenLayer {
        namespace: Namespace,
    },
    CloseLayer {
        namespace: Namespace,
    },
    SubMap {
        submap: SubMapName,
    },
    ChangeFloatingMode {
        address: WindowAddres,
        floating: Floating,
    },
    Urgent {
        address: WindowAddres,
    },
    ScreenCast {
        state: ScreenCastState,
        owner: ScreenCastOwner,
    },
    ScreenCastV2 {
        state: ScreenCastState,
        owner: ScreenCastOwner,
        name: ScreenCastName,
    },
    WindowTitle {
        address: WindowAddres,
    },
    WindowTitleV2 {
        address: WindowAddres,
        title: WindowTitle,
    },
    ToggleGroup {
        status: ToggleStatus,
        addresses: Vec<WindowAddres>,
    },
    MoveIntoGroup {
        address: WindowAddres,
    },
    MoveOutOfGroup {
        address: WindowAddres,
    },
    IgnoreGroupLock {
        ignore: IgnoreGroupLock,
    },
    LockGroups {
        lock: LockGroups,
    },
    ConfigReloaded,
    Pin {
        address: WindowAddres,
        state: PinState,
    },
    Minimized {
        address: WindowAddres,
        minimized: Minimized,
    },
    Bell {
        address: WindowAddres,
    },
}

#[derive(Clone, Debug)]
pub enum EventParseError {
    NoDelimiter,
    InvalidData,
    NotEnoughData,
    UnknownEvent(String),
}

macro_rules! parse_event_data {
    ($data:ident, $first:ident) => {
        $first::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?
    };
    ($data:ident, $first:ident, $second:ident) => {{
        let first = $first::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        let second = $second::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        (first, second)
    }};
    ($data:ident, $first:ident, $second:ident, $third:ident) => {{
        let first = $first::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        let second = $second::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        let third = $third::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        (first, second, third)
    }};
    ($data:ident, $first:ident, $second:ident, $third:ident, $fourth:ident) => {{
        let first = $first::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        let second = $second::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        let third = $third::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        let fourth = $fourth::try_from($data.next().ok_or(EventParseError::NotEnoughData)?)
            .map_err(|_| EventParseError::InvalidData)?;
        (first, second, third, fourth)
    }};
}

impl HyprlandEvent {
    pub fn parse_from_line(line: &str) -> Result<Self, EventParseError> {
        let (event, data) = line
            .trim()
            .split_once(DELIMITER)
            .ok_or(EventParseError::NoDelimiter)?;

        let mut data = data.split(DATA_SEPARATOR);
        let event = match event {
            "workspace" => {
                let name = parse_event_data!(data, WorkspaceName);
                HyprlandEvent::Workspace { name }
            }
            "workspacev2" => {
                let (id, name) = parse_event_data!(data, WorkspaceId, WorkspaceName);
                HyprlandEvent::WorkspaceV2 { id, name }
            }
            "focusedmon" => {
                let (monitor, workspace) = parse_event_data!(data, MonitorName, WorkspaceName);
                HyprlandEvent::FocusedMonitor { monitor, workspace }
            }
            "focusedmonv2" => {
                let (monitor, workspace_id) = parse_event_data!(data, MonitorName, WorkspaceId);
                HyprlandEvent::FocusedMonitorV2 {
                    monitor,
                    workspace_id,
                }
            }
            "activewindow" => {
                let (class, title) = parse_event_data!(data, WindowClass, WindowTitle);
                HyprlandEvent::ActiveWindow { class, title }
            }
            "activewindowv2" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::ActiveWindowV2 { address }
            }
            "fullscreen" => {
                let fullscreen = parse_event_data!(data, Fullscreen);
                HyprlandEvent::Fullscreen { fullscreen }
            }
            "monitorremoved" => {
                let name = parse_event_data!(data, MonitorName);
                HyprlandEvent::MonitorRemoved { name }
            }
            "monitorremovedv2" => {
                let (id, name, description) =
                    parse_event_data!(data, MonitorId, MonitorName, MonitorDescription);
                HyprlandEvent::MonitorRemovedV2 {
                    id,
                    name,
                    description,
                }
            }
            "monitoradded" => {
                let name = parse_event_data!(data, MonitorName);
                HyprlandEvent::MonitorAdded { name }
            }
            "monitoraddedv2" => {
                let (id, name, description) =
                    parse_event_data!(data, MonitorId, MonitorName, MonitorDescription);
                HyprlandEvent::MonitorAddedV2 {
                    id,
                    name,
                    description,
                }
            }
            "createworkspace" => {
                let name = parse_event_data!(data, WorkspaceName);
                HyprlandEvent::CreateWorkspace { name }
            }
            "createworkspacev2" => {
                let (id, name) = parse_event_data!(data, WorkspaceId, WorkspaceName);
                HyprlandEvent::CreateWorkspaceV2 { id, name }
            }
            "destroyworkspace" => {
                let name = parse_event_data!(data, WorkspaceName);
                HyprlandEvent::DestroyWorkspace { name }
            }
            "destroyworkspacev2" => {
                let (id, name) = parse_event_data!(data, WorkspaceId, WorkspaceName);
                HyprlandEvent::DestroyWorkspaceV2 { id, name }
            }
            "moveworkspace" => {
                let (workspace, monitor) = parse_event_data!(data, WorkspaceName, MonitorName);
                HyprlandEvent::MoveWorkspace { workspace, monitor }
            }
            "moveworkspacev2" => {
                let (workspace_id, workspace, monitor) =
                    parse_event_data!(data, WorkspaceId, WorkspaceName, MonitorName);
                HyprlandEvent::MoveWorkspaceV2 {
                    workspace_id,
                    workspace,
                    monitor,
                }
            }
            "activespecial" => {
                let (workspace, monitor) = parse_event_data!(data, WorkspaceName, MonitorName);
                HyprlandEvent::ActiveSpecial { workspace, monitor }
            }
            "activelayout" => {
                let (keyboard, layout) = parse_event_data!(data, KeyboardName, LayoutName);
                HyprlandEvent::ActiveLayout { keyboard, layout }
            }
            "openwindow" => {
                let (address, workspace, class, title) =
                    parse_event_data!(data, WindowAddres, WorkspaceName, WindowClass, WindowTitle);
                HyprlandEvent::OpenWindow {
                    address,
                    workspace,
                    class,
                    title,
                }
            }
            "closewindow" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::CloseWindow { address }
            }
            "kill" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::Kill { address }
            }
            "movewindow" => {
                let (address, workspace) = parse_event_data!(data, WindowAddres, WorkspaceName);
                HyprlandEvent::MoveWindow { address, workspace }
            }
            "movewindowv2" => {
                let (address, workspace_id, workspace) =
                    parse_event_data!(data, WindowAddres, WorkspaceId, WorkspaceName);
                HyprlandEvent::MoveWindowV2 {
                    address,
                    workspace_id,
                    workspace,
                }
            }
            "openlayer" => {
                let namespace = parse_event_data!(data, Namespace);
                HyprlandEvent::OpenLayer { namespace }
            }
            "closelayer" => {
                let namespace = parse_event_data!(data, Namespace);
                HyprlandEvent::CloseLayer { namespace }
            }
            "submap" => {
                let submap = parse_event_data!(data, SubMapName);
                HyprlandEvent::SubMap { submap }
            }
            "changefloatingmode" => {
                let (address, floating) = parse_event_data!(data, WindowAddres, Floating);
                HyprlandEvent::ChangeFloatingMode { address, floating }
            }
            "urgent" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::Urgent { address }
            }
            "screencast" => {
                let (state, owner) = parse_event_data!(data, ScreenCastState, ScreenCastOwner);
                HyprlandEvent::ScreenCast { state, owner }
            }
            "screencastv2" => {
                let (state, owner, name) =
                    parse_event_data!(data, ScreenCastState, ScreenCastOwner, ScreenCastName);
                HyprlandEvent::ScreenCastV2 { state, owner, name }
            }
            "windowtitle" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::WindowTitle { address }
            }
            "windowtitlev2" => {
                let (address, title) = parse_event_data!(data, WindowAddres, WindowTitle);
                HyprlandEvent::WindowTitleV2 { address, title }
            }
            "togglegroup" => {
                let status = parse_event_data!(data, ToggleStatus);
                let addresses = data
                    .map(WindowAddres::try_from)
                    .collect::<Result<_, _>>()
                    .map_err(|_| EventParseError::InvalidData)?;
                HyprlandEvent::ToggleGroup { status, addresses }
            }
            "moveintogroup" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::MoveIntoGroup { address }
            }
            "moveoutofgroup" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::MoveOutOfGroup { address }
            }
            "ignoregrouplock" => {
                let ignore = parse_event_data!(data, IgnoreGroupLock);
                HyprlandEvent::IgnoreGroupLock { ignore }
            }
            "lockgroups" => {
                let lock = parse_event_data!(data, LockGroups);
                HyprlandEvent::LockGroups { lock }
            }
            "configreloaded" => HyprlandEvent::ConfigReloaded,
            "pin" => {
                let (address, state) = parse_event_data!(data, WindowAddres, PinState);
                HyprlandEvent::Pin { address, state }
            }
            "minimized" => {
                let (address, minimized) = parse_event_data!(data, WindowAddres, Minimized);
                HyprlandEvent::Minimized { address, minimized }
            }
            "bell" => {
                let address = parse_event_data!(data, WindowAddres);
                HyprlandEvent::Bell { address }
            }
            unknown => return Err(EventParseError::UnknownEvent(unknown.to_owned())),
        };

        Ok(event)
    }
}
