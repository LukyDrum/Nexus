use crate::{
    elemental::SignalSender,
    language::{
        Library,
        libraries::stdlib::{
            basic::basic_functions, conversion::conversion_functions, math::math_functions,
            signals::signals_functions, string_and_array::string_and_array_functions,
            tasks::task_functions,
        },
    },
};

mod basic;
mod conversion;
mod math;
mod signals;
mod string_and_array;
mod tasks;

pub fn standard_library(signal_sender: SignalSender) -> Library {
    basic_functions()
        .merge(conversion_functions())
        .merge(signals_functions(signal_sender))
        .merge(task_functions())
        .merge(math_functions())
        .merge(string_and_array_functions())
}
