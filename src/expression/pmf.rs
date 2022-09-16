use super::Expression;
use crate::combinatorics::Combinations;
use crate::iter_func::IterFunc;
use crate::pmf::Pmf;
use ibig::IBig;

pub fn pmf(e: &Expression, combinations: &mut Combinations) -> Result<Pmf<IBig>, anyhow::Error> {
    match e {
        Expression::Dice { left, right, .. } => {
            let left = pmf(left, combinations)?
                .iter()
                .map(|outcome| Ok((outcome.p, super::parse::dice(&outcome.value, left)?)))
                .collect::<Result<Pmf<_>, anyhow::Error>>()?;

            let right = pmf(right, combinations)?
                .iter()
                .map(|outcome| Ok((outcome.p, super::parse::die(&outcome.value, right)?)))
                .collect::<Result<Pmf<_>, anyhow::Error>>()?;

            Ok(left
                .iter()
                .cartesian_product_by(right.iter(), |n_dice, die| {
                    (n_dice.p * die.p, n_dice.value, die.value)
                })
                .flat_map(|(p, n_dice, die)| {
                    let max_roll = n_dice * die;

                    (n_dice..=max_roll)
                        .into_iter()
                        .map(move |sum| (p, sum, n_dice, die))
                })
                .scan(combinations, |combinations, (p, sum, n_dice, die)| {
                    Some((
                        p * combinations.probability_dice_roll_sum(sum, n_dice, die),
                        IBig::from(sum),
                    ))
                })
                .collect::<Pmf<IBig>>())
        }
        Expression::Difference { left, right, .. } => {
            let left = pmf(left, combinations)?;
            let right = pmf(right, combinations)?;

            Ok(left.cartesian_product(&right, |l, r| l - r))
        }
        Expression::Exponentiation { left, right, .. } => {
            let left = pmf(left, combinations)?;
            let right = pmf(right, combinations)?
                .iter()
                .map(|outcome| Ok((outcome.p, super::parse::exponent(&outcome.value, right)?)))
                .collect::<Result<Pmf<_>, anyhow::Error>>()?;

            Ok(left.cartesian_product(&right, |b, x| b.pow(*x)))
        }
        Expression::Sum { left, right, .. } => {
            let left = pmf(left, combinations)?;
            let right = pmf(right, combinations)?;

            Ok(left.cartesian_product(&right, |l, r| l + r))
        }
        Expression::Minus { operand, .. } => Ok(pmf(operand, combinations)?.map(|value| -value)),
        Expression::Plus { operand, .. } => pmf(operand, combinations),
        Expression::Literal(literal) => Ok(Pmf::constant(literal.clone())),
    }
}
