use std::{path::Path, pin::Pin, time::Duration};

use interprocess::os::windows::named_pipe::{pipe_mode, tokio::{PipeListener, PipeListenerOptionsExt}, PipeListenerOptions};
use log::*;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, time::sleep};
use anyhow::Result;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, ip_address: String, port: u16, pipe_name: String) -> Result<()> {
        
        let address = format!("{}:{}", ip_address, port);
        let listener = TcpListener::bind(&address).await?;
        let ipc_listener = Self::setup_ipc_server(&pipe_name).await?;
        info!("Server running on {}", address);

        let (tx, rx) = async_channel::unbounded::<Vec<u8>>();
        let mut ipc_handler: Option<tokio::task::JoinHandle<std::result::Result<(), anyhow::Error>>> = None;
        let mut streams: Vec<tokio::net::TcpStream> = Vec::new();
            
        loop {

            tokio::select! {
                _ = async {
                    if let Some(handle) = &ipc_handler {
                        if handle.is_finished() {
                            match ipc_handler.take().unwrap().await {
                                Ok(Ok(())) => info!("Previous ipc handler completed cleanly."),
                                Ok(Err(e)) => error!("Previous ipc handler returned error: {:?}", e),
                                Err(e) => error!("Previous ipc handler panicked: {:?}", e),
                            }
                        }
                    }
                } => {}
                ipc_stream = ipc_listener.accept() => {
                    match ipc_stream {
                        Ok(stream) => {
                     

                            info!("New ipc client connected");

                            let tx = tx.clone();

                            let handle = tokio::spawn(async move {
                                let (mut ipc_rx, ipc_tx) = stream.split();
                                let mut buffer = [0u8; 65535];

                                loop {
                                    let bytes_read = ipc_rx.read(&mut buffer).await?;
                            
                                    if bytes_read == 0 {
                                        debug!("Client disconnected, shutting down.");
                                        drop((ipc_rx, ipc_tx));
                                        break;
                                    }
                            
                                    let data = buffer[..bytes_read].to_vec();
                                    tx.send(data).await?;
                                }

                                anyhow::Ok(())
                            });

                            ipc_handler = Some(handle);
                        },
                        Err(err) => error!("Failed to accept ipc connection: {}", err),
                    }
                }
                data = rx.recv() => {
                    match data {
                        Ok(data) => {
                            let mut failed_indices = Vec::new();

                            for (i, stream) in streams.iter_mut().enumerate() {
                                if let Err(err) = stream.write_all(&data).await {
                                    if err.raw_os_error().filter(|pr| *pr != 10054).is_some() {
                                        error!("Error writing to stream {}: {}", i, err);
                                    }
                                    failed_indices.push(i);
                                }
                            }

                            for i in failed_indices.iter().rev() {
                                streams.remove(*i);
                            }
                        },
                        Err(err) =>  error!("Failed at receiving {}", err),
                    }
                }
                stream = listener.accept() => {
                    match stream {
                        Ok((stream, addr)) => {
                            info!("New client connected: {}", addr);

                            streams.push(stream);
                        }
                        Err(err) => error!("Failed to accept connection: {}", err),
                    }
                }
            } 
        }

        Ok(())
    }

    pub async fn setup_ipc_server(pipe_name: &str) -> Result<PipeListener<pipe_mode::Bytes, pipe_mode::Bytes>> {
        let pipe_path = format!("\\\\.\\pipe\\{}", pipe_name);
        let pipe_path = Path::new(pipe_path.as_str());
    
        let ipc_listener= PipeListenerOptions::new()
            .path(pipe_path)
            .create_tokio_duplex::<pipe_mode::Bytes>()?;
            // .create_tokio_recv_only::<pipe_mode::Bytes>()?;

        let pipe_path = format!("\\\\.\\pipe\\{}", pipe_name);
        let pipe_path = Path::new(pipe_path.as_str());
        info!("Accepting data at {}", pipe_path.display());

        Ok(ipc_listener)
    }
}