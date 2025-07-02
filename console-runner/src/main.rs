use std::{io::{self, Write}, pin::Pin};

use log::*;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();

    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}