use clap::{arg, command, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "ConsoleRunner")]
#[command(about = "Run example")]
pub struct CommandArgs {
    #[arg(long, value_enum, default_value_t = Example::Tcp1)]
    pub example: Example,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum Example {
    Tcp1,
    Tcp2,
    Windivert1,
    Windivert2,
}