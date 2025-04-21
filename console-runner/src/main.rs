use std::{io::{self, Write}, pin::Pin};

use args::CommandArgs;
use clap::Parser;
use log::*;
use simple_logger::SimpleLogger;

mod examples;
mod args;

use examples::*;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();
    let args = CommandArgs::parse();

    let example: Pin<Box<dyn Future<Output = anyhow::Result<()>>>> = match args.example {
        args::Example::Tcp1 => Box::pin(async { fake_tcp_sniffer().await }),
        args::Example::Tcp2 => Box::pin(async { fake_tcp_sniffer_separate_thread().await }),
        args::Example::Windivert1 => Box::pin(async { fake_windivert_sniffer().await }),
        args::Example::Windivert2 => Box::pin(async { fake_windivert_sniffer_separate_thread().await }),
    };

    match example.await {
        Err(err) => error!("{}", err),
        _ => {}
    }

    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}