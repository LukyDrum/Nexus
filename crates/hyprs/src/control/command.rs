use std::{borrow::Cow, fmt::Display};

use crate::control::selectors::Selector;

const DISPATCH: &str = "/dispatch";

const FOCUS_FUNCTION: &str = "hl.dsp.focus";

pub enum ControlCommand<FocusSelector = ()> {
    Focus(FocusSelector),
}

/// List of parts of the command to be joined
#[derive(Debug, Default)]
pub struct CommandParts(Vec<Cow<'static, str>>);

impl CommandParts {
    fn add(mut self, part: impl Into<Cow<'static, str>>) -> Self {
        self.0.push(part.into());
        self
    }
}

impl Display for CommandParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self.0.join(" ");
        write!(f, "{joined}")
    }
}

impl<FocusSelector> ControlCommand<FocusSelector>
where
    FocusSelector: Selector,
{
    pub fn parts(&self) -> CommandParts {
        match self {
            Self::Focus(selector) => CommandParts::default()
                .add(DISPATCH)
                .add(FOCUS_FUNCTION)
                .add("({")
                .add(selector.as_arg_pair_string())
                .add("})"),
        }
    }
}
