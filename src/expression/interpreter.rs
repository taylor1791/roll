use super::Expression;

pub fn evaluate(
    rng: &mut rand::rngs::StdRng,
    rolls: &mut std::collections::HashMap<u64, Vec<u64>>,
    e: &Expression,
) -> Result<i64, anyhow::Error> {
    match e {
        Expression::Dice {
            left,
            right: right_e,
            ..
        } => {
            let left = evaluate(rng, rolls, left)?;
            let right = evaluate(rng, rolls, right_e)?;
            let right = super::parse::die(right, right_e)?;

            let mut sum = 0;
            for _ in 0..left {
                let roll = int(rng, right);
                let dice_rolls = rolls.entry(right).or_insert(vec![]);
                dice_rolls.push(roll);

                sum += TryInto::<i64>::try_into(roll).unwrap();
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
            let right = super::parse::exponent(right, right_e)?;
            Ok(left.pow(right))
        }
        Expression::Sum { left, right, .. } => {
            Ok(evaluate(rng, rolls, left)? + evaluate(rng, rolls, right)?)
        }
        Expression::Minus { operand, .. } => Ok(-evaluate(rng, rolls, operand)?),
        Expression::Plus { operand, .. } => Ok(evaluate(rng, rolls, operand)?),
        Expression::Literal(literal) => Ok(*literal),
    }
}

fn int(rng: &mut rand::rngs::StdRng, sides: u64) -> u64 {
    rand::RngCore::next_u64(rng) % sides + 1
}
