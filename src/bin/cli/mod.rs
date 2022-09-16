use crate::command::{Command, Pmf, Roll};
use crate::json;
use clap::Parser;
use log::warn;
use roll::expression::Expression;

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

        let command = match args.pmf {
            true => CliCommand::Pmf(Pmf::new(args.expression)),
            false => CliCommand::Roll(Roll::new(args.expression, seed)),
        };

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
    Pmf(Pmf),
    Roll(Roll),
}

impl CliCommand {
    pub fn exec(self) -> Result<CliOutput, anyhow::Error> {
        match self {
            CliCommand::Pmf(pmf) => {
                let output = pmf.exec()?;

                Ok(CliOutput::Pmf(pmf, output))
            }
            CliCommand::Roll(roll) => {
                let output = roll.exec()?;
                Ok(CliOutput::Roll(roll, output))
            }
        }
    }
}

pub enum CliOutput {
    Roll(Roll, <Roll as Command>::Output),
    Pmf(Pmf, <Pmf as Command>::Output),
}

impl CliOutput {
    pub fn formatter(self, args: Arguments) -> Box<dyn std::fmt::Display> {
        if args.json {
            return match self {
                CliOutput::Roll(_, output) => Box::from(JsonFormatter(json::Evaluand::new(output))),
                CliOutput::Pmf(_, output) => Box::from(JsonFormatter(json::Pmf::new(output))),
            };
        }

        match self {
            CliOutput::Roll(command, output) => command.formatter(args, output),
            CliOutput::Pmf(command, output) => command.formatter(args, output),
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
    pmf: bool,

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
            CliOutput::Pmf(_, output) => std::fmt::Debug::fmt(output, formatter),
        }
    }
}
