mod instance;
mod message;
#[expect(clippy::module_inception)]
mod runner;
mod settings;
mod signal;

pub use instance::{WidgetId, WidgetInstance};
pub use message::{ControlMessage, RunnerMessage, WidgetMessage};
pub use runner::KoolRunner;
pub use settings::WidgetSettings;
pub use signal::{
    Signal, SignalSender, get_signal_sender, init_signal_channel, take_signal_receiver,
};
