use crate::{
    language::{
        Library,
        libraries::stdlib::{
            basic::basic_functions, channels::channels_functions,
            collections::collections_functions, conversion::conversion_functions,
            format::format_library, math::math_functions, signals::signals_functions,
            string_and_array::string_and_array_functions, tasks::task_functions,
        },
    },
    runner::SignalSender,
};

mod basic;
mod channels;
mod collections;
mod conversion;
mod format;
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
        .merge(format_library())
        .merge(collections_functions())
        .merge(channels_functions())
}
