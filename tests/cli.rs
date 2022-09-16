use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use std::process::Command;

#[test]
fn roll_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--json").arg("--seed").arg("45").arg("3d6");
    cmd.assert()
        .success()
        .stdout("{\"rolls\":{\"d6\":[1,6,4]},\"value\":11}\n");

    Ok(())
}

#[test]
fn roll_tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--colors").arg("--seed").arg("45").arg("3d6");
    cmd.assert()
        .success()
        .stdout("\u{1b}[1m\u{1b}[35mExpression:\u{1b}[39m\u{1b}[0m \u{1b}[34m3d6\u{1b}[39m\n\u{1b}[1m\u{1b}[35mRolls:\u{1b}[39m\u{1b}[0m\n  d6: {\u{1b}[31m1\u{1b}[0m, 4, \u{1b}[32m6\u{1b}[0m}\n\n\u{1b}[34m11\u{1b}[39m\n");

    Ok(())
}

#[test]
fn roll_missing_tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--seed").arg("65").arg("1d100 + 10");
    cmd.assert().success().stdout("23\n");

    Ok(())
}

#[test]
fn pmf_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--pmf").arg("--json").arg("2d6");
    cmd.assert()
        .success()
        .stdout("{\"pmf\":[{\"value\":2,\"p\":0.02777777777777777},{\"value\":3,\"p\":0.05555555555555554},{\"value\":4,\"p\":0.08333333333333331},{\"value\":5,\"p\":0.11111111111111108},{\"value\":6,\"p\":0.13888888888888887},{\"value\":7,\"p\":0.16666666666666663},{\"value\":8,\"p\":0.13888888888888887},{\"value\":9,\"p\":0.11111111111111108},{\"value\":10,\"p\":0.08333333333333331},{\"value\":11,\"p\":0.05555555555555554},{\"value\":12,\"p\":0.02777777777777777}],\"statistics\":{\"min\":2,\"mean\":6.999999999999998,\"max\":12}}\n");

    Ok(())
}

#[test]
fn pmf_tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--colors").arg("--pmf").arg("2d6");
    cmd.assert().success().stdout("\u{1b}[1m\u{1b}[35mExpression:\u{1b}[39m\u{1b}[0m \u{1b}[34m2d6\u{1b}[39m\n  \u{1b}[1m\u{1b}[36mMean:\u{1b}[39m\u{1b}[0m 7.00\n\n   2  2.78%\n   3  5.56%\n   4  8.33%\n   5 11.11%\n   6 13.89%\n   7 16.67%\n   8 13.89%\n   9 11.11%\n  10  8.33%\n  11  5.56%\n  12  2.78%\n\n");

    Ok(())
}

#[test]
fn pmf_missing_tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--pmf").arg("2d6");
    cmd.assert().success().stdout("   2  2.78%\n   3  5.56%\n   4  8.33%\n   5 11.11%\n   6 13.89%\n   7 16.67%\n   8 13.89%\n   9 11.11%\n  10  8.33%\n  11  5.56%\n  12  2.78%\n\n");
    Ok(())
}
