use crate::language::{Environment, Function, FunctionCode, FunctionParams, Library, Value};

pub fn standard_library() -> Library {
    Library::from([("print", print_function())])
}

fn print_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let print_impl = |environment: &mut Environment| -> Value {
        let null = Value::Null;
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or(&null);

        println!("{tail}");

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(print_impl),
    }
}
