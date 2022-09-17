use super::Expression;
use ibig::IBig;
use num_traits::Zero;
use std::cmp::Ordering;

pub fn die(sides: &IBig, expression: &Expression) -> Result<usize, anyhow::Error> {
    match to_usize(sides, expression) {
        Ok(usize) => Ok(usize),
        Err((Ordering::Less, err)) => {
            Err(err.context("Dice with negative sides are not supported."))
        }
        Err((Ordering::Greater, err)) => {
            Err(err.context(format!("Dice cannot have more than {} sides.", usize::MAX)))
        }
        Err((Ordering::Equal, err)) => {
            Err(err.context(format!("Could not parse {} into usize die.", sides)))
        }
    }
}

pub fn dice(n: &IBig, expression: &Expression) -> Result<usize, anyhow::Error> {
    match to_usize(n, expression) {
        Ok(usize) => Ok(usize),
        Err((Ordering::Less, err)) => Err(err.context("Rolling negative dice are not supported.")),
        Err((Ordering::Greater, err)) => {
            Err(err.context(format!("Cannot role more than {} dice.", usize::MAX)))
        }
        Err((Ordering::Equal, err)) => {
            Err(err.context(format!("Could not parse {} into usize dice.", n)))
        }
    }
}

pub fn exponent(x: &IBig, expression: &Expression) -> Result<usize, anyhow::Error> {
    match to_usize(x, expression) {
        Ok(usize) => Ok(usize),
        Err((Ordering::Less, err)) => Err(err.context("Negative exponents are not supported.")),
        Err((Ordering::Greater, err)) => {
            Err(err.context(format!("Exponents must not exceed {}.", usize::MAX)))
        }
        Err((Ordering::Equal, err)) => {
            Err(err.context(format!("Could not parse {} into usize exponent.", x)))
        }
    }
}

pub fn nonzero<'a, A>(n: &'a A, expression: &Expression) -> Result<&'a A, anyhow::Error>
where
    A: PartialEq + Zero,
{
    if n == &A::zero() {
        return Err(anyhow::anyhow!(format!(
            "The divisor in the expression {} evaluated to 0. Dividing by zero is undefined.",
            expression.to_string(),
        )));
    }

    Ok(n)
}

fn to_usize(n: &IBig, expression: &Expression) -> Result<usize, (Ordering, anyhow::Error)> {
    if n < &IBig::zero() {
        return Err((
            Ordering::Less,
            anyhow::anyhow!(format!(
                "The expression {} evaluated to {}, a negative number.",
                expression.to_string(),
                n
            )),
        ));
    }

    if n > &IBig::from(usize::MAX) {
        return Err((
            Ordering::Greater,
            anyhow::anyhow!(format!(
                "The expression {} evaluated to {}, an excessively large number.",
                expression.to_string(),
                n
            )),
        ));
    }

    usize::try_from(n).map_err(|err| (Ordering::Equal, anyhow::anyhow!(err)))
}
