use std::sync::Mutex;

use iced::futures::channel::mpsc;
use koolctl::{ControlListener, ControlMessage, ControlStream};

pub(super) type ControlSender = mpsc::UnboundedSender<ControlMessage>;
pub(super) type ControlReceiver = mpsc::UnboundedReceiver<ControlMessage>;

static CONTROL_RECEIVER: Mutex<Option<ControlReceiver>> = Mutex::new(None);

/// Spawns a control server in a separate thread that listens on a Unix socket and returns a receiver for `ControlMessage`s.
pub(super) fn init_control_server() {
    let (sender, receiver) = mpsc::unbounded();

    std::thread::spawn(move || {
        let listener = ControlListener::new().expect("Failed to create listener");
        for stream in listener.incoming().flatten() {
            let sender = sender.clone();
            std::thread::spawn(|| handle_connection(stream, sender));
        }
    });

    *CONTROL_RECEIVER.lock().expect("Lock poisoned") = Some(receiver);
}

pub(super) fn take_control_receiver() -> ControlReceiver {
    CONTROL_RECEIVER
        .lock()
        .unwrap()
        .take()
        .expect("Control receiver not initialized or already taken")
}

fn handle_connection(mut stream: ControlStream, sender: ControlSender) {
    while let Ok(Some(message)) = stream.read() {
        let _ = sender.unbounded_send(message);
    }
}
