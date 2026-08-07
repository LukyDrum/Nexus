use std::{collections::HashMap, sync::Arc};

use crate::language::Function;

#[derive(Clone, Debug, Default)]
pub struct Library {
    pub(super) functions: HashMap<String, Arc<Function>>,
}

impl<const N: usize> From<[(&str, Function); N]> for Library {
    fn from(values: [(&str, Function); N]) -> Self {
        let mut functions = HashMap::new();
        for (name, function) in values {
            functions.insert(name.to_owned(), Arc::new(function));
        }

        Self { functions }
    }
}

impl Library {
    pub fn extend(&mut self, other: Self) {
        self.functions.extend(other.functions);
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.extend(other);
        self
    }
}
