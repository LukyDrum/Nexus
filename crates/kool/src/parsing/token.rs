use crate::{element::environment::Value, parsing::Metadata};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    Ident(&'a str),   // eg. Text
    Value(Value),     // eg. "Hello world"
    Variable(String), // eg. $count
    LeftParen,        // (
    RightParen,       // )
    LeftBracket,      // [
    RightBracket,     // ]
    Colon,            // :
    Equal,            // =
    Comma,            // ,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenWithMeta<'a> {
    pub token: Token<'a>,
    pub meta: Metadata,
}
