// Because of macros
#![allow(unreachable_patterns)]

use crate::language::{
    EvaluationError, Expression, Function, FunctionCode, FunctionParams, Operator, Statement,
    StatementBlock, Value,
};
use crate::parsing::multi_peek::MultiPeekable;
use std::collections::HashMap;

use crate::parsing::token::{Token, TokenWithMeta};

pub type Tokens<Iter> = MultiPeekable<Iter, TokenWithMeta>;

#[derive(Clone, Debug)]
pub enum ParserError {
    DuplicateArgument(String),
    ExpectedValue,
    ExpressionEvaluation(EvaluationError),
    MultipleTailParams,
    UndeclaredVariable(String),
    UnexpectedToken {
        token: TokenWithMeta,
        expected: &'static str,
    },
    UnexpectedEof {
        expected: String,
    },
    VariableRedeclaration(String),
}

macro_rules! match_token {
    ($token:expr, $pat:pat => $value:ident, expected = $expected:expr) => {
        match $token {
            Some(TokenWithMeta {
                token: $pat,
                meta: _,
            }) => $value,
            Some(token) => {
                return Err(ParserError::UnexpectedToken {
                    token: token.clone(),
                    expected: $expected,
                })
            }
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
            Some(token) => {
                return Err(ParserError::UnexpectedToken {
                    token: token.clone(),
                    expected: stringify!($expected),
                })
            }
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: format!("{:?}", $expected),
                })
            }
        }
    };
}

macro_rules! if_token {
    ($token:expr, $expected:pat, $code:block) => {
        if_token!($token, $expected, $code else {})
    };
    ($token:expr, $expected:pat, $true:block else $false:block) => {
        match $token {
            Some(TokenWithMeta { token, meta: _ }) if matches!(token, $expected) => $true,
            Some(_other) => $false,
            None => {
                return Err(ParserError::UnexpectedEof {
                    expected: format!("{:?}", stringify!($expected)),
                })
            }
        }
    };
}

const NULL_KEYWORD: &str = "null";
const TRUE_KEYWORD: &str = "true";
const FALSE_KEYWORD: &str = "false";
const VAR_KEY_WORD: &str = "var";
const DEF_KEYWORD: &str = "def";
const IF_KEYWORD: &str = "if";
const ELSE_KEYWORD: &str = "else";
const WHILE_KEYWORD: &str = "while";
const IMPORT_KEYWORD: &str = "import";

pub fn parse(tokens: impl Iterator<Item = TokenWithMeta>) -> Result<StatementBlock, ParserError> {
    let mut tokens = MultiPeekable::new(tokens);

    let mut root_block = Vec::new();

    while tokens.peek().is_some() {
        let statement = statement(&mut tokens)?;
        root_block.push(statement);
    }

    Ok(StatementBlock {
        statements: root_block,
    })
}

fn statement(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Statement, ParserError> {
    let peek = match_token!(tokens.peek(), anything => anything, expected = "a statement");

    match peek {
        Token::Ident(ident) if ident == VAR_KEY_WORD => var_declaration(tokens),
        Token::Ident(ident) if ident == DEF_KEYWORD => function_definition(tokens),
        Token::Ident(ident) if ident == IMPORT_KEYWORD => import(tokens),
        _ => {
            let target = expression(tokens)?;

            if let Some(TokenWithMeta {
                token: Token::Equal,
                ..
            }) = tokens.peek()
            {
                // Consume =
                tokens.next();
                let right_side = expression(tokens)?;

                Ok(Statement::Assignment { target, right_side })
            } else {
                Ok(Statement::Expression { expression: target })
            }
        }
    }
}

fn import(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Statement, ParserError> {
    match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "the import keyword");

    let library_name = match_token!(tokens.next(), Token::Ident(library_name) => library_name, expected = "a library name");

    let as_namespace = if let Some(TokenWithMeta {
        token: Token::Dot, ..
    }) = tokens.peek()
    {
        match_token!(tokens.next(), Token::Dot);
        match_token!(tokens.next(), Token::Star);

        false
    } else {
        true
    };

    Ok(Statement::Import {
        library_name,
        as_namespace,
    })
}

fn block(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<StatementBlock, ParserError> {
    match_token!(tokens.next(), Token::LeftBrace);

    let mut statements = Vec::new();
    loop {
        if_token!(tokens.peek(), &Token::RightBrace, {
            let _ = tokens.next();
            break;
        });

        let statement = statement(tokens)?;
        statements.push(statement);
    }

    Ok(StatementBlock { statements })
}

fn var_declaration(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Statement, ParserError> {
    match_token!(tokens.next(), Token::Ident(keyword) => keyword, expected = "variable declaration keyword `var`");

    let name =
        match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "a variable name");
    match_token!(tokens.next(), Token::Equal);

    let expression = expression(tokens)?;

    Ok(Statement::VariableDeclaration {
        variable: name,
        right_side: expression,
    })
}

fn function_definition(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Statement, ParserError> {
    match_token!(tokens.next(), Token::Ident(keyword) => keyword, expected = "function definition keyword `def`");

    let name =
        match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "a function name");

    let function = function_params_and_body(tokens)?;

    Ok(Statement::VariableDeclaration {
        variable: name.to_owned(),
        right_side: Expression::Function(function),
    })
}

fn function_params_and_body(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Function, ParserError> {
    match_token!(tokens.next(), Token::LeftParen);

    let mut params = FunctionParams::default();
    let mut tail = None;
    loop {
        if_token!(tokens.peek(), Token::RightParen, {
            let _ = tokens.next();
            break;
        });

        // Special handling for tail param
        if_token!(tokens.peek(), Token::Star, {
            let _ = tokens.next();
            let tail_name = match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "a tail parameter name");

            if tail.is_some() {
                return Err(ParserError::MultipleTailParams);
            }

            tail = Some(tail_name);

            if_token!(tokens.peek(), Token::Comma, {
                let _ = tokens.next();
            });

            continue;
        });

        let param_name = match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "a parameter name");
        let TokenWithMeta { token: peek, meta } =
            tokens.peek().ok_or(ParserError::UnexpectedEof {
                expected: "rest of function parameters".to_owned(),
            })?;

        let default = match peek {
            Token::Equal => {
                // Consume equal token
                let _ = tokens.next();
                let Expression::Value(value) = expression(tokens)? else {
                    return Err(ParserError::ExpectedValue);
                };

                Some(value)
            }
            Token::Comma | Token::RightParen => None,
            token => {
                return Err(ParserError::UnexpectedToken {
                    token: TokenWithMeta {
                        token: token.clone(),
                        meta: *meta,
                    },
                    expected: "one of '=', ',', ')'",
                });
            }
        };

        params = params.with_param(param_name, default);

        // Discard trailing comma
        if_token!(tokens.peek(), Token::Comma, {
            let _ = tokens.next();
        });
    }

    if let Some(tail) = tail {
        params = params.with_tail(tail);
    }

    let code = block(tokens)?;

    let function = Function {
        params,
        code: FunctionCode::Block(code),
        closure: None,
    };

    Ok(function)
}

fn function_call(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
    callee: Expression,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::LeftParen);

    let mut args = HashMap::new();
    let mut tail = Vec::new();

    loop {
        if_token!(tokens.peek(), &Token::RightParen, {
            let _ = tokens.next();
            break;
        });

        // 2 cases:
        //      - <param>: <value>
        //      - <value>
        let peek = tokens.multi_peek(2);
        match peek.get(1) {
            Some(TokenWithMeta {
                token: Token::Colon,
                ..
            }) => {
                let param = match_token!(tokens.next(), Token::Ident(param) => param, expected = "param name");
                match_token!(tokens.next(), Token::Colon);
                let expression = expression(tokens)?;

                if args.contains_key(&param) {
                    return Err(ParserError::DuplicateArgument(param));
                }

                args.insert(param, expression);
            }
            _ => {
                let expression = expression(tokens)?;
                tail.push(expression);
            }
        }

        // An end of the arguments or a comma must follow
        let TokenWithMeta { token: peek, meta } =
            tokens.peek().ok_or(ParserError::UnexpectedEof {
                expected: "comma or paren".to_owned(),
            })?;
        match peek {
            Token::Comma => {
                let _ = tokens.next();
            }
            Token::RightParen => {}
            token => {
                return Err(ParserError::UnexpectedToken {
                    token: TokenWithMeta {
                        token: token.clone(),
                        meta: *meta,
                    },
                    expected: "one of ',', ')'",
                });
            }
        }
    }

    Ok(Expression::FunctionCall {
        callee: Box::new(callee),
        args,
        tail,
    })
}

fn expression(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    logical_or(tokens)
}

fn logical_or(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = logical_and(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Or))
    {
        let Some(token) = tokens.next() else { break };
        let operator = token_to_operator(token)?;
        let right = logical_and(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn logical_and(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = equality(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::And))
    {
        let Some(token) = tokens.next() else { break };
        let operator = token_to_operator(token)?;
        let right = equality(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn equality(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = comparison(tokens)?;

    while tokens.peek().is_some_and(|TokenWithMeta { token, .. }| {
        matches!(token, Token::EqualEqual | Token::BangEqual)
    }) {
        let Some(token) = tokens.next() else { break };
        let operator = token_to_operator(token)?;
        let right = comparison(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn comparison(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = addition(tokens)?;

    while tokens.peek().is_some_and(|TokenWithMeta { token, .. }| {
        matches!(
            token,
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual
        )
    }) {
        let Some(token) = tokens.next() else { break };
        let operator = token_to_operator(token)?;
        let right = addition(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn addition(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
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

fn term(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = if_null_operator(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::Star | Token::Slash))
    {
        let Some(token) = tokens.next() else {
            break;
        };
        let operator = token_to_operator(token)?;
        let right = if_null_operator(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
    }

    Ok(left)
}

fn if_null_operator(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut left = unary(tokens)?;

    while tokens
        .peek()
        .is_some_and(|TokenWithMeta { token, .. }| matches!(token, Token::QuestionMark))
    {
        let Some(_token) = tokens.next() else {
            break;
        };
        let right = unary(tokens)?;

        left = Expression::Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator: Operator::IfNull,
        };
    }

    Ok(left)
}

fn unary(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    if tokens.peek().is_some_and(|TokenWithMeta { token, .. }| {
        matches!(token, Token::Plus | Token::Minus | Token::Bang)
    }) {
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

fn power(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
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

fn primary(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    let mut expression = match tokens.peek() {
        Some(TokenWithMeta {
            token: Token::LeftBracket,
            ..
        }) => parse_array(tokens)?,
        Some(TokenWithMeta {
            token: Token::LeftBrace,
            ..
        }) => parse_hash_map(tokens)?,
        Some(TokenWithMeta {
            token: Token::Ident(ident),
            ..
        }) if ident == IF_KEYWORD => if_else(tokens)?,
        Some(TokenWithMeta {
            token: Token::Ident(ident),
            ..
        }) if ident == WHILE_KEYWORD => while_loop(tokens)?,
        _ => {
            let Some(TokenWithMeta { token, meta }) = tokens.next() else {
                return Err(ParserError::UnexpectedEof {
                    expected: "anything expression like".to_owned(),
                });
            };

            match token {
                Token::Ident(ident) if ident == NULL_KEYWORD => Expression::Value(Value::Null),
                Token::Ident(ident) if ident == FALSE_KEYWORD => {
                    Expression::Value(Value::Bool(false))
                }
                Token::Ident(ident) if ident == TRUE_KEYWORD => {
                    Expression::Value(Value::Bool(true))
                }
                Token::Ident(ident) if ident == DEF_KEYWORD => {
                    let function = function_params_and_body(tokens)?;
                    Expression::Function(function)
                }
                Token::Ident(ident) => Expression::Variable(ident),
                Token::Int(int) => Expression::Value(Value::Int(int)),
                Token::String(string) => Expression::Value(Value::new_string(string)),
                Token::LeftParen => {
                    let expression = expression(tokens)?;
                    match_token!(tokens.next(), Token::RightParen);

                    expression
                }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        token: TokenWithMeta {
                            token: token.clone(),
                            meta,
                        },
                        expected: "a token for primary expression",
                    });
                }
            }
        }
    };

    while let Some(TokenWithMeta { token, .. }) = tokens.peek() {
        match token {
            Token::LeftBracket => {
                let index = parse_indexing(tokens)?;
                expression = Expression::Binary {
                    operator: Operator::Index,
                    left: Box::new(expression),
                    right: Box::new(index),
                };
            }
            Token::LeftParen => {
                expression = function_call(tokens, expression)?;
            }
            Token::Dot => {
                let property = property_access(tokens)?;
                expression = Expression::Binary {
                    operator: Operator::Index,
                    left: Box::new(expression),
                    right: Box::new(Expression::Value(Value::new_string(property))),
                };
            }
            _ => break,
        }
    }

    Ok(expression)
}

fn if_else(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::Ident(keyword) => keyword, expected = "if keyword");

    let condition = expression(tokens)?;
    let then_branch = block(tokens)?;
    let else_branch = if let Some(TokenWithMeta {
        token: Token::Ident(keyword),
        ..
    }) = tokens.peek()
        && keyword == ELSE_KEYWORD
    {
        match_token!(tokens.next(), Token::Ident(keyword) => keyword, expected = "else keyword");
        Some(block(tokens)?)
    } else {
        None
    };

    Ok(Expression::IfElse {
        condition: Box::new(condition),
        then_branch,
        else_branch,
    })
}

fn while_loop(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::Ident(keyword) => keyword, expected = "while keyword");

    let condition = expression(tokens)?;
    let body = block(tokens)?;

    Ok(Expression::WhileLoop {
        condition: Box::new(condition),
        body,
    })
}

fn parse_array(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
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
                let expression = expression(tokens)?;
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

fn parse_hash_map(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::LeftBrace);

    let mut pairs = Vec::new();
    while let Some(TokenWithMeta { token, .. }) = tokens.peek() {
        match token {
            // We need this branch because of empty hash maps {}
            Token::RightBrace => {
                tokens.next();
                return Ok(Expression::HashMap(pairs));
            }
            _ => {
                let key = expression(tokens)?;
                match_token!(tokens.next(), Token::Colon);
                let value = expression(tokens)?;

                pairs.push((key, value));
            }
        }

        let token = match_token!(tokens.peek(), t @ (Token::Comma | Token::RightBrace) => t, expected = "a comma or an end of hash map");
        // Discard comma
        if let Token::Comma = token {
            tokens.next();
        }
    }

    Err(ParserError::UnexpectedEof {
        expected: "an arrays next item or an end of the array".to_owned(),
    })
}

fn parse_indexing(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<Expression, ParserError> {
    match_token!(tokens.next(), Token::LeftBracket);

    let expression = expression(tokens)?;

    match_token!(tokens.next(), Token::RightBracket);

    Ok(expression)
}

fn property_access(
    tokens: &mut Tokens<impl Iterator<Item = TokenWithMeta>>,
) -> Result<String, ParserError> {
    match_token!(tokens.next(), Token::Dot);
    let property = match_token!(tokens.next(), Token::Ident(ident) => ident, expected = "accessed property name");

    Ok(property)
}

fn token_to_operator(token: TokenWithMeta) -> Result<Operator, ParserError> {
    Ok(match token.token {
        Token::Plus => Operator::Add,
        Token::Minus => Operator::Sub,
        Token::Star => Operator::Mul,
        Token::Slash => Operator::Div,
        Token::Caret => Operator::Power,
        Token::EqualEqual => Operator::Eq,
        Token::BangEqual => Operator::NotEq,
        Token::Less => Operator::Less,
        Token::LessEqual => Operator::LessEq,
        Token::Greater => Operator::Greater,
        Token::GreaterEqual => Operator::GreaterEq,
        Token::Bang => Operator::Not,
        Token::And => Operator::And,
        Token::Or => Operator::Or,
        _ => {
            return Err(ParserError::UnexpectedToken {
                token,
                expected: "an operator sign",
            });
        }
    })
}
