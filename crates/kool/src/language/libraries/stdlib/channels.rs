use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};

pub(super) fn channels_functions() -> Library {
    Library::from([
        ("recv", recv_function()),
        ("tryRecv", try_recv_function()),
        ("send", send_function()),
        ("subscribe", subscribe_function()),
    ])
}

/// Blocks until a value comes in and returns it.
/// If an error occurs withing the inner channel, then `null` is returned.
fn recv_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let recv_impl = |environment: &SharedEnvironment| {
        let Some(Value::Channel(channel)) = environment.get_variable(TAIL_PARAM) else {
            return Value::Null;
        };

        match channel.recv() {
            Ok(value) => value,
            Err(_err) => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(recv_impl),
        closure: None,
    }
}

/// Tries to receive a value without blocking.
/// If a value is ready, then it is returned, otherwise `null` is returned.
fn try_recv_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let try_recv_impl = |environment: &SharedEnvironment| {
        let Some(Value::Channel(channel)) = environment.get_variable(TAIL_PARAM) else {
            return Value::Null;
        };

        match channel.try_recv() {
            Ok(value) => value,
            Err(_err) => Value::Null,
        }
    };

    Function {
        params,
        code: FunctionCode::new_host(try_recv_impl),
        closure: None,
    }
}

/// Sends a `value` into the channel for others to read.
fn send_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const VALUE_PARAM: &str = "value";

    let params = FunctionParams::default()
        .with_param(VALUE_PARAM, None)
        .with_tail(TAIL_PARAM);
    let send_impl = |environment: &SharedEnvironment| {
        let Some(value) = environment.get_variable(VALUE_PARAM) else {
            return Value::Null;
        };
        let Some(Value::Channel(channel)) = environment.get_variable(TAIL_PARAM) else {
            return Value::Null;
        };

        let _ = channel.send(value);

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(send_impl),
        closure: None,
    }
}

/// Subscribes to the incoming values of this channel.
/// Every time a value comes in the `callback` function is called with the new value.
/// This function spawns a background tasks, that watches the channel, and returns immediately.
fn subscribe_function() -> Function {
    const TAIL_PARAM: &str = "tail";
    const CALLBACK_PARAM: &str = "callback";

    let params = FunctionParams::default()
        .with_param(CALLBACK_PARAM, None)
        .with_tail(TAIL_PARAM);
    let subscribe_impl = |environment: &SharedEnvironment| {
        let Some(Value::Function(callback)) = environment.get_variable(CALLBACK_PARAM) else {
            return Value::Null;
        };
        let Some(Value::Channel(channel)) = environment.get_variable(TAIL_PARAM) else {
            return Value::Null;
        };

        let _handle = std::thread::spawn(move || {
            while let Ok(value) = channel.recv() {
                let mut args = callback.default_args();
                let _ = args.set_singular_arg(value);

                let _ = callback.call(args);
            }
        });

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(subscribe_impl),
        closure: None,
    }
}
