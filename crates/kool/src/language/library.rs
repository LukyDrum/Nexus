use std::{collections::HashMap, sync::Arc};

use crate::language::Function;

#[derive(Clone, Debug)]
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
