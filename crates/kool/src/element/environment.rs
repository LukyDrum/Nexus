use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    String(String),
}

#[derive(Clone, Debug)]
pub struct RepeatingCommand {
    pub period: Duration,
    pub variable: String,
    pub command: String,
}
