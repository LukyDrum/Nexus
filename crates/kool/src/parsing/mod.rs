//! # Grammar
//!
//! This grammar is meant to represent every possible element definition.
//! However, not every element produced by this grammar is valid in the Kool context.
//! Only a finite subset of the values produced by this grammar are can be represented as Kool elements.
//! This subset can change over time as new elements are added, or removed, from the Kool elements.
//!
//! ```
//! element      -> Ident ( body )
//! body         -> keyPair, body
//!              -> content
//! keyPair      -> Ident = value
//! value        -> String
//! content      -> element
//!              -> value
//!              -> [ multiContent ]
//! multiContent -> content, multiContent
//!              -> content
//! ```

use crate::{
    element::KoolElement,
    parsing::{
        parser::{ParserContext, ParserError, parse},
        scanner::{ScannerError, scan},
    },
};

mod construction;
mod parser;
mod scanner;
mod token;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub enum ParsingError<'a> {
    Scanner(ScannerError),
    Parser(ParserError<'a>),
}

pub fn scan_and_parse<'a>(
    input: &'a str,
) -> Result<(ParserContext, KoolElement), ParsingError<'a>> {
    let tokens = scan(input).map_err(ParsingError::Scanner)?;

    let mut context = ParserContext::default();
    let element = parse(tokens.into_iter(), &mut context).map_err(ParsingError::Parser)?;

    Ok((context, element))
}
