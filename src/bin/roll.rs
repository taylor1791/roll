use anyhow::Result;
use human_panic::setup_panic;
use log::trace;

mod cli;
mod command;

fn main() -> Result<()> {
    setup_panic!();
    env_logger::init();
    trace!("Logger initialized");

    let (cmd, args) = cli::Arguments::parse();
    trace!("Command: {:?}", cmd);

    let output = cmd.exec()?;
    trace!("Evaluated: {:?}", output);

    let formatter = output.formatter(args);
    println!("{}", formatter);

    Ok(())
}
