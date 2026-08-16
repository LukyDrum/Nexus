use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type SignalSender = UnboundedSender<Signal>;
pub type SignalReceiver = UnboundedReceiver<Signal>;

/// A signal that the Kool language can produce and should be handled outside the runtime itself.
/// This ties together the language runtime and the widget runtime.
#[derive(Clone, Copy, Debug)]
pub enum Signal {
    Refresh,
}
