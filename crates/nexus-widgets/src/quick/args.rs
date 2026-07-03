use crate::{quick::widget::QuickWidget, style::WidgetAppStyle};

#[derive(Debug, clap::Parser)]
pub(super) struct QuickArgs {
    #[arg(short, long, help = "Path to the style file in .toml format.")]
    style: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub(super) enum ConvertError {
    #[error("Error reading a file: {0}")]
    Io(std::io::Error),
    #[error("Error parsing a file: {0}")]
    Toml(toml::de::Error),
}

impl TryFrom<QuickArgs> for QuickWidget {
    type Error = ConvertError;

    fn try_from(value: QuickArgs) -> Result<Self, Self::Error> {
        let QuickArgs { style } = value;

        let style = if let Some(path) = style {
            let content = std::fs::read_to_string(path).map_err(ConvertError::Io)?;
            toml::from_str(&content).map_err(ConvertError::Toml)?
        } else {
            WidgetAppStyle::default_dark()
        };

        Ok(QuickWidget {
            name: "Quick Widget".to_owned(),
            style,
        })
    }
}
