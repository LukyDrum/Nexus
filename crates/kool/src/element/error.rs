use crate::BuildContext;

/// An element that displays an error message.
/// Servers as the error type in `Result` returned from building widgets.
pub struct ErrorElement {
    message: String,
}

impl ErrorElement {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn build<'a>(&self, _context: &'a BuildContext) -> iced::widget::Text<'a> {
        iced::widget::text!("{}", self.message)
    }
}
