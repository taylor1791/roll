use anyhow::Result;
use clap::Parser;
use human_panic::setup_panic;
use log::{info, trace};

mod cli;
mod expression;

fn main() -> Result<()> {
    setup_panic!();
    env_logger::init();
    trace!("Logger initialized");

    let args = cli::Arguments::parse();
    trace!("Parsed Expression: {:?}", args.expression);

    let seed = args
        .seed
        .unwrap_or_else(|| rand::RngCore::next_u64(&mut rand::rngs::OsRng));
    info!("Using seed: {}", seed);

    let evaluand = args.expression.eval(seed)?;
    trace!("Evaluated: {:?}", evaluand);

    println!("{}", cli::Output::from(args, evaluand));

    Ok(())
}
