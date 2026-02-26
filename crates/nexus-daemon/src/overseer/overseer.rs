use nexus_api::{NexusMessage, WorkspaceNumber};

use crate::overseer::NexusState;

/// [`Overseer`] acts as the middle man between the tasks that listen for incoming messages, for Hyprland event and so on.
/// It is responsible for reacting to any events and keeping the state of Nexus in coherent state.
pub(crate) struct Overseer {
    state: NexusState,
}

impl Overseer {
    pub fn new(state: NexusState) -> Self {
        Self { state }
    }

    pub fn process_nexus_message(&mut self, message: NexusMessage) {
        match message {
            NexusMessage::SwitchWorkspace(number) => todo!(),
            NexusMessage::SwitchGroup(name) => todo!(),
            NexusMessage::SwitchToHub => todo!(),
        }
    }

    /* Bellow lay the implementations of the individual reactions */

    /// Switches to a workspace (identified by number) in the current group.
    fn switch_workspace(&mut self, number: WorkspaceNumber) {}

    fn switch_group(&mut self, name: String) {}

    fn switch_to_hub(&mut self) {}
}
