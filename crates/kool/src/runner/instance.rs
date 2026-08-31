use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use thiserror::Error;

use crate::{
    language::{Function, FunctionError, SharedEnvironment, StatementExecutionError, Value},
    parsing::ScanAndParserError,
    root_environment,
    runner::WidgetSettings,
    scan_and_parse,
};

const INIT_FUNCTION: &str = "init";
const VIEW_FUNCTION: &str = "view";

/// A widget is identified by its script.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WidgetId(PathBuf);

impl WidgetId {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

#[derive(Debug)]
pub struct WidgetInstance {
    pub id: WidgetId,
    // We need to hold on to this
    pub _environment: SharedEnvironment,
    pub settings: WidgetSettings,
    pub view_function: Arc<Function>,
}

#[derive(Debug, Error)]
pub enum WidgetError {
    #[error("Failed to read Kool source: {0}")]
    Io(std::io::Error),
    #[error("Failed to parse Kool source: {0:?}")]
    Parse(ScanAndParserError),
    #[error("Failed to execute Kool source: {0:?}")]
    Execution(StatementExecutionError),
    #[error("Failed to execute Kool function: {0:?}")]
    FunctionExecution(FunctionError),
    #[error("The init function is missing")]
    MissingInit,
    #[error("The view function is missing")]
    MissingView,
}

impl WidgetInstance {
    pub fn init(id: WidgetId) -> Result<Self, WidgetError> {
        let name = id
            .path()
            .file_stem()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or("none".to_owned());

        let source = std::fs::read_to_string(id.path()).map_err(WidgetError::Io)?;
        let source = scan_and_parse(&source).map_err(WidgetError::Parse)?;

        let environment = root_environment().into_shared();
        let _ = source
            .execute(&environment)
            .map_err(WidgetError::Execution)?;

        let Some(Value::Function(init_function)) = environment.get_variable(INIT_FUNCTION) else {
            return Err(WidgetError::MissingInit);
        };
        let Some(Value::Function(view_function)) = environment.get_variable(VIEW_FUNCTION) else {
            return Err(WidgetError::MissingView);
        };

        let settings = init_function
            .call_with_default_args()
            .map_err(WidgetError::FunctionExecution)?;
        let mut settings = WidgetSettings::from(settings);
        settings.name = Some(name);

        Ok(Self {
            id,
            _environment: environment,
            settings,
            view_function,
        })
    }
}
