use std::{io::{self, Write}, time::Duration};

use anyhow::*;
use log::{error, info};
use lost_metrics_sniffer::{FakeSender, PacketSnifferServiceWrapper};
use simple_logger::SimpleLogger;
use tokio::{runtime::Runtime, time::sleep};

async fn runner() -> Result<()> {

    let mut service = PacketSnifferServiceWrapper::fake_tcp()?;

    let port = 80;
    let mut rx = service.start(port)?;

    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        let config = bincode::config::standard();        
        std::thread::sleep(std::time::Duration::from_secs(5));
        
        rt.block_on(async {
            let mut sender = FakeSender::new();
            sender.open().await?;

            loop {
                let packet = lost_metrics_sniffer::models::Packet::CounterAttack { source_id: 1 };
                let data = bincode::encode_to_vec(packet, config)?;

                sender.send(&data).await?;
                sleep(Duration::from_secs(1)).await;
            }

            Ok(())    
        })?;

        Ok(())
    });

    while let Some(packet) = rx.recv().await {
        info!("Received: {:?}", packet);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();

    match runner().await {
        Err(err) => error!("{}", err),
        _ => {}
    }

    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}