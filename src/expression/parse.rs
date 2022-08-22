const MAX_SIDES: u64 = 2u64.pow(13);

pub fn die(sides: i64, expression: &super::Expression) -> Result<u64, anyhow::Error> {
    if sides < 1 {
        return Err(anyhow::anyhow!(format!(
            "The expression {} evaluated to, {}, a non-positive number.",
            expression.to_string(),
            sides,
        ))
        .context("Dice with non-positive sides are not supported."));
    }

    if sides > 1024 {
        return Err(anyhow::anyhow!(format!(
            "The expression {} evaluated to {}.",
            expression.to_string(),
            sides
        ))
        .context(format!(
            "Dice with more than {} sides are not supported.",
            MAX_SIDES
        )));
    }

    Ok(TryInto::<u64>::try_into(sides).unwrap())
}

pub fn exponent(x: i64, expression: &super::Expression) -> Result<u32, anyhow::Error> {
    u32::try_from(x).map_err(|err| {
        anyhow::anyhow!(err)
            .context(format!(
                "The expression {} evaluated to {}, a negative number",
                expression.to_string(),
                x
            ))
            .context("Negative exponents are not supported")
    })
}
