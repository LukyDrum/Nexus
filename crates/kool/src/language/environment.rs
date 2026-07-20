use std::collections::HashMap;

use crate::language::Value;

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
