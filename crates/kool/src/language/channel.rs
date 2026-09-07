use crossbeam::channel::{Receiver, RecvError, SendError, Sender, TryRecvError, unbounded};

use crate::language::Value;

#[derive(Clone, Debug)]
pub struct DuplexChannel {
    sender: Sender<Value>,
    receiver: Receiver<Value>,
}

impl Default for DuplexChannel {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }
}

impl DuplexChannel {
    pub fn send(&self, value: Value) -> Result<(), SendError<Value>> {
        self.sender.send(value)
    }

    pub fn recv(&self) -> Result<Value, RecvError> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Result<Value, TryRecvError> {
        self.receiver.try_recv()
    }
}
