use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Ok, Result};
use interprocess::os::windows::named_pipe::{pipe_mode, tokio::{DuplexPipeStream, PipeStream}};
use log::*;
use tokio::io::AsyncWriteExt;

pub struct FakeSender {
    sender: Option<PipeStream<pipe_mode::None, pipe_mode::Bytes>>,
}

impl FakeSender {
    pub fn new() -> Self {
        Self {
            sender: None
        }
    }

    pub async fn open(&mut self) -> Result<()> {
        let pipe_name = "Collector";
        let pipe_path = format!("\\\\.\\pipe\\{}", pipe_name);
        let connection = DuplexPipeStream::<pipe_mode::Bytes>::connect_by_path(pipe_path).await?;
    
        let (rx, sender) = connection.split();

        self.sender = Some(sender);

        Ok(())
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<()> {
    
        let mut sender = self.sender.as_mut().unwrap();
        sender.write_all(&data).await?;
       
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.sender.as_mut().unwrap().shutdown().await?;
        Ok(())
    }
}