use super::Command;
use ibig::IBig;
use owo_colors::OwoColorize;
use roll::{
    expression::Expression,
    pmf::{Outcome, Pmf as ExpressionPmf},
};
use std::fmt::{Display, Formatter};
use terminal_size::terminal_size;

#[derive(Debug)]
pub struct Pmf {
    expression: Expression,
}

impl Pmf {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

pub struct TextFormatter {
    colors: bool,
    pmf: ExpressionPmf<IBig>,
    expression: Expression,
}

impl Command for Pmf {
    type Output = ExpressionPmf<IBig>;

    fn exec(&self) -> Result<Self::Output, anyhow::Error> {
        self.expression.pmf()
    }

    fn formatter(
        self,
        args: crate::cli::Arguments,
        pmf: Self::Output,
    ) -> Box<dyn std::fmt::Display> {
        Box::from(TextFormatter {
            colors: args.use_colors(),
            pmf,
            expression: self.expression,
        })
    }
}

impl Display for TextFormatter {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        if self.colors {
            formatter.write_fmt(format_args!(
                "{} {}\n",
                "Expression:".magenta().bold(),
                self.expression.to_string().blue(),
            ))?;
            formatter.write_fmt(format_args!(
                "  {} {:.2}\n\n",
                "Mean:".cyan().bold(),
                self.pmf.expected_value(),
            ))?;
        }

        let max_digits = self
            .pmf
            .iter()
            .last()
            .map(|outcome| outcome.value.to_string().len())
            .unwrap_or(0);
        let max_p = self
            .pmf
            .iter()
            .map(|outcome| outcome.p)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        for Outcome { p, value: roll } in self.pmf.iter() {
            let padding = "  ";
            formatter.write_fmt(format_args!(
                "{}{:>align$}",
                padding,
                roll.if_supports_color(owo_colors::Stream::Stdout, |text| text
                    .style(owo_colors::Style::new().blue().bold())),
                align = max_digits
            ))?;

            if self.colors {
                if let Some(width) = terminal_size().map(|(width, _)| width) {
                    let padding = " ";
                    let percent_chars = 8;
                    let other_chars =
                        padding.len() + max_digits + padding.len() + padding.len() + percent_chars;

                    let max_width =
                        Into::<usize>::into(width.0).min(75).max(other_chars) - other_chars;
                    let bar_width = (p / max_p) * max_width as f64;

                    formatter.write_fmt(format_args!(
                        "{}{}{}",
                        padding,
                        "▬".repeat(bar_width as usize).blue(),
                        padding
                    ))?;
                }
            }

            formatter.write_fmt(format_args!("{:>6.2}%\n", p * 100.0))?;
        }

        Ok(())
    }
}
