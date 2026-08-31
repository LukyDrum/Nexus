use crate::{
    element_library,
    language::{Environment, libraries::standard_library},
    runner::get_signal_sender,
};

/// The base root environment preloaded with standard library and element library.
pub fn root_environment() -> Environment {
    let mut root = Environment::default();

    root.import(standard_library(get_signal_sender()));
    root.import(element_library());

    root
}
