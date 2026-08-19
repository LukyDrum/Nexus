use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    language::{Library, Value},
    utils::CloneInner,
};

#[derive(Clone, Debug, Default)]
pub struct SharedEnvironment {
    inner: Arc<RwLock<Environment>>,
}

impl SharedEnvironment {
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.inner.read().expect("Lock poisoned").get_variable(name)
    }

    pub fn define_variable(&self, name: String, value: Value) -> Option<Value> {
        self.inner
            .write()
            .expect("Lock poisoned")
            .define_variable(name, value)
    }

    pub fn set_variable(&self, name: &str, value: Value) -> Option<Value> {
        self.inner
            .write()
            .expect("Lock poisoned")
            .set_variable(name, value)
    }

    /// Clones the inner environment and converts it into a `Library`.
    pub fn as_library(&self) -> Library {
        self.inner.clone_inner().into_library()
    }

    pub fn import(&self, library: Library) {
        self.inner.write().expect("Lock poisoned").import(library);
    }

    pub fn import_namespace(&self, name: String, library: Library) {
        self.inner
            .write()
            .expect("Lock poisoned")
            .import_namespace(name, library);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Environment {
    outer: Option<SharedEnvironment>,
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new(outer: Option<SharedEnvironment>) -> Self {
        Self {
            outer,
            ..Default::default()
        }
    }

    pub fn into_shared(self) -> SharedEnvironment {
        SharedEnvironment {
            inner: Arc::new(RwLock::new(self)),
        }
    }

    /// Looks up and returns a variable.
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name).cloned() {
            return Some(value);
        }

        if let Some(outer) = &self.outer {
            return outer.get_variable(name);
        }

        None
    }

    /// Defines a variable and returns its previous value for error handling purposes.
    pub fn define_variable(&mut self, name: String, value: Value) -> Option<Value> {
        self.variables.insert(name, value)
    }

    /// Looks up a variable by its name in this or some of its enclosing environments and sets its value.
    /// Returns the old value of the variable.
    pub fn set_variable(&mut self, name: &str, value: Value) -> Option<Value> {
        if self.variables.contains_key(name) {
            return self.variables.insert(name.to_owned(), value);
        }

        if let Some(outer) = &self.outer {
            return outer.set_variable(name, value);
        }

        None
    }

    /// Converts the whole environment into a `Library`.
    pub fn into_library(self) -> Library {
        let values = self
            .variables
            .into_iter()
            .fold(HashMap::new(), |mut lib, (name, value)| {
                lib.insert(name, value);

                lib
            });

        Library { values }
    }

    /// Imports `library` into this environment.
    pub fn import(&mut self, library: Library) {
        let Library { values } = library;

        self.variables.extend(values);
    }

    /// Imports `library` as a namespace into this environment.
    pub fn import_namespace(&mut self, name: String, library: Library) {
        let library_values = library
            .values
            .into_iter()
            .map(|(key, value)| (Value::new_string(key), value))
            .collect();
        let map = Value::new_hash_map(library_values);

        self.variables.insert(name, map);
    }
}
