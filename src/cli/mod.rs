use crate::expression::{Evaluand, Expression};
use clap::Parser;
use std::fmt::{Display, Formatter};

mod roll;

pub struct Output {
    formatter: Box<dyn Display>,
}

impl Output {
    pub fn from(args: Arguments, evaluand: Evaluand) -> Self {
        let formatter: Box<dyn Display> = if args.json {
            Box::from(roll::JsonFormatter::from(evaluand))
        } else {
            Box::from(roll::TextFormatter::from(args, evaluand))
        };

        Output { formatter }
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Forces color output (even if stdout is not a TTY)
    #[clap(long)]
    colors: bool,

    /// Print JSON to stdout
    #[clap(long)]
    pub json: bool,

    /// Seeds the rng
    #[clap(long)]
    pub seed: Option<u64>,

    /// The dice expression to evaluate.
    pub expression: Expression,
}

impl Arguments {
    fn use_colors(&self) -> bool {
        atty::is(atty::Stream::Stdout) || self.colors
    }
}

impl Display for Output {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.formatter.fmt(formatter)
    }
}
