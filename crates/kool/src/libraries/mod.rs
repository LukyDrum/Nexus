mod hyprland;
mod mpris;
mod stdlib;

use std::{collections::HashMap, sync::LazyLock};

pub use stdlib::standard_library;

use crate::language::Library;

pub static BUILTIN_LIBRARIES: LazyLock<HashMap<&'static str, Library>> = LazyLock::new(|| {
    HashMap::from([
        ("mpris", mpris::mpris_library()),
        ("hyprland", hyprland::hyprland_library()),
    ])
});
