use crate::language::{
    Library,
    stdlib::{basic::basic_functions, conversion::conversion_functions},
};

mod basic;
mod conversion;

pub fn standard_library() -> Library {
    basic_functions().merge(conversion_functions())
}
