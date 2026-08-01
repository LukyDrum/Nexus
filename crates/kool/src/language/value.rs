use std::fmt::Display;

use crate::KoolElement;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Value {
    #[default]
    Null,
    Number(i64),
    String(String),
    Array(Vec<Value>),
    Element(Box<KoolElement>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Number(number) => write!(f, "{number}"),
            Value::String(string) => write!(f, "\"{string}\""),
            Value::Array(array) => {
                write!(f, "[")?;

                for value in array {
                    write!(f, "{value}, ")?;
                }

                write!(f, "]")
            }
            Value::Element(element) => write!(f, "<{}>", element.element_type()),
        }
    }
}
