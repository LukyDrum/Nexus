use crate::{
    language::{
        Expression, Function, FunctionCode, FunctionParams, Library, Operator, SharedEnvironment,
        Value,
    },
    utils::CloneInner,
};

pub(super) fn basic_functions() -> Library {
    Library::from([("print", print_function()), ("sum", sum_function())])
}

fn print_function() -> Function {
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default().with_tail(TAIL_PARAM);
    let print_impl = |environment: &SharedEnvironment| -> Value {
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or_default();

        println!("{tail}");

        Value::Null
    };

    Function {
        params,
        code: FunctionCode::new_host(print_impl),
        closure: None,
    }
}

fn sum_function() -> Function {
    const SEP_PARAM: &str = "sep";
    const TAIL_PARAM: &str = "tail";

    let params = FunctionParams::default()
        .with_param(SEP_PARAM, None)
        .with_tail(TAIL_PARAM);

    let sum_impl = |environment: &SharedEnvironment| -> Value {
        let sep = environment.get_variable(SEP_PARAM).unwrap_or(Value::Null);
        let tail = environment.get_variable(TAIL_PARAM).unwrap_or(Value::Null);

        let array = match tail {
            Value::Array(array) => array,
            single => return single,
        };

        // The only time reduce returns `None` is when the array was empty, then it makes sense to return an empty array as well.
        array
            .clone_inner()
            .into_iter()
            .reduce(|sum, value| {
                // If there is a separator than add the sum and the sep first
                let sum = if sep.is_null() {
                    sum
                } else {
                    let expr = Expression::Binary {
                        operator: Operator::Add,
                        left: Box::new(Expression::Value(sum)),
                        right: Box::new(Expression::Value(sep.clone())),
                    };

                    expr.evaluate_or_null(environment)
                };

                let expr = Expression::Binary {
                    operator: Operator::Add,
                    left: Box::new(Expression::Value(sum)),
                    right: Box::new(Expression::Value(value)),
                };
                expr.evaluate_or_null(environment)
            })
            .unwrap_or(Value::new_array(Vec::new()))
    };

    Function {
        params,
        code: FunctionCode::new_host(sum_impl),
        closure: None,
    }
}
