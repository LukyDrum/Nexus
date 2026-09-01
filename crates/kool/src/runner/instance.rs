use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use thiserror::Error;

use crate::{
    BuildContext,
    language::{Function, FunctionError, SharedEnvironment, StatementExecutionError, Value},
    parsing::ScanAndParserError,
    root_environment,
    runner::{WidgetMessage, WidgetSettings},
    scan_and_parse,
};

const INIT_FUNCTION: &str = "init";
const STYLE_FUNCTION: &str = "style";
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
    pub style_function: Arc<Function>,
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
        let style_function = if let Some(Value::Function(style_function)) =
            environment.get_variable(STYLE_FUNCTION)
        {
            style_function
        } else {
            Arc::new(Function::empty_function())
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
            style_function,
            view_function,
        })
    }

    pub fn update(&mut self, message: WidgetMessage) {
        match message {
            WidgetMessage::Callback(function) => {
                let _result = function.call_with_default_args();
            }
        }
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, WidgetMessage> {
        let Ok(Value::Element(element)) = self.view_function.call_with_default_args() else {
            return iced::widget::text("View function did not return an element").into();
        };

        let default_style = self
            .style_function
            .call_with_default_args()
            .unwrap_or_default()
            .into();
        let context = BuildContext { default_style };

        element.build(&context)
    }
}
