use ibig::{ibig, ops::UnsignedAbs, IBig, UBig};

pub fn die(sides: &IBig, expression: &super::Expression) -> Result<UBig, anyhow::Error> {
    if *sides < ibig!(1) {
        return Err(anyhow::anyhow!(format!(
            "The expression {} evaluated to, {}, a non-positive number.",
            expression.to_string(),
            sides,
        ))
        .context("Dice with non-positive sides are not supported."));
    }

    Ok(sides.unsigned_abs())
}

pub fn dice(sides: &IBig, expression: &super::Expression) -> Result<UBig, anyhow::Error> {
    if *sides < ibig!(1) {
        return Err(anyhow::anyhow!(format!(
            "The expression {} evaluated to, {}, a non-positive number.",
            expression.to_string(),
            sides,
        ))
        .context("Rolling negative dice is not supported."));
    }

    Ok(sides.unsigned_abs())
}

pub fn exponent(x: &IBig, expression: &super::Expression) -> Result<usize, anyhow::Error> {
    if *x > IBig::from(usize::MAX) {
        return Err(anyhow::anyhow!(format!(
            "The expression {} evaluated to the excessively large number {}.",
            expression.to_string(),
            x
        ))
        .context(format!("Exponents must not exceed {}.", usize::MAX)));
    }

    usize::try_from(x).map_err(|err| {
        anyhow::anyhow!(err)
            .context(format!(
                "The expression {} evaluated to {}, a negative number.",
                expression.to_string(),
                x
            ))
            .context("Negative exponents are not supported.")
    })
}
