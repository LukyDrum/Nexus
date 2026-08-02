use std::rc::Rc;

use crate::language::{Environment, Function, FunctionCode, TAIL_VAR_NAME, Value};

pub fn standard_environment() -> Environment {
    let mut environment = Environment::new();

    environment.define_function("print".to_owned(), print_function());

    environment
}

fn print_function() -> Function {
    let params = Vec::new();
    let print_impl = |environment: &mut Environment| -> Value {
        let null = Value::Null;
        let tail = environment.get_variable(TAIL_VAR_NAME).unwrap_or(&null);

        println!("{tail}");

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::Host(Rc::new(print_impl)),
    }
}
