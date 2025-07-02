use std::{io::{self, Write}, pin::Pin, time::Duration};

use log::*;
use lost_metrics_sniffer::{models::Packet, PacketSnifferFactory};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();

    let mut sniffer = PacketSnifferFactory::windivert().await.unwrap();

    let recv = sniffer.start(6040).unwrap();
    let timeout_duration = Duration::from_secs(1);

    loop {
        let packet = recv.recv_timeout(timeout_duration).ok();

        if let Some(packet) = packet {
            print!("{packet:?}")
        }
    }

    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}