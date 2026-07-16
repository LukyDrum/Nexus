use std::process::Command;

use crate::element::{BuildContext, KoolBuilder, kool};

const ERROR: &str = "<ERROR>";

#[derive(Clone, Debug)]
pub struct Output {
    pub key: String,
    pub command: String,
}

impl KoolBuilder for Output {
    type IcedElement = iced::widget::Text<'static>;

    fn build(&self, context: BuildContext) -> Self::IcedElement {
        let output = Command::new("bash").arg("-c").arg(&self.command).output();
        let output = match output {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8(output.stdout).unwrap_or(ERROR.to_owned())
                } else {
                    String::from_utf8(output.stderr).unwrap_or(ERROR.to_owned())
                }
            }
            Err(_) => ERROR.to_owned(),
        };

        let widget = kool::Text {
            key: self.key.clone(),
            content: output,
        };

        widget.build(context)
    }
}
