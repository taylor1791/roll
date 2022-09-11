use crate::command::{Command, Roll};
use crate::json;
use clap::Parser;
use log::warn;
use roll::expression::{Evaluand, Expression};

pub struct Arguments {
    colors: bool,
    json: bool,
}

impl Arguments {
    pub fn parse() -> (CliCommand, Self) {
        let args = RawArguments::parse();
        let seed = args
            .seed
            .unwrap_or_else(|| rand::RngCore::next_u64(&mut rand::rngs::OsRng));
        let command = CliCommand::Roll(Roll::new(args.expression, seed));

        (
            command,
            Self {
                colors: args.colors,
                json: args.json,
            },
        )
    }

    pub fn use_colors(&self) -> bool {
        atty::is(atty::Stream::Stdout) || self.colors
    }
}

#[derive(Debug)]
pub enum CliCommand {
    Roll(Roll),
}

impl CliCommand {
    pub fn exec(self) -> Result<CliOutput, anyhow::Error> {
        match self {
            CliCommand::Roll(roll) => {
                let output = roll.exec()?;
                Ok(CliOutput::Roll(roll, output))
            }
        }
    }
}

pub enum CliOutput {
    Roll(Roll, Evaluand),
}

impl CliOutput {
    pub fn formatter(self, args: Arguments) -> Box<dyn std::fmt::Display> {
        if args.json {
            return match self {
                CliOutput::Roll(_, output) => Box::from(JsonFormatter(json::Evaluand::new(output))),
            };
        }

        match self {
            CliOutput::Roll(roll, output) => roll.formatter(args, output),
        }
    }
}

struct JsonFormatter<A>(A);

impl<A> std::fmt::Display for JsonFormatter<A>
where
    A: serde::Serialize,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let json = serde_json::to_string(&self.0).map_err(|err| {
            warn!("{}", err);
            std::fmt::Error
        })?;

        formatter.write_str(&json)
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

impl std::fmt::Debug for CliOutput {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            CliOutput::Roll(_, output) => std::fmt::Debug::fmt(output, formatter),
        }
    }
}
