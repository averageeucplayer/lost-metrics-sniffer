use std::time::Duration;

use log::*;
use lost_metrics_sniffer::models::Class;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::sleep};
use anyhow::Result;

use crate::packet_builder::{PacketBuilder};

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, ip_address: String, port: u16) -> Result<()> {

        let address = format!("{}:{}", ip_address, port);
        let listener = TcpListener::bind(&address).await?;
        info!("Server running on {}", address);
        let mut streams: Vec<TcpStream> = Vec::new();
        let mut runner = PacketBuilder::new()
            .add_new_pc("Playera", Class::Bard as u32, 1700.0)
            .add_new_pc("Playerb", Class::Breaker as u32, 1700.0)
            .add_new_pc("Playerc", Class::Scrapper as u32, 1700.0)
            .add_new_pc("Playerd", Class::Aeromancer as u32, 1700.0)
            .add_new_pc("Playere", Class::Paladin as u32, 1700.0)
            .add_new_pc("Playerf", Class::Deathblade as u32, 1700.0)
            .add_new_pc("Playerg", Class::Gunlancer as u32, 1700.0)
            .add_new_pc("Playerh", Class::Gunslinger as u32, 1700.0)
            .add_init_pc("Playera")
            .add_party_info(vec!["Playera", "Playerb", "Playerc", "Playerd"])
            .add_party_info(vec!["Playere", "Playerf", "Playerg", "Playerh"])
            .add_new_npc(485800, 1680, 1.4e12 as i64)
            .add_raid_begin(2)
            .add_skill_damage(1e9 as i64, 1e10 as i64)
            .build();

        loop {
            tokio::select! {
                Ok((stream, addr)) = listener.accept() => {
                    info!("New client: {}", addr);
                    streams.push(stream);
                }

                _ = sleep(Duration::from_secs(1)) => {

                    let bytes = match runner.next_packet() {
                        Some(bytes) => bytes,
                        None => return Ok(())
                    };

                    debug!("Sending...");

                    let mut still_connected = Vec::new();
                    for mut stream in streams.drain(..) {
                        match stream.write_all(&bytes).await {
                            Ok(_) => {
                                still_connected.push(stream);
                            }
                            Err(e) => {
                                warn!("Client disconnected: {}", e);
                            }
                        }
                    }
                    streams = still_connected;
                }
            }
        }

        Ok(())
    }
}