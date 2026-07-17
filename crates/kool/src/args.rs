use std::{num::ParseIntError, str::FromStr};

use crate::element::environment::Value;

#[derive(Clone, Debug, thiserror::Error)]
pub enum ArgsError {
    #[error("Missing '=' sign.")]
    EqualSignMissing,
    #[error("No name for the variable.")]
    NameIsEmpty,
    #[error("Parse int error: {0}")]
    Parse(ParseIntError),
}

#[derive(Debug, clap::Parser)]
pub struct ElementalArgs {
    #[arg(short, long, help = "Path to a config file in .toml format.")]
    pub config: String,
    #[arg(
        short,
        long,
        help = "Variables to be passed in format 'varName = value'."
    )]
    pub var: Vec<NameValuePair>,
}

#[derive(Clone, Debug)]
pub struct NameValuePair {
    pub name: String,
    pub value: Value,
}

impl FromStr for NameValuePair {
    type Err = ArgsError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let Some((name, value)) = string.split_once("=") else {
            return Err(ArgsError::EqualSignMissing);
        };
        let name = name.trim();
        let value = value.trim();

        if name.is_empty() {
            return Err(ArgsError::NameIsEmpty);
        }

        let value = if value.is_empty() {
            Value::Null
        } else if value.chars().all(|char| char.is_ascii_digit()) {
            Value::Number(value.parse().map_err(ArgsError::Parse)?)
        } else {
            Value::String(value.to_owned())
        };

        Ok(Self {
            name: name.to_owned(),
            value,
        })
    }
}
