
use std::time::Duration;

use anyhow::*;
use log::*;
use lost_metrics_sniffer::{FakeSender, PacketSnifferServiceWrapper};
use tokio::time::sleep;

use crate::examples::utils::*;

pub async fn fake_windivert_sniffer_separate_thread() -> Result<()> {

    debug!("running example fake_windivert_sniffer_separate_thread");
    let mut service = PacketSnifferServiceWrapper::fake_tcp()?;
    let port = 6042;
    let mut rx = service.start(port)?;

    let handle = separate_thread();

    loop {
        match rx.recv().await {
            Some(packet) => {
                debug!("recv: {packet:?}");
            },
            None => {
                debug!("Stopped.");
                break;
            },
        }
        sleep(Duration::from_secs(1)).await;
    }

    handle.join()
        .map_err(|err| anyhow::anyhow!("{:?}", err))??;

    Ok(())
}


pub async fn fake_windivert_sniffer() -> Result<()> {

    debug!("running example fake_windivert_sniffer");
    let mut service = PacketSnifferServiceWrapper::fake_windivert()?;
    let port = 6042;
    let mut rx = service.start(port)?;

    let config = bincode::config::standard();
    let mut sender = FakeSender::new();
    sender.open().await?;
    debug!("Start");

    loop {
        tokio::select! {
            packet = rx.recv() => {
                match packet {
                    Some(packet) => {
                        debug!("recv: {packet:?}");
                    },
                    None => {
                        debug!("Stopped.");
                        break;
                    },
                }
            }
            _ = async {
                let packet = random_packet();
                let data = bincode::encode_to_vec(&packet, config)?;
                info!("Sending... {}", get_type_name(packet));
                sender.send(&data).await?;
                anyhow::Ok(())
            } => {}
        }
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}