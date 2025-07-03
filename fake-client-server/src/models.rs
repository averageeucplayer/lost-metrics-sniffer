use bincode::{Decode, Encode};
use clap::{arg, command, Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(name = "FakeClientServer")]
#[command(about = "App that spawns a child process with args")]
pub struct CommandArgs {
    #[arg(long, value_enum, default_value_t = ProcessType::Server)]
    pub r#type: ProcessType,

    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub ip_address: String,

    #[arg(long, default_value_t = 6040)]
    pub port: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ProcessType {
    Server,
    Child,
}