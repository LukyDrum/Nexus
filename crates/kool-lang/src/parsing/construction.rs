use std::collections::HashMap;
use std::time::Duration;

use crate::element::{KoolElement, kool};
use crate::elemental::RepeatingCommand;
use crate::language::{EvaluationError, Expression, Value};
use crate::parsing::parser::ParserContext;
use crate::parsing::scan_and_parse_with_context;

#[derive(Clone, Debug)]
pub enum ElementConstructionError<'a> {
    ExtraKeyValues(HashMap<&'a str, Expression>),
    Evaluation(EvaluationError),
    Import(String),
    InvalidElementContent(Expression),
    InvalidValueForKey { key: &'static str, value: Value },
    UnknownElement(&'a str),
}

pub(super) struct ElementInConstruction<'a> {
    pub ident: &'a str,
    pub values: HashMap<&'a str, Expression>,
    pub content: Expression,
}

/// For now, key values pairs must be evaluateable at "parse time".
macro_rules! key_value {
    ($key:ident, $map:expr, $variables:expr, $pat:pat => $value:ident) => {
        key_value!($key, $map, $variables, $pat => $value, Default::default())
    };
    ($key:ident, $map:expr, $variables:expr, $pat:pat => $value:ident, $default:expr) => {
         match $map.remove(stringify!($key)) {
             Some(expr) => {
                 let value = expr.evaluate($variables).map_err(ElementConstructionError::Evaluation)?;
                 if let $pat = value {
                     $value.into()
                 } else {
                     return Err(ElementConstructionError::InvalidValueForKey {
                         key: stringify!($key),
                         value,
                     });
                 }
             }
             None => $default,
         }
     };
}

macro_rules! content {
    ($content:expr, $pat:pat => $value:ident) => {
        if let $pat = $content {
            $value.into()
        } else {
            return Err(ElementConstructionError::InvalidElementContent(
                $content.into(),
            ));
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
            content,
        } = self;

        let element = match ident {
            /* SPECIAL */
            "Import" => {
                let evaluated = content
                    .evaluate(&context.variables)
                    .map_err(ElementConstructionError::Evaluation)?;
                let file = content!(evaluated, Value::String(file) => file);
                resolve_import(file, context)?
            }
            "Output" => {
                let refresh: i64 = key_value!(refresh, values, &context.variables, Value::Number(refresh) => refresh, 0);

                let command_evaluated = content
                    .evaluate(&context.variables)
                    .map_err(ElementConstructionError::Evaluation)?;
                let command = content!(command_evaluated, Value::String(command) => command);

                let internal_var = format!("__output_{}", context.claim_id());
                context.variables.set(&internal_var, Value::Null);
                context.commands.push(RepeatingCommand {
                    period: (refresh > 0).then(|| Duration::from_secs(refresh as u64)),
                    variable: internal_var.clone(),
                    command,
                });

                KoolElement::Text(kool::Text {
                    key: key_value!(key, values, &context.variables, Value::String(string) => string),
                    content: Expression::Variable(internal_var),
                })
            }

            /* REGULAR */
            "Text" => KoolElement::Text(kool::Text {
                key: key_value!(key, values, &context.variables, Value::String(string) => string),
                content,
            }),
            "Container" => KoolElement::Container(kool::Container {
                key: key_value!(key, values, &context.variables, Value::String(string) => string),
                content,
            }),
            "Image" => KoolElement::Image(kool::Image {
                key: key_value!(key, values, &context.variables, Value::String(string) => string),
                file: content,
            }),
            "Column" => KoolElement::Column(kool::Column {
                key: key_value!(key, values, &context.variables, Value::String(string) => string),
                content,
            }),
            "Row" => KoolElement::Row(kool::Row {
                key: key_value!(key, values, &context.variables, Value::String(string) => string),
                content,
            }),
            "Stack" => KoolElement::Stack(kool::Stack {
                key: key_value!(key, values, &context.variables, Value::String(string) => string),
                content,
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

    scan_and_parse_with_context(&input, context)
        .map_err(|error| ElementConstructionError::Import(format!("{error:?}")))
}
