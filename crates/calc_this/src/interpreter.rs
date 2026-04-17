use crate::{
    ast::{AstNode, Function, Operator},
    error::CalcError,
};

pub(crate) fn evaluate<'a>(
    node: AstNode<'a>,
    variables: &[(&str, f64)],
) -> Result<f64, CalcError<'a>> {
    let value = match node {
        AstNode::Value(value) => value,
        AstNode::Variable(ident) => {
            let (_, value) = variables
                .iter()
                .find(|(name, _)| *name == ident)
                .ok_or(CalcError::UndefinedVariable(ident))?;
            *value
        }
        AstNode::Function { function, operand } => {
            let value = evaluate(*operand, variables)?;
            apply_function(function, value)
        }
        AstNode::Unary { operator, operand } => {
            let value = evaluate(*operand, variables)?;
            match operator {
                Operator::Plus => value,
                Operator::Minus => -value,
                op => return Err(CalcError::IllegelOperator(op)),
            }
        }
        AstNode::Binary {
            left,
            right,
            operator,
        } => {
            let left = evaluate(*left, variables)?;
            let right = evaluate(*right, variables)?;

            match operator {
                Operator::Plus => left + right,
                Operator::Minus => left - right,
                Operator::Star => left * right,
                Operator::Slash => {
                    if right == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }

                    left / right
                }
                Operator::Caret => left.powf(right),
            }
        }
    };

    Ok(value)
}

fn apply_function(function: Function, value: f64) -> f64 {
    match function {
        Function::Sqrt => value.sqrt(),
        Function::Sin => value.sin(),
        Function::Cos => value.cos(),
        Function::Sign => value.signum(),
    }
}
