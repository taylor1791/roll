use crate::expression::{Evaluand, Expression};
use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter};

pub struct JsonFormatter {
    evaluand: Evaluand,
}

impl JsonFormatter {
    pub fn from(evaluand: Evaluand) -> Self {
        JsonFormatter { evaluand }
    }
}

pub struct TextFormatter {
    colors: bool,
    evaluand: Evaluand,
    expression: Expression,
}

impl TextFormatter {
    pub fn from(args: super::Arguments, evaluand: Evaluand) -> Self {
        TextFormatter {
            colors: args.use_colors(),
            evaluand,
            expression: args.expression,
        }
    }
}

impl Display for JsonFormatter {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        let rolls = self
            .evaluand
            .rolls
            .iter()
            .map(|(k, v)| (format!("d{}", k), v))
            .collect::<std::collections::HashMap<_, _>>();

        formatter.write_fmt(format_args!(
            "{}",
            serde_json::json!({
                "value": self.evaluand.value,
                "rolls": rolls
            })
        ))
    }
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
                let style = if *roll == 1 {
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
