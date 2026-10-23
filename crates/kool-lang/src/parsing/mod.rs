//! # Grammar
//!
//! This grammar is meant to represent every possible element definition.
//! However, not every element produced by this grammar is valid in the Kool context.
//! Only a finite subset of the values produced by this grammar are can be represented as Kool elements.
//! This subset can change over time as new elements are added, or removed, from the Kool elements.
//!
//! ```
//! start        -> varDef start
//!              -> element
//! varDef       -> var Ident = value
//! element      -> Ident ( body )
//! body         -> keyPair, body
//!              -> content
//! keyPair      -> Ident: value
//! value        -> String
//! content      -> element
//!              -> value
//!              -> [ multiContent ]
//! multiContent -> content, multiContent
//!              -> content
//! ```

use crate::{
    language::StatementBlock,
    parsing::{
        parser::{ParserError, parse},
        scanner::{ScannerError, scan},
    },
};

mod multi_peek;
mod parser;
mod scanner;
mod token;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub enum ScanAndParserError {
    Scanner(ScannerError),
    Parser(ParserError),
}

pub fn scan_and_parse(input: &str) -> Result<StatementBlock, ScanAndParserError> {
    let tokens = scan(input).map_err(ScanAndParserError::Scanner)?;
    let root = parse(tokens.into_iter()).map_err(ScanAndParserError::Parser)?;

    Ok(root)
}
