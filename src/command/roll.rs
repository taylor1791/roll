use super::Command;
use crate::expression::{Evaluand, Expression};
use ibig::UBig;
use num_traits::One;
use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Roll {
    expression: Expression,
    seed: u64,
}

impl Roll {
    pub fn new(expression: Expression, seed: u64) -> Self {
        Self { expression, seed }
    }
}

impl Command for Roll {
    type Output = Evaluand;

    fn exec(&self) -> Result<Self::Output, anyhow::Error> {
        self.expression.eval(self.seed)
    }

    fn formatter(
        self,
        args: crate::cli::Arguments,
        output: Self::Output,
    ) -> Box<dyn std::fmt::Display> {
        Box::from(TextFormatter {
            colors: args.use_colors(),
            evaluand: output,
            expression: self.expression,
        })
    }
}

pub struct TextFormatter {
    colors: bool,
    evaluand: Evaluand,
    expression: Expression,
}

impl Display for TextFormatter {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        if !self.colors {
            return formatter.write_fmt(format_args!("{}", self.evaluand.value));
        }

        let red = owo_colors::Style::new().fg::<owo_colors::colors::Red>();
        let default = owo_colors::Style::new();
        let green = owo_colors::Style::new().fg::<owo_colors::colors::Green>();

        formatter.write_fmt(format_args!(
            "{} {}\n",
            "Expression:".magenta().bold(),
            self.expression.to_string().blue()
        ))?;

        formatter.write_fmt(format_args!("{}\n", "Rolls:".magenta().bold()))?;
        for (i, (dice, dice_rolls)) in self.evaluand.rolls.iter().enumerate() {
            formatter.write_fmt(format_args!("  d{}: {{", dice))?;

            let mut dice_rolls = dice_rolls.clone();
            dice_rolls.sort_unstable();

            for (i, roll) in dice_rolls.iter().enumerate() {
                let style = if *roll == UBig::one() {
                    red
                } else if roll == dice {
                    green
                } else {
                    default
                };

                formatter.write_fmt(format_args!("{}", style.style(roll)))?;

                if i < dice_rolls.len() - 1 {
                    formatter.write_str(", ")?;
                }
            }

            formatter.write_str("}")?;

            if i < self.evaluand.rolls.len() - 1 {
                formatter.write_str("\n")?;
            }
        }

        formatter.write_fmt(format_args!(
            "\n\n{}",
            self.evaluand.value.to_string().blue()
        ))?;

        Ok(())
    }
}
