mod action;
mod ctl;
mod response;

pub(crate) use action::HyprlandAction;
pub(crate) use ctl::hyprctl_eval;
pub(crate) use response::{HyprlandClient, HyprlandResponse};
