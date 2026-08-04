use std::rc::Rc;

use crate::language::{Environment, Function, FunctionCode, Library, TAIL_VAR, Value};

pub fn standard_library() -> Library {
    Library::from([("print", print_function())])
}

fn print_function() -> Function {
    let params = Vec::new();
    let print_impl = |environment: &mut Environment| -> Value {
        let null = Value::Null;
        let tail = environment.get_variable(TAIL_VAR).unwrap_or(&null);

        println!("{tail}");

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::Host(Rc::new(print_impl)),
    }
}
