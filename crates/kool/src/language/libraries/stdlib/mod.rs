use crate::{
    elemental::SignalSender,
    language::{
        Library,
        libraries::stdlib::{
            basic::basic_functions, conversion::conversion_functions, signals::signals_functions,
            tasks::task_functions,
        },
    },
};

mod basic;
mod conversion;
mod signals;
mod tasks;

pub fn standard_library(signal_sender: SignalSender) -> Library {
    basic_functions()
        .merge(conversion_functions())
        .merge(signals_functions(signal_sender))
        .merge(task_functions())
}
