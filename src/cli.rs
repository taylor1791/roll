use super::expression::Expression;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Forces color output (even if stdout is not a TTY).
    #[clap(long)]
    pub colors: bool,

    /// Print JSON to stdout
    #[clap(long)]
    pub json: bool,

    /// Seeds the rng
    #[clap(long)]
    pub seed: Option<u64>,

    /// The dice expression to evaluate.
    pub expression: Expression,
}
