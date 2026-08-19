mod stdlib;

use std::{collections::HashMap, sync::LazyLock};

pub use stdlib::standard_library;

use crate::language::Library;

pub static BUILTIN_LIBRARIES: LazyLock<HashMap<&'static str, Library>> =
    LazyLock::new(|| HashMap::from([]));
