use super::Expression;

const MAX_SIDES: u64 = 2u64.pow(13);

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

            if right < 1 {
                return Err(anyhow::anyhow!(format!(
                    "The expression {} evaluated to, {}, a non-positive number.",
                    right_e.to_string(),
                    right,
                ))
                .context("Dice with non-positive sides are not supported."));
            }

            if right > 1024 {
                return Err(anyhow::anyhow!(format!(
                    "The expression {} evaluated to {}.",
                    right_e.to_string(),
                    right
                ))
                .context(format!(
                    "Dice with more than {} sides are not supported.",
                    MAX_SIDES
                )));
            }

            let right = TryInto::<u64>::try_into(right).unwrap();

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
            let right = u32::try_from(right).map_err(|err| {
                anyhow::anyhow!(err)
                    .context(format!(
                        "The expression {} evaluated to {}, a negative number",
                        right_e.to_string(),
                        right
                    ))
                    .context("Negative exponents are not supported")
            })?;
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
