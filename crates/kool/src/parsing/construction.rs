use std::collections::HashMap;

use crate::element::environment::Value;
use crate::parsing::parser::ParserContext;
use crate::{
    element::{KoolElement, kool},
    scan_and_parse,
};

#[derive(Clone, Debug)]
pub enum ElementConstructionError<'a> {
    ExtraKeyValues(HashMap<&'a str, Value>),
    Import(String),
    InvalidElementContent(ElementContent),
    InvalidValueForKey { key: &'static str, value: Value },
    UnknownElement(&'a str),
}

pub(super) struct ElementInConstruction<'a> {
    pub ident: &'a str,
    pub values: HashMap<&'a str, Value>,
    pub inner: ElementContent,
}

#[derive(Clone, Debug)]
pub enum ElementContent {
    Empty,
    Value(Value),
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

impl<'a> ElementInConstruction<'a> {
    pub(super) fn try_construct(
        self,
        context: &mut ParserContext,
    ) -> Result<KoolElement, ElementConstructionError<'a>> {
        let ElementInConstruction {
            ident,
            mut values,
            inner,
        } = self;

        #[expect(unreachable_patterns, reason = "The variants will grow in future")]
        let element = match ident {
            "Import" => {
                let file = content!(inner, ElementContent::Value(Value::String(file)) => file);
                resolve_import(file, context)?
            }
            "Text" => KoolElement::Text(kool::Text {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Value(Value::String(content)) => content),
            }),
            "Container" => KoolElement::Container(kool::Container {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Element(content) => content),
            }),
            "Image" => KoolElement::Image(kool::Image {
                key: key_value!(key, values, Value::String(string) => string),
                file: content!(inner, ElementContent::Value(Value::String(string)) => string),
            }),
            "Column" => KoolElement::Column(kool::Column {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Multiple(content) => content),
            }),
            "Row" => KoolElement::Row(kool::Row {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Multiple(content) => content),
            }),
            "Stack" => KoolElement::Stack(kool::Stack {
                key: key_value!(key, values, Value::String(string) => string),
                content: content!(inner, ElementContent::Multiple(content) => content),
            }),
            "Output" => KoolElement::Output(kool::Output {
                key: key_value!(key, values, Value::String(string) => string),
                command: content!(inner, ElementContent::Value(Value::String(string)) => string),
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
fn resolve_import(
    filename: String,
    context: &mut ParserContext,
) -> Result<KoolElement, ElementConstructionError<'static>> {
    let input = std::fs::read_to_string(filename)
        .map_err(|error| ElementConstructionError::Import(format!("{error}")))?;
    let (imported_context, element) = scan_and_parse(&input)
        .map_err(|error| ElementConstructionError::Import(format!("{error:?}")))?;

    context.extend(imported_context);

    Ok(element)
}
