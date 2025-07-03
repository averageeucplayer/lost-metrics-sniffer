use std::{io::{self, Write}, pin::Pin, time::Duration};

use log::*;
use lost_metrics_sniffer::{models::Packet, PacketSnifferFactory};
use simple_logger::SimpleLogger;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();

    let mut sniffer = match PacketSnifferFactory::current().await {
        Ok(sniffer) => sniffer,
        Err(err) => {
            error!("{:?}", err);
            sleep(Duration::from_secs(5)).await;
            return;
        }
    };

    let recv = sniffer.start(6040).unwrap();
    let timeout_duration = Duration::from_millis(250);

    loop {
        let packet = recv.recv_timeout(timeout_duration).ok();

        if let Some(packet) = packet {
            println!("{:#?}", packet);
        }
    }

    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}