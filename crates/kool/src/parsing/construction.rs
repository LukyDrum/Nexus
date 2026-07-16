use std::collections::HashMap;

use crate::{
    element::{KoolElement, kool},
    parsing::token::Value,
    scan_and_parse,
};

#[derive(Clone, Debug)]
pub enum ElementConstructionError<'a> {
    ExtraKeyValues(HashMap<&'a str, Value<'a>>),
    Import(String),
    InvalidElementContent(ElementContent<'a>),
    InvalidValueForKey { key: &'static str, value: Value<'a> },
    UnknownElement(&'a str),
}

pub(super) struct ElementInConstruction<'a> {
    pub ident: &'a str,
    pub values: HashMap<&'a str, Value<'a>>,
    pub inner: ElementContent<'a>,
}

#[derive(Clone, Debug)]
pub enum ElementContent<'a> {
    Empty,
    Value(Value<'a>),
    Element(KoolElement),
    Multiple(Vec<KoolElement>),
}

macro_rules! key_value {
    ($key:ident, $map:expr, $pat:pat => $value:ident) => {
        match $map.remove(stringify!($key)) {
            Some($pat) => $value.into(),
            Some(value) => {
                return Err(ElementConstructionError::InvalidValueForKey {
                    key: stringify!($key),
                    value,
                })
            }
            None => Default::default(),
        }
    };
}

macro_rules! content {
    ($content:expr, $pat:pat => $value:ident) => {
        if let $pat = $content {
            $value.into()
        } else {
            return Err(ElementConstructionError::InvalidElementContent($content));
        }
    };
}

impl<'a> TryFrom<ElementInConstruction<'a>> for KoolElement {
    type Error = ElementConstructionError<'a>;

    #[allow(unreachable_patterns)]
    fn try_from(
        ElementInConstruction {
            ident,
            mut values,
            inner,
        }: ElementInConstruction<'a>,
    ) -> Result<Self, Self::Error> {
        let element = match ident {
            "Import" => {
                let file = content!(inner, ElementContent::Value(Value::String(file)) => file);
                resolve_import(file)?
            }
            "Text" => Self::Text(kool::Text {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Value(Value::String(content)) => content),
            }),
            "Container" => Self::Container(kool::Container {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Element(content) => content),
            }),
            "Image" => Self::Image(kool::Image {
                key: key_value!(key, values, Value::String(string) => string),
                file: content!(inner, ElementContent::Value(Value::String(string)) => string),
            }),
            "Column" => Self::Column(kool::Column {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Multiple(content) => content),
            }),
            "Row" => Self::Row(kool::Row {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Multiple(content) => content),
            }),
            "Stack" => Self::Stack(kool::Stack {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Multiple(content) => content),
            }),
            _ => return Err(ElementConstructionError::UnknownElement(ident)),
        };

        if !values.is_empty() {
            return Err(ElementConstructionError::ExtraKeyValues(values));
        }

        Ok(element)
    }
}

// The stringification of the errors is not ideal... However it fixes the troubles with lifetimes.
fn resolve_import(filename: String) -> Result<KoolElement, ElementConstructionError<'static>> {
    let input = std::fs::read_to_string(filename)
        .map_err(|error| ElementConstructionError::Import(format!("{error}")))?;
    scan_and_parse(&input).map_err(|error| ElementConstructionError::Import(format!("{error:?}")))
}
