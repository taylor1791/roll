use clap::Parser;
use human_panic::setup_panic;
use log::{info, trace};
use owo_colors::OwoColorize;

mod cli;
mod expression;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic!();
    env_logger::init();
    trace!("Logger initialized");

    let args = cli::Arguments::parse();
    trace!("Parsed Expression: {:?}", args.expression);

    let seed = args
        .seed
        .unwrap_or_else(|| rand::RngCore::next_u64(&mut rand::rngs::OsRng));
    info!("Using seed: {}", seed);

    let (mut rolls, sum) = args.expression.exec(seed);
    rolls.sort_by(|a, b| b.value.cmp(&a.value));
    let evaluand = Evaluand::Rolls(rolls, sum);
    trace!("Evaluated: {:?}", evaluand);

    print_evaluand(args, evaluand);

    Ok(())
}

#[derive(Debug)]
enum Evaluand {
    Rolls(Vec<expression::Roll>, i64),
}

impl Evaluand {
    fn to_json(&self) -> String {
        match self {
            Evaluand::Rolls(_, value) => value.to_string(),
        }
    }

    fn to_tsv(&self) -> String {
        match self {
            Evaluand::Rolls(_, value) => value.to_string(),
        }
    }
}

fn print_evaluand(args: cli::Arguments, evaluand: Evaluand) {
    if args.colors || (atty::is(atty::Stream::Stdout) && !args.json) {
        let Evaluand::Rolls(rolls, sum) = evaluand;

        let red = owo_colors::Style::new().fg::<owo_colors::colors::Red>();
        let default = owo_colors::Style::new();
        let green = owo_colors::Style::new().fg::<owo_colors::colors::Green>();
        let mut colored_rolls = Vec::with_capacity(rolls.len() * 2 - 1);
        for (i, roll) in rolls.iter().enumerate() {
            if roll.value == 1 {
                colored_rolls.push(red.style(roll.value.to_string()));
            } else if roll.value == roll.sides {
                colored_rolls.push(green.style(roll.value.to_string()));
            } else {
                colored_rolls.push(default.style(roll.value.to_string()));
            }

            if i != rolls.len() - 1 {
                colored_rolls.push(default.style(", ".to_string()));
            }
        }

        println!(
            "{} {}\n{} {{{}}}\n\n{}",
            "Expression:".magenta(),
            args.expression.to_string().blue(),
            "Rolls:".magenta(),
            owo_colors::StyledList::from(colored_rolls),
            sum.to_string().blue()
        );
    } else if args.json {
        println!("{}", evaluand.to_json())
    } else {
        println!("{}", evaluand.to_tsv())
    }
}
