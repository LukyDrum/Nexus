use std::fmt::Debug;

use iced_layershell::to_layer_message;

#[to_layer_message]
#[derive(Clone, Debug)]
pub enum LayerShellAppMessage<Msg: Clone + Debug + Send> {
    AppMessage(Msg),
}
