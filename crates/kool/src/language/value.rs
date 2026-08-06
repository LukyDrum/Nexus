use std::{fmt::Display, sync::Arc};

use crate::{KoolElement, language::Function};

#[derive(Clone, Debug, Default)]
pub enum Value {
    // Pass by value
    #[default]
    Null,
    Number(i64),
    // Pass by reference
    String(Arc<String>),
    Array(Arc<Vec<Value>>),
    Element(Arc<KoolElement>),
    Function(Arc<Function>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
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
            Value::Function(functon) => write!(f, "func({:?})", &functon.params),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
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

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
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
}
