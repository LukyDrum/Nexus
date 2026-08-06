use std::{collections::HashMap, fmt::Display, hash::Hash, sync::Arc};

use crate::{KoolElement, language::Function};

#[derive(Clone, Debug, Default)]
pub enum Value {
    // Pass by value
    #[default]
    Null,
    Bool(bool),
    Number(i64),
    // Pass by reference
    String(Arc<String>),
    Array(Arc<Vec<Value>>),
    Element(Arc<KoolElement>),
    Function(Arc<Function>),
    HashMap(Arc<HashMap<Value, Value>>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(bool) => write!(f, "{bool}"),
            Value::Number(number) => write!(f, "{number}"),
            Value::String(string) => write!(f, "{string}"),
            Value::Array(array) => {
                write!(f, "[")?;

                for value in array.iter() {
                    write!(f, "{value}, ")?;
                }

                write!(f, "]")
            }
            Value::Element(element) => write!(f, "<{}>", element.element_type()),
            Value::Function(function) => write!(f, "func({:?})", &function.params),
            Value::HashMap(map) => {
                write!(f, "{{")?;

                for (key, value) in map.iter() {
                    write!(f, "{key}: {value}, ")?;
                }

                write!(f, "}}")
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Array(left), Self::Array(right)) => left == right,
            (Self::Element(left), Self::Element(right)) => left == right,
            (Self::Function(left), Self::Function(right)) => left == right,
            _ => false,
        }
    }
}
impl Eq for Value {}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Bool(bool) => bool.hash(state),
            Value::Number(num) => num.hash(state),
            Value::String(string) => string.hash(state),
            Value::Array(values) => values.hash(state),
            _ => core::mem::discriminant(&Self::Null).hash(state),
        }
    }
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null | Self::Number(0) => false,
            Self::Bool(bool) => *bool,
            _ => true,
        }
    }

    pub fn new_string(string: String) -> Self {
        Self::String(Arc::new(string))
    }

    pub fn new_array(array: Vec<Value>) -> Self {
        Self::Array(Arc::new(array))
    }

    pub fn new_element(element: KoolElement) -> Self {
        Self::Element(Arc::new(element))
    }

    pub fn new_function(function: Function) -> Self {
        Self::Function(Arc::new(function))
    }

    pub fn new_hash_map(map: HashMap<Value, Value>) -> Self {
        Self::HashMap(Arc::new(map))
    }
}
