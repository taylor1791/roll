use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use std::process::Command;

#[test]
fn tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--colors").arg("--seed").arg("29").arg("3d6");
    cmd.assert()
        .success()
        .stdout("\u{1b}[35mExpression:\u{1b}[39m \u{1b}[34m3d6\u{1b}[39m\n\u{1b}[35mRolls:\u{1b}[39m {\u{1b}[32m6\u{1b}[0m, 4, \u{1b}[31m1\u{1b}[0m}\n\n\u{1b}[34m11\u{1b}[39m\n");

    Ok(())
}

#[test]
fn missing_tty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("--seed").arg("57").arg("1d100 + 10");
    cmd.assert().success().stdout("23\n");

    Ok(())
}
