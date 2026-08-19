use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

use thiserror::Error;

use crate::{
    language::{Library, SharedEnvironment, StatementExecutionError, libraries::BUILTIN_LIBRARIES},
    parsing::ScanAndParserError,
    scan_and_parse,
};

static LIBRARY_CACHE: LazyLock<Mutex<HashMap<String, Library>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const KOOL_DIR: &str = "kool";
const CONFIG_DIR: &str = ".config";

#[derive(Clone, Debug, Error)]
pub enum LibraryLoadError {
    #[error("The library was not found")]
    NotFound,
    #[error("The library file could not be parsed: {0:?}")]
    ParsingFailed(ScanAndParserError),
    #[error("The library failed to execute: {0:?}")]
    ExecutionFailed(StatementExecutionError),
}

/// Libraries are searched for in the following order:
///     1. The library cache is checked,
///     2. The built-in Rust libraries are checked,
///     3. The relative directory is checked,
///     4. The user Kool config libraries directory is checked (usually `~/.config/kool/libraries`).
///
/// The first check that succeeds is returned.
pub fn load_library(name: &str) -> Result<Library, LibraryLoadError> {
    if let Some(cached) = LIBRARY_CACHE
        .lock()
        .ok()
        .and_then(|cache| cache.get(name).cloned())
    {
        return Ok(cached);
    }

    if let Some(builtin) = BUILTIN_LIBRARIES.get(name) {
        return Ok(builtin.clone());
    }

    let local_file = format!("./{name}.kool");
    match try_load_from_file(&PathBuf::from(local_file)) {
        Err(LibraryLoadError::NotFound) => {}
        result => return result,
    }

    let user_file = user_library_dir().join(format!("{name}.kool"));
    try_load_from_file(&user_file)
}

fn user_library_dir() -> PathBuf {
    env::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(CONFIG_DIR)
        .join(KOOL_DIR)
}

fn try_load_from_file(file: &Path) -> Result<Library, LibraryLoadError> {
    let input = fs::read_to_string(file).map_err(|_| LibraryLoadError::NotFound)?;
    let source = scan_and_parse(&input).map_err(LibraryLoadError::ParsingFailed)?;

    let environment = SharedEnvironment::default();
    let _ = source
        .execute(&environment)
        .map_err(LibraryLoadError::ExecutionFailed);

    Ok(environment.as_library())
}
