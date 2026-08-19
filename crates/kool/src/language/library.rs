use std::collections::HashMap;

use crate::language::{Function, Value};

#[derive(Clone, Debug, Default)]
pub struct Library {
    pub(super) values: HashMap<String, Value>,
}

impl<const N: usize> From<[(&str, Function); N]> for Library {
    fn from(array: [(&str, Function); N]) -> Self {
        let mut values = HashMap::new();
        for (name, function) in array {
            values.insert(name.to_owned(), Value::new_function(function));
        }

        Self { values }
    }
}

impl<const N: usize> From<[(&str, Value); N]> for Library {
    fn from(array: [(&str, Value); N]) -> Self {
        let mut values = HashMap::new();
        for (name, value) in array {
            values.insert(name.to_owned(), value);
        }

        Self { values }
    }
}

impl Library {
    pub fn extend(&mut self, other: Self) {
        self.values.extend(other.values);
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}
