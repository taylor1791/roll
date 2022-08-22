use anyhow::Result;
use human_panic::setup_panic;
use log::trace;

mod cli;
mod expression;

fn main() -> Result<()> {
    setup_panic!();
    env_logger::init();
    trace!("Logger initialized");

    let args = cli::Arguments::parse();
    let evaluand = args.expression.eval(args.seed)?;
    trace!("Evaluated: {:?}", evaluand);

    let formatter = cli::CliFormatter::from(args, evaluand);
    println!("{}", formatter);

    Ok(())
}
