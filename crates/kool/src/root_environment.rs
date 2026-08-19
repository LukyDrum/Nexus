use std::sync::OnceLock;

use tokio::sync::mpsc;

use crate::{
    element_library,
    elemental::Signal,
    language::{Environment, libraries::standard_library},
};

static SIGNAL_SENDER: OnceLock<mpsc::UnboundedSender<Signal>> = OnceLock::new();

pub fn init_signal_channel() -> mpsc::UnboundedReceiver<Signal> {
    let (sender, receiver) = mpsc::unbounded_channel();
    let _ = SIGNAL_SENDER.set(sender);

    receiver
}

pub fn root_environment() -> Environment {
    let mut root = Environment::default();

    if let Some(sender) = SIGNAL_SENDER.get() {
        root.import(standard_library(sender.clone()));
    }
    root.import(element_library());

    root
}
