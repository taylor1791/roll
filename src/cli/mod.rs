use crate::expression::{Evaluand, Expression};
use clap::Parser;
use log::{info, trace};
use std::fmt::{Display, Formatter};

mod roll;

pub struct Arguments {
    colors: bool,
    pub expression: Expression,
    json: bool,
    pub seed: u64,
}

impl Arguments {
    fn use_colors(&self) -> bool {
        atty::is(atty::Stream::Stdout) || self.colors
    }
}

impl Arguments {
    pub fn parse() -> Self {
        let args = RawArguments::parse();
        trace!("Parsed Expression: {:?}", args.expression);

        let seed = args
            .seed
            .unwrap_or_else(|| rand::RngCore::next_u64(&mut rand::rngs::OsRng));
        info!("Using seed: {}", seed);

        Self {
            colors: args.colors,
            expression: args.expression,
            json: args.json,
            seed,
        }
    }
}

pub struct CliFormatter {
    formatter: Box<dyn Display>,
}

impl CliFormatter {
    pub fn from(args: Arguments, evaluand: Evaluand) -> Self {
        Self {
            formatter: match args.json {
                true => Box::from(roll::JsonFormatter::from(evaluand)),
                false => Box::from(roll::TextFormatter::from(args, evaluand)),
            },
        }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct RawArguments {
    /// Forces color output (even if stdout is not a TTY)
    #[clap(long)]
    colors: bool,

    /// Print JSON to stdout
    #[clap(long)]
    json: bool,

    /// Display the distribution instead of rolling
    #[clap(long)]
    pdf: bool,

    /// Seeds the rng
    #[clap(long)]
    seed: Option<u64>,

    /// The dice expression to evaluate.
    expression: Expression,
}

impl Display for CliFormatter {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.formatter.fmt(formatter)
    }
}
