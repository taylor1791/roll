use super::{operators, precedence, Expression};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::one_of,
    combinator::{all_consuming, fail, map, map_res, opt, recognize},
    multi::many1,
    sequence::{delimited, terminated},
    IResult,
};

pub fn parse(i: &str) -> IResult<&str, Expression> {
    all_consuming(terminated(expression, opt(space)))(i)
}

fn expression(i: &str) -> IResult<&str, Expression> {
    precedence::precedence(
        alt((
            prefix_op(operators::MINUS),
            prefix_op(operators::PLUS),
            prefix_op(operators::D),
        )),
        fail,
        alt((
            binary_op(operators::DICE),
            binary_op(operators::EXPONENT),
            binary_op(operators::DIFFERENCE),
            binary_op(operators::SUM),
        )),
        alt((literal, delimited(tag("("), expression, tag(")")))),
        |op: precedence::Operation<&str, &str, &str, Expression>| match op {
            // Binary Expressions
            precedence::Operation::Binary(left, operator, right) => {
                if operator == operators::DICE.symbol {
                    Ok::<Expression, Expression>(Expression::Dice {
                        left: Box::from(left),
                        right: Box::from(right),
                        operator: operators::DICE,
                    })
                } else if operator == operators::DIFFERENCE.symbol {
                    Ok::<Expression, Expression>(Expression::Difference {
                        left: Box::from(left),
                        right: Box::from(right),
                        operator: operators::DIFFERENCE,
                    })
                } else if operator == operators::EXPONENT.symbol {
                    Ok::<Expression, Expression>(Expression::Exponentiation {
                        left: Box::from(left),
                        right: Box::from(right),
                        operator: operators::EXPONENT,
                    })
                } else if operator == operators::SUM.symbol {
                    Ok::<Expression, Expression>(Expression::Sum {
                        left: Box::from(left),
                        right: Box::from(right),
                        operator: operators::SUM,
                    })
                } else {
                    unreachable!("Unknown binary operator: {}", operator)
                }
            }

            // Prefix Expressions
            precedence::Operation::Prefix(operator, operand) => {
                if operator == operators::DICE.symbol {
                    Ok::<Expression, Expression>(Expression::Dice {
                        left: Box::from(Expression::Literal(1)),
                        right: Box::from(operand),
                        operator: operators::DICE,
                    })
                } else if operator == operators::MINUS.symbol {
                    Ok::<Expression, Expression>(Expression::Minus {
                        operand: Box::from(operand),
                        operator: operators::MINUS,
                    })
                } else if operator == operators::PLUS.symbol {
                    Ok::<Expression, Expression>(Expression::Plus {
                        operand: Box::from(operand),
                        operator: operators::PLUS,
                    })
                } else {
                    unreachable!("Unknown prefix operator: {}", operator)
                }
            }

            // Postfix Expressions
            precedence::Operation::Postfix(_, operator) => {
                unreachable!("Unknown postfix operator: {}", operator)
            }
        },
    )(i)
}

fn binary_op(
    operator: operators::Binary,
) -> impl FnMut(&str) -> IResult<&str, precedence::Binary<&str, u64>> {
    move |i: &str| {
        if operator.space {
            precedence::binary_op(
                operator.precedence,
                operator.assoc,
                space_delimited(operator.symbol),
            )(i)
        } else {
            precedence::binary_op(operator.precedence, operator.assoc, tag(operator.symbol))(i)
        }
    }
}

fn prefix_op(
    operator: operators::Unary,
) -> impl FnMut(&str) -> IResult<&str, precedence::Unary<&str, u64>> {
    move |i: &str| precedence::unary_op(operator.precedence, tag(operator.symbol))(i)
}

fn space_delimited(s: &str) -> impl FnMut(&str) -> IResult<&str, &str> + '_ {
    move |i: &str| delimited(space, tag(s), space)(i)
}

fn literal(i: &str) -> IResult<&str, Expression> {
    map(decimal, Expression::Literal)(i)
}

fn decimal(i: &str) -> IResult<&str, i64> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        str::parse(out).map_err(|_| ())
    })(i)
}

fn space(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";

    take_while(move |c| chars.contains(c))(i)
}
