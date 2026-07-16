use std::{collections::HashMap, fmt::Display, time::Duration};

#[derive(Clone, Debug, Default)]
pub struct Variables(HashMap<String, Value>);

impl Variables {
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.0.insert(name, value);
    }

    pub fn extend(&mut self, other: Variables) {
        self.0.extend(other.0);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnresolvedValue {
    Variable(String),
    Value(Value),
}

impl UnresolvedValue {
    pub fn resolve<'a>(&'a self, variables: &'a Variables) -> Option<&'a Value> {
        match self {
            Self::Value(value) => Some(value),
            Self::Variable(var) => variables.get(var),
        }
    }

    pub fn resolve_or_null<'a>(&'a self, variables: &'a Variables) -> Value {
        self.resolve(variables).cloned().unwrap_or(Value::Null)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Null,
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::String(string) => write!(f, "{string}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RepeatingCommand {
    pub period: Duration,
    pub variable: String,
    pub command: String,
}
