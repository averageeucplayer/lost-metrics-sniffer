use std::time::Duration;

use log::*;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::sleep};
use anyhow::Result;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, ip_address: String, port: u16, pipe_name: String) -> Result<()> {
        
        let address = format!("{}:{}", ip_address, port);
        let listener = TcpListener::bind(&address).await?;
        info!("Server running on {}", address);

        let mut streams: Vec<TcpStream> = Vec::new();

        loop {
            tokio::select! {
                Ok((stream, addr)) = listener.accept() => {
                    info!("New client: {}", addr);
                    streams.push(stream);
                }

                _ = sleep(Duration::from_secs(1)) => {

                    let msg = format!("Hello from server via {}!\n", pipe_name);

                    let mut still_connected = Vec::new();
                    for mut stream in streams.drain(..) {
                        match stream.write_all(msg.as_bytes()).await {
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