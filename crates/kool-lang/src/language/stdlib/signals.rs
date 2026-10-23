use crate::{
    language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value},
    runner::{Signal, SignalSender},
};

pub(super) fn signals_functions(signal_sender: SignalSender) -> Library {
    Library::from([("refresh", refresh_function(signal_sender.clone()))])
}

fn refresh_function(signal_sender: SignalSender) -> Function {
    let params = FunctionParams::default();
    let refresh_impl = move |_environment: &SharedEnvironment| -> Value {
        let _ = signal_sender.unbounded_send(Signal::Refresh);

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(refresh_impl),
        closure: None,
    }
}
