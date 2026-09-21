use std::{borrow::Cow, collections::HashMap, process::Command};

use hyprs::events::sync::HyprlandEvents;
use serde_json::Value as JsonValue;

use crate::language::{
    DuplexChannel, Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value,
};

pub(super) fn hyprland_library() -> Library {
    Library::from([
        ("events", events_function()),
        ("focusWorkspace", focus_workspace_function()),
    ])
}

fn events_function() -> Function {
    let params = FunctionParams::default();
    let events_impl = |_environment: &SharedEnvironment| -> Value {
        let events_channel = DuplexChannel::default();
        let events_channel_value = Value::Channel(events_channel.clone());

        let _handle = std::thread::spawn(move || {
            let Ok(mut hypr_events) = HyprlandEvents::new() else {
                return;
            };

            loop {
                let Ok(events) = hypr_events.read() else {
                    continue;
                };

                for event in events {
                    let Ok(json) = serde_json::to_value(event) else {
                        continue;
                    };
                    let JsonValue::Object(object) = json else {
                        continue;
                    };

                    let mut map = HashMap::new();
                    for (key, value) in object {
                        let value = match value {
                            JsonValue::String(string) => Value::new_string(string),
                            JsonValue::Number(number) => {
                                Value::Int(number.as_i64().unwrap_or_default())
                            }
                            JsonValue::Bool(bool) => Value::Bool(bool),
                            _ => continue,
                        };

                        map.insert(Value::new_string(key), value);
                    }

                    let _ = events_channel.send(Value::new_hash_map(map));
                }
            }
        });

        events_channel_value
    };

    Function {
        params,
        code: FunctionCode::new_host(events_impl),
        closure: None,
    }
}

fn focus_workspace_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let focus_workspace_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();
        let workspace = match &tail {
            Value::String(string) => Cow::Borrowed(string.as_str()),
            Value::Int(int) => Cow::Owned(int.to_string()),
            _ => return Value::Null,
        };

        let dispatch = format!("hl.dsp.focus({{ workspace = \"{workspace}\" }})");
        hyprctl_dispatch(&dispatch);

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(focus_workspace_impl),
        closure: None,
    }
}

fn hyprctl_dispatch(dispatch: &str) {
    let _ = Command::new("hyprctl")
        .arg("dispatch")
        .arg(dispatch)
        .spawn();
}
