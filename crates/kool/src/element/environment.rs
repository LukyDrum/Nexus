use std::{collections::HashMap, fmt::Display, process::Command, time::Duration};

#[derive(Clone, Debug, Default)]
pub struct Variables(HashMap<String, Value>);

impl Variables {
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }

    pub fn set(&mut self, name: &str, value: Value) {
        if let Some(old) = self.0.get_mut(name) {
            *old = value;
        } else {
            self.0.insert(name.to_owned(), value);
        }
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
    Number(i64),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Number(number) => write!(f, "{number}"),
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

impl RepeatingCommand {
    pub fn run_or_null(&self) -> Value {
        let output = Command::new("bash").arg("-c").arg(&self.command).output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8(output.stdout).map_or(Value::Null, Value::String)
                } else {
                    String::from_utf8(output.stderr).map_or(Value::Null, Value::String)
                }
            }
            Err(_) => Value::Null,
        }
    }
}
