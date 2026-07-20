use crate::parsing::Metadata;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token<'a> {
    Ident(&'a str),   // eg. Text
    String(String),   // eg. "Hello world"
    Number(i64),      // eg. 42
    Variable(String), // eg. $count
    LeftParen,        // (
    RightParen,       // )
    LeftBracket,      // [
    RightBracket,     // ]
    Colon,            // :
    Equal,            // =
    Comma,            // ,
    Plus,             // +
    Minus,            // -
    Slash,            // /
    Star,             // *
    Caret,            // ^
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenWithMeta<'a> {
    pub token: Token<'a>,
    pub meta: Metadata,
}
