use crate::elemental::RepeatingCommand;
use crate::language::{EvaluationError, Expression, Operator, Value, Variables};
use std::{collections::HashMap, iter::Peekable};

use crate::{
    element::KoolElement,
    parsing::{
        construction::{ElementConstructionError, ElementContent, ElementInConstruction},
        token::{Token, TokenWithMeta},
    },
};

#[derive(Clone, Debug)]
pub enum ParserError<'a> {
    Construction(ElementConstructionError<'a>),
    ExpectedValue,
    ExpressionEvaluation(EvaluationError),
    UndeclaredVariable(&'a str),
    UnexpectedToken(TokenWithMeta<'a>),
    UnexpectedEof { expected: String },
    VariableRedeclaration(&'a str),
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
    next_id: u32,
    pub variables: Variables,
    pub commands: Vec<RepeatingCommand>,
}

impl ParserContext {
    pub fn claim_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

const VAR_DECL_KEYWORD: &str = "var";

pub(super) fn parse<'a>(
    tokens: impl Iterator<Item = TokenWithMeta<'a>>,
    context: &mut ParserContext,
) -> Result<KoolElement, ParserError<'a>> {
    let mut tokens = tokens.peekable();

    loop {
        let ident = match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "Element identifier");

        match ident {
            VAR_DECL_KEYWORD => parse_var_assignment(&mut tokens, context, true)?,
            ident => {
                return parse_element(&mut tokens, ident, context);
            }
        }
    }
}

fn parse_var_assignment<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
    context: &mut ParserContext,
    is_declaration: bool,
) -> Result<(), ParserError<'a>> {
    let name = match_token!(tokens.next(), Token::Ident(name) => name, expected = "Variable name");

    match_token!(tokens.next(), Token::Equal);

    let expression = parse_expression(tokens)?;
    let value = expression
        .evaluate(&context.variables)
        .map_err(ParserError::ExpressionEvaluation)?;

    let old_value = context.variables.set(name, value);
    match (old_value, is_declaration) {
        (Some(_), true) => Err(ParserError::VariableRedeclaration(name)),
        (None, false) => Err(ParserError::UndeclaredVariable(name)),
        _ => Ok(()),
    }
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
    let peek = tokens.peek().ok_or(ParserError::UnexpectedEof {
        expected: "Element identifier, key identifier, list or an expression".to_owned(),
    })?;

    match peek {
        TokenWithMeta {
            token: Token::Number(_) | Token::String(_) | Token::Variable(_),
            ..
        } => {
            let expression = parse_expression(tokens)?;
            element.inner = ElementContent::Expression(expression);
            return Ok(());
        }
        TokenWithMeta {
            token: Token::LeftBracket,
            ..
        } => {
            let elements = parse_elements_list(tokens, context)?;
            element.inner = ElementContent::Multiple(elements);
            return Ok(());
        }
        _ => {}
    }

    let ident =
        match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "Element identifier");

    // Parse stuff needing to know the next token
    match tokens.peek() {
        Some(TokenWithMeta {
            token: Token::Colon,
            meta: _,
        }) => {
            parse_key_value(tokens, element, ident, context)?;

            match_token!(tokens.next(), Token::Comma);

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
    match_token!(tokens.next(), Token::Colon);

    let expression = parse_expression(tokens)?;
    element.values.insert(key, expression);

    Ok(())
}

fn parse_elements_list<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
    context: &mut ParserContext,
) -> Result<Vec<KoolElement>, ParserError<'a>> {
    match_token!(tokens.next(), Token::LeftBracket);

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

fn parse_expression<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
) -> Result<Expression, ParserError<'a>> {
    let mut left = term(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Plus | Token::Minus))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = term(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn term<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
) -> Result<Expression, ParserError<'a>> {
    let mut left = unary(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Star | Token::Slash))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = unary(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn unary<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
) -> Result<Expression, ParserError<'a>> {
    if tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Plus | Token::Minus))
    {
        let Some(token) = tokens.next() else {
            return Err(ParserError::UnexpectedEof {
                expected: "an operator".to_owned(),
            });
        };
        let operator = token_to_operator(token)?;
        let operand = unary(tokens)?;

        Ok(Expression::Unary {
            operator,
            operand: Box::new(operand),
        })
    } else {
        power(tokens)
    }
}

fn power<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
) -> Result<Expression, ParserError<'a>> {
    let mut left = primary(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Caret))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = primary(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn primary<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
) -> Result<Expression, ParserError<'a>> {
    let mut expression = if let Some(TokenWithMeta {
        token: Token::LeftBracket,
        ..
    }) = tokens.peek()
    {
        parse_array(tokens)?
    } else {
        let Some(TokenWithMeta { token, meta }) = tokens.next() else {
            return Err(ParserError::UnexpectedEof {
                expected: "a number, a string, an array, an expression in parens, or variable"
                    .to_owned(),
            });
        };

        match token {
            Token::Number(num) => Expression::Value(Value::Number(num)),
            Token::String(string) => Expression::Value(Value::String(string)),
            Token::Variable(name) => Expression::Variable(name),
            Token::LeftParen => {
                let expression = parse_expression(tokens)?;
                match_token!(tokens.next(), Token::RightParen);

                expression
            }
            _ => return Err(ParserError::UnexpectedToken(TokenWithMeta { token, meta })),
        }
    };

    while let Some(TokenWithMeta {
        token: Token::LeftBracket,
        ..
    }) = tokens.peek()
    {
        let index = parse_indexing(tokens)?;
        expression = Expression::Binary {
            operator: Operator::Index,
            left: Box::new(expression),
            right: Box::new(index),
        };
    }

    Ok(expression)
}

fn parse_array<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
) -> Result<Expression, ParserError<'a>> {
    match_token!(tokens.next(), Token::LeftBracket);

    let mut array = Vec::new();
    while let Some(TokenWithMeta { token, .. }) = tokens.peek() {
        match token {
            // We need this branch because of empty arrays []
            Token::RightBracket => {
                tokens.next();
                return Ok(Expression::Array(array));
            }
            _ => {
                let expression = parse_expression(tokens)?;
                array.push(expression);
            }
        }

        let token = match_token!(tokens.peek(), t @ (Token::Comma | Token::RightBracket) => t, expected = "a comma or an end of array");
        // Discard comma
        if let Token::Comma = token {
            tokens.next();
        }
    }

    Err(ParserError::UnexpectedEof {
        expected: "an arrays next item or an end of the array".to_owned(),
    })
}

fn parse_indexing<'a>(
    tokens: &mut Peekable<impl Iterator<Item = TokenWithMeta<'a>>>,
) -> Result<Expression, ParserError<'a>> {
    match_token!(tokens.next(), Token::LeftBracket);

    let expression = parse_expression(tokens)?;

    match_token!(tokens.next(), Token::RightBracket);

    Ok(expression)
}

fn token_to_operator(token: TokenWithMeta<'_>) -> Result<Operator, ParserError<'_>> {
    Ok(match token.token {
        Token::Plus => Operator::Add,
        Token::Minus => Operator::Sub,
        Token::Star => Operator::Mul,
        Token::Slash => Operator::Div,
        Token::Caret => Operator::Power,
        _ => return Err(ParserError::UnexpectedToken(token)),
    })
}
