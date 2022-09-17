use crate::combinatorics::Combinations;
use ibig::{IBig, UBig};
use owo_colors::OwoColorize;
use std::collections::HashMap;

mod interpreter;
mod operators;
mod parse;
mod parser;
mod pmf;

#[allow(clippy::all)]
mod precedence;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum Expression {
    Dice {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operators::Binary,
    },
    Difference {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operators::Binary,
    },
    Exponentiation {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operators::Binary,
    },
    IQuotient {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operators::Binary,
    },
    Product {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operators::Binary,
    },
    Sum {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: operators::Binary,
    },
    Minus {
        operand: Box<Expression>,
        operator: operators::Unary,
    },
    Plus {
        operand: Box<Expression>,
        operator: operators::Unary,
    },
    Literal(IBig),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Evaluand {
    pub rolls: HashMap<UBig, Vec<UBig>>,
    pub value: IBig,
}

impl Expression {
    pub fn eval(&self, seed: u64) -> Result<Evaluand, anyhow::Error> {
        let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(seed);
        let mut rolls = HashMap::new();
        let value = interpreter::evaluate(&mut rng, &mut rolls, self)?;

        Ok(Evaluand { rolls, value })
    }

    pub fn pmf(&self) -> Result<crate::pmf::Pmf<IBig>, anyhow::Error> {
        let mut combinations = Combinations::default();

        pmf::pmf(self, &mut combinations)
    }

    fn operator(&self) -> Option<operators::Operator> {
        match self {
            Expression::Dice { .. } => Some(operators::Operator::Binary(operators::DICE)),
            Expression::Difference { .. } => {
                Some(operators::Operator::Binary(operators::DIFFERENCE))
            }
            Expression::Exponentiation { .. } => {
                Some(operators::Operator::Binary(operators::EXPONENT))
            }
            Expression::IQuotient { .. } => Some(operators::Operator::Binary(operators::IDIVISION)),
            Expression::Product { .. } => Some(operators::Operator::Binary(operators::PRODUCT)),
            Expression::Sum { .. } => Some(operators::Operator::Binary(operators::SUM)),
            Expression::Minus { .. } => Some(operators::Operator::Unary(operators::MINUS)),
            Expression::Plus { .. } => Some(operators::Operator::Unary(operators::PLUS)),
            Expression::Literal(..) => None,
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    err: anyhow::Error,
    expression: String,
    token: Option<(usize, usize)>,
}

fn convert_error(input: &str, nom_error: nom::error::VerboseError<&str>) -> ParseError {
    let (position, length) = match nom_error.errors.as_slice() {
        [(substring, _), ..] => (nom::Offset::offset(input, substring), substring.len()),
        _ => (0, input.len()),
    };

    if position == input.len() {
        ParseError {
            err: anyhow::anyhow!(format!("Unexpected end of input.")),
            expression: String::from(input),
            token: None,
        }
    } else {
        ParseError {
            err: anyhow::anyhow!(format!("Unexpected token at position {}.", position + 1)),
            expression: String::from(input),
            token: Some((position, length)),
        }
    }
}

impl std::str::FromStr for Expression {
    type Err = ParseError;

    fn from_str(i: &str) -> Result<Self, Self::Err> {
        match parser::parse(i) {
            Ok((_, expression)) => Ok(expression),
            Err(err) => Err(convert_error(i, err)),
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Dice {
                left,
                right,
                operator,
            }
            | Expression::Difference {
                left,
                right,
                operator,
            }
            | Expression::Exponentiation {
                left,
                right,
                operator,
            }
            | Expression::IQuotient {
                left,
                right,
                operator,
            }
            | Expression::Product {
                left,
                right,
                operator,
            }
            | Expression::Sum {
                left,
                right,
                operator,
            } => {
                let mut str = String::from("");
                let self_precedence = self.operator().map(|op| op.precedence()).unwrap_or(0);
                let left_precedence = left.operator().map(|op| op.precedence()).unwrap_or(0);
                let right_precedence = right.operator().map(|op| op.precedence()).unwrap_or(0);

                if operator.assoc == precedence::Assoc::Left && left_precedence > self_precedence
                    || operator.assoc == precedence::Assoc::Right
                        && left_precedence >= self_precedence
                {
                    str.push_str(&format!("({})", left.to_string()));
                } else {
                    str.push_str(&left.to_string());
                }

                if operator.space {
                    str.push_str(&format!(" {} ", operator.symbol));
                } else {
                    str.push_str(operator.symbol);
                }

                if operator.assoc == precedence::Assoc::Left && self_precedence <= right_precedence
                    || operator.assoc == precedence::Assoc::Right
                        && self_precedence < right_precedence
                {
                    str.push_str(&format!("({})", right.to_string()));
                } else {
                    str.push_str(&right.to_string());
                }

                str
            }
            Expression::Minus { operand, operator } | Expression::Plus { operand, operator } => {
                format!("{}{}", operator.symbol, operand.to_string())
            }
            Expression::Literal(literal) => literal.to_string(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_fmt(format_args!("{}\n", self.err))?;

        if let Some((position, length)) = self.token {
            formatter.write_str("\n")?;
            formatter.write_fmt(format_args!("    {}\n", "|".blue()))?;
            formatter.write_fmt(format_args!("    {}    {}\n", "|".blue(), self.expression))?;
            formatter.write_fmt(format_args!(
                "    {}    {}{}\n",
                "|".blue(),
                " ".repeat(position),
                "^".repeat(length).red()
            ))?;
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}
