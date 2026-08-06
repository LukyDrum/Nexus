use std::collections::HashMap;

use crate::language::{Library, Value};

#[derive(Clone, Debug)]
pub struct Environment {
    scopes: Vec<Scope>,
}

#[derive(Clone, Debug, Default)]
struct Scope {
    variables: HashMap<String, Value>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::default()],
        }
    }
}

impl Environment {
    /// Pushes a new scope onto the stack.
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    /// Pops the top scope off the stack.
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Looks up a variable starting from the innermost scope.
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.variables.get(name) {
                return Some(value);
            }
        }

        None
    }

    /// Sets a variable in the current (innermost) environment only.
    /// Returns the previous value of this variable.
    pub fn define_variable(&mut self, name: String, value: Value) -> Option<Value> {
        self.scopes
            .last_mut()
            .expect("Scopes being empty should never happen")
            .variables
            .insert(name, value)
    }

    /// Looks up the variable in any scope, sets it to a new value, and returns a reference to its value.
    pub fn set_variable(&mut self, name: &str, value: Value) -> Option<&Value> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(variable) = scope.variables.get_mut(name) {
                *variable = value;
                return Some(variable);
            }
        }

        None
    }

    /// Converts the whole environment into a `Library`.
    /// Libraries contain only functions.
    pub fn into_library(self) -> Library {
        let functions = self
            .scopes
            .into_iter()
            .flat_map(|scope| scope.variables.into_iter())
            .fold(HashMap::new(), |mut lib, (name, value)| {
                if let Value::Function(function) = value {
                    lib.insert(name, function);
                }

                lib
            });

        Library { functions }
    }

    /// Imports `library` into the inner most scope.
    pub fn import(&mut self, library: Library) {
        let scope = self
            .scopes
            .last_mut()
            .expect("Scopes being empty should never happen");

        let Library { functions } = library;
        let functions = functions
            .into_iter()
            .map(|(name, function)| (name, Value::Function(function)));

        scope.variables.extend(functions);
    }
}
