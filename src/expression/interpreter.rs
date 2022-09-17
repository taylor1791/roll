use super::Expression;
use ibig::{rand::UniformUBig, IBig, UBig};
use num_traits::{One, Zero};
use rand::distributions::uniform::UniformSampler;

pub fn evaluate(
    rng: &mut rand::rngs::StdRng,
    rolls: &mut std::collections::HashMap<UBig, Vec<UBig>>,
    e: &Expression,
) -> Result<IBig, anyhow::Error> {
    match e {
        Expression::Dice {
            left: left_e,
            right: right_e,
            ..
        } => {
            let left = evaluate(rng, rolls, left_e)?;
            let right = evaluate(rng, rolls, right_e)?;
            let right = UBig::from(super::parse::die(&right, right_e)?);

            let mut sum = IBig::zero();
            for _ in 0..super::parse::dice(&left, left_e)? {
                let roll = int(rng, &right);
                sum += IBig::from(&roll);

                let dice_rolls = rolls.entry(right.clone()).or_insert(vec![]);
                dice_rolls.push(roll);
            }

            Ok(sum)
        }
        Expression::Difference { left, right, .. } => {
            Ok(evaluate(rng, rolls, left)? - evaluate(rng, rolls, right)?)
        }
        Expression::Exponentiation {
            left,
            right: right_e,
            ..
        } => {
            let left = evaluate(rng, rolls, left)?;
            let right = evaluate(rng, rolls, right_e)?;
            let right = super::parse::exponent(&right, right_e)?;
            Ok(left.pow(right))
        }
        Expression::IQuotient {
            left,
            right: right_e,
            ..
        } => {
            let right = evaluate(rng, rolls, right_e)?;
            let right = super::parse::nonzero(&right, right_e)?;
            Ok(evaluate(rng, rolls, left)? / right)
        }
        Expression::Product { left, right, .. } => {
            Ok(evaluate(rng, rolls, left)? * evaluate(rng, rolls, right)?)
        }
        Expression::Sum { left, right, .. } => {
            Ok(evaluate(rng, rolls, left)? + evaluate(rng, rolls, right)?)
        }
        Expression::Minus { operand, .. } => Ok(-evaluate(rng, rolls, operand)?),
        Expression::Plus { operand, .. } => Ok(evaluate(rng, rolls, operand)?),
        Expression::Literal(literal) => Ok(literal.clone()),
    }
}

fn int(rng: &mut rand::rngs::StdRng, sides: &UBig) -> UBig {
    UniformUBig::new_inclusive(UBig::one(), sides).sample(rng)
}
