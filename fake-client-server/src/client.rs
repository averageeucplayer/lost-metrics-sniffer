use std::time::Duration;
use anyhow::Result;
use log::*;
use tokio::{io::AsyncReadExt, net::TcpStream, time::sleep};

pub struct Client {

}

impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, ip_address: String, port: u16) -> Result<()> {
        
        let address = format!("{}:{}", ip_address, port);
        let retry_delay = Duration::from_secs(3);
    
        loop {
            match TcpStream::connect(&address).await {
                Ok(mut stream) => {
                    info!("Connected to server at {}", address);
        
                    let mut buffer = vec![0; 512];
                    loop {
                        match stream.read(&mut buffer).await {
                            Ok(0) => {
                                warn!("Server closed connection.");
                                break;
                            }
                            Ok(_) => {}
                            Err(err) => {
                                error!("Failed to connect to {}: {}", address, err);
                                error!("Retrying in {} seconds...", retry_delay.as_secs());
                                sleep(retry_delay).await;
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("Failed to connect to {}: {}", address, err);
                    return Ok(())
                },
            }

        }
    }
}