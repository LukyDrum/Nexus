use std::{collections::HashMap, rc::Rc};

use crate::language::{Function, Value};

#[derive(Clone, Debug)]
pub struct Environment {
    scopes: Vec<Scope>,
}

#[derive(Clone, Debug, Default)]
struct Scope {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Rc<Function>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
        }
    }

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

    /// Looks up a function starting from the innermost scope outwards.
    pub fn get_function(&self, name: &str) -> Option<Rc<Function>> {
        for scope in self.scopes.iter().rev() {
            if let Some(function) = scope.functions.get(name) {
                return Some(function.clone());
            }
        }

        None
    }

    /// Sets function in the current (innermost) environment only.
    /// Returns the previous function defined under this name.
    pub fn define_function(&mut self, name: String, function: Function) -> Option<Rc<Function>> {
        self.scopes
            .last_mut()
            .expect("Scopes being empty should never happen")
            .functions
            .insert(name, Rc::new(function))
    }
}
