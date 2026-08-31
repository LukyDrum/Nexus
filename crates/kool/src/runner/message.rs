use std::{fmt::Debug, sync::Arc};

use iced_exwlshell::to_exwlshell_message;
use koolctl::ControlMessage;

use crate::{
    language::Function,
    runner::{Signal, WidgetId},
};

#[to_exwlshell_message]
#[derive(Clone, Debug)]
pub enum RunnerMessage {
    Control(ControlMessage),
    Signal(Signal),
    Widget {
        id: WidgetId,
        message: WidgetMessage,
    },
}

#[derive(Clone, Debug)]
pub enum WidgetMessage {
    Callback(Arc<Function>),
}
