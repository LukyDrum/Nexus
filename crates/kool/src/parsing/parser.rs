use std::{collections::HashMap, iter::Peekable};

use crate::{
    element::{
        KoolElement,
        environment::{RepeatingCommand, Value},
    },
    parsing::{
        construction::{ElementConstructionError, ElementContent, ElementInConstruction},
        token::{Token, TokenWithMeta},
    },
};

#[derive(Clone, Debug)]
pub enum ParserError<'a> {
    Construction(ElementConstructionError<'a>),
    UnexpectedToken(TokenWithMeta<'a>),
    UnexpectedEof { expected: String },
}

macro_rules! match_token {
    ($token:expr, $pat:pat => $value:ident, expected = $expected:expr) => {
        match $token {
            Some(TokenWithMeta {
                token: $pat,
                meta: _,
            }) => $value,
            Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: $expected.to_owned(),
                })
            }
        }
    };
    ($token:expr, $expected:expr) => {
        match $token {
            Some(TokenWithMeta { token, meta: _ }) if token == $expected => {}
            Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: format!("{:?}", $expected),
                })
            }
        }
    };
}

#[derive(Debug, Default)]
pub struct ParserContext {
    pub variables: HashMap<String, Value>,
    pub commands: Vec<RepeatingCommand>,
}

impl ParserContext {
    pub fn extend(&mut self, other: ParserContext) {
        let ParserContext {
            variables,
            commands,
        } = other;

        self.variables.extend(variables.into_iter());
        self.commands.extend(commands.into_iter());
    }
}

pub(super) fn parse<'a>(
    tokens: impl Iterator<Item = TokenWithMeta<'a>>,
    context: &mut ParserContext,
) -> Result<KoolElement, ParserError<'a>> {
    let mut tokens = tokens.peekable();

    let ident =
        match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "Element identifier");
    parse_element(&mut tokens, ident, context)
}

fn parse_element<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
    ident: &'a str,
    context: &mut ParserContext,
) -> Result<KoolElement, ParserError<'a>> {
    match_token!(tokens.next(), Token::LeftParen);

    let mut element = ElementInConstruction {
        ident,
        values: HashMap::new(),
        inner: ElementContent::Empty,
    };
    parse_body(tokens, &mut element, context)?;

    match_token!(tokens.next(), Token::RightParen);

    element
        .try_construct(context)
        .map_err(ParserError::Construction)
}

fn parse_body<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
    element: &mut ElementInConstruction<'a>,
    context: &mut ParserContext,
) -> Result<(), ParserError<'a>> {
    // Possibly parse things needing to only know the first token
    let ident = match tokens.next() {
        Some(TokenWithMeta {
            token: Token::Ident(ident),
            meta: _,
        }) => ident,
        Some(TokenWithMeta {
            token: Token::LeftBracket,
            meta: _,
        }) => {
            let elements = parse_elements_list(tokens, context)?;
            element.inner = ElementContent::Multiple(elements);
            return Ok(());
        }
        Some(TokenWithMeta {
            token: Token::Value(value),
            meta: _,
        }) => {
            element.inner = ElementContent::Value(value);
            return Ok(());
        }
        Some(token) => return Err(ParserError::UnexpectedToken(token)),
        None => {
            return Err(ParserError::UnexpectedEof {
                expected: "Element identifier, key identifier, value or a list".to_owned(),
            });
        }
    };

    // Parse stuff needing to know the next token
    match tokens.peek() {
        Some(TokenWithMeta {
            token: Token::Equal,
            meta: _,
        }) => {
            parse_key_value(tokens, element, ident, context)?;
            parse_body(tokens, element, context)
        }
        Some(TokenWithMeta {
            token: Token::LeftParen,
            meta: _,
        }) => {
            let inner_element = parse_element(tokens, ident, context)?;
            element.inner = ElementContent::Element(inner_element);
            Ok(())
        }
        Some(token) => Err(ParserError::UnexpectedToken(token.clone())),
        None => Err(ParserError::UnexpectedEof {
            expected: "Key value pair or Element body".to_owned(),
        }),
    }
}

fn parse_key_value<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
    element: &mut ElementInConstruction<'a>,
    key: &'a str,
    _context: &mut ParserContext,
) -> Result<(), ParserError<'a>> {
    match_token!(tokens.next(), Token::Equal);
    let value = match_token!(tokens.next(), Token::Value(value) => value, expected = "Some value");

    element.values.insert(key, value);

    // Discard possible comma
    let _ = tokens.next_if(|TokenWithMeta { token, meta: _ }| matches!(token, Token::Comma));

    Ok(())
}

fn parse_elements_list<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
    context: &mut ParserContext,
) -> Result<Vec<KoolElement>, ParserError<'a>> {
    let mut elements = Vec::new();
    loop {
        match tokens.next() {
            Some(TokenWithMeta {
                token: Token::Comma,
                meta: _,
            }) => continue,
            Some(TokenWithMeta {
                token: Token::RightBracket,
                meta: _,
            }) => return Ok(elements),
            Some(TokenWithMeta {
                token: Token::Ident(ident),
                meta: _,
            }) => elements.push(parse_element(tokens, ident, context)?),
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: "An element, comma, or end of list".to_owned(),
                });
            }
        }
    }
}
