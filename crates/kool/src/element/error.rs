use crate::{
    BuildContext, Element,
    language::{Function, FunctionCode, FunctionParams, Value},
};

/// An element that displays an error message.
/// Servers as the error type in `Result` returned from building widgets.
#[derive(Clone, Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<'a> Element<'a> for Error {
    type IcedElement = iced::widget::Text<'a>;

    fn build(&self, _context: &BuildContext) -> Self::IcedElement {
        iced::widget::text!("{}", self.message)
    }

    fn kool_function() -> (&'static str, Function) {
        (
            "__Error",
            Function {
                params: FunctionParams::default(),
                code: FunctionCode::new_host(|_env| Value::Null),
                closure: None,
            },
        )
    }
}
