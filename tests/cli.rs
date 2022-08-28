use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use std::process::Command;

#[test]
fn json() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--json").arg("--seed").arg("45").arg("3d6");
    cmd.assert()
        .success()
        .stdout("{\"value\":11,\"rolls\":{\"d6\":[1,6,4]}}\n");

    Ok(())
}

#[test]
fn tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--colors").arg("--seed").arg("45").arg("3d6");
    cmd.assert()
        .success()
        .stdout("\u{1b}[1m\u{1b}[35mExpression:\u{1b}[39m\u{1b}[0m \u{1b}[34m3d6\u{1b}[39m\n\u{1b}[1m\u{1b}[35mRolls:\u{1b}[39m\u{1b}[0m\n  d6: {\u{1b}[31m1\u{1b}[0m, 4, \u{1b}[32m6\u{1b}[0m}\n\n\u{1b}[34m11\u{1b}[39m\n");

    Ok(())
}

#[test]
fn missing_tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--seed").arg("65").arg("1d100 + 10");
    cmd.assert().success().stdout("23\n");

    Ok(())
}
