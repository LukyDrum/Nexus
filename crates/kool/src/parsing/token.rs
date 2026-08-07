use crate::parsing::Metadata;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(String),  // eg. Text
    String(String), // eg. "Hello world"
    Int(i64),       // eg. 42
    LeftParen,      // (
    RightParen,     // )
    LeftBracket,    // [
    RightBracket,   // ]
    LeftBrace,      // {
    RightBrace,     // }
    Colon,          // :
    Equal,          // =
    Comma,          // ,
    Plus,           // +
    Minus,          // -
    Slash,          // /
    Star,           // *
    Caret,          // ^
    Bang,           // !
    EqualEqual,     // ==
    BangEqual,      // !=
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=
    And,            // &&
    Or,             // ||
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenWithMeta {
    pub token: Token,
    pub meta: Metadata,
}
