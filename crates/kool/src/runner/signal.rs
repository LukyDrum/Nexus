use std::sync::{Mutex, OnceLock};

use iced::futures::channel::mpsc;

pub type SignalSender = mpsc::UnboundedSender<Signal>;
pub type SignalReceiver = mpsc::UnboundedReceiver<Signal>;

static SIGNAL_SENDER: OnceLock<SignalSender> = OnceLock::new();
static SIGNAL_RECEIVER: Mutex<Option<SignalReceiver>> = Mutex::new(None);

pub fn init_signal_channel() {
    if SIGNAL_SENDER.get().is_none() {
        let (sender, receiver) = mpsc::unbounded();
        let _ = SIGNAL_SENDER.set(sender);
        *SIGNAL_RECEIVER.lock().expect("Lock poisoned") = Some(receiver);
    }
}

pub fn get_signal_sender() -> SignalSender {
    SIGNAL_SENDER
        .get()
        .expect("Signal channel should have been initialized")
        .clone()
}

pub fn take_signal_receiver() -> SignalReceiver {
    dbg!("receiver taken");
    SIGNAL_RECEIVER
        .lock()
        .unwrap()
        .take()
        .expect("Signal receiver not initialized or already taken")
}

/// A signal that the Kool language can produce and should be handled outside the runtime itself.
/// This ties together the language runtime and the widget runtime.
#[derive(Clone, Copy, Debug)]
pub enum Signal {
    Refresh,
}
