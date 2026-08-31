use std::time::Duration;

use crate::language::{Function, FunctionCode, FunctionParams, Library, SharedEnvironment, Value};

pub(super) fn task_functions() -> Library {
    Library::from([("spawn", spawn_function()), ("sleep", sleep_function())])
}

fn spawn_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let spawn_impl = |environment: &SharedEnvironment| -> Value {
        if let Some(Value::Function(func)) = environment.get_variable(TAIL_PARAM) {
            std::thread::spawn(move || {
                let _ = func.call_with_default_args();
            });
        }

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(spawn_impl),
        closure: None,
    }
}

fn sleep_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let sleep_impl = |environment: &SharedEnvironment| -> Value {
        if let Some(Value::Int(millis)) = environment.get_variable(TAIL_PARAM) {
            std::thread::sleep(Duration::from_millis(millis as u64));
        }

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(sleep_impl),
        closure: None,
    }
}
