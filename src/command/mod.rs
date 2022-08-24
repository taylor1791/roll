mod roll;

pub use roll::Roll;

pub trait Command {
    type Output;

    fn exec(&self) -> Result<Self::Output, anyhow::Error>;

    fn formatter(
        self,
        args: crate::cli::Arguments,
        output: Self::Output,
    ) -> Box<dyn std::fmt::Display>;
}
