use abi_stable::std_types::ROption::{self, RSome};
use rand::{distr::Alphanumeric, rng, rngs::ThreadRng, Rng};
use lost_metrics_sniffer::{models::{Packet, SkillDamageEvent, SkillMoveOptionData}, packet_capture::PacketCapture};
use anyhow::*;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

pub struct FakePacketCapturer {
    port: u16,
    ip_address: String,
    client: Option<TcpStream>,
}

impl FakePacketCapturer {

    pub async fn start(&mut self, port: u16) -> Result<()> {
        self.port = port;

        let address = format!("{}:{}", self.ip_address, self.port);
        let client = TcpStream::connect(&address).await?;

        self.client = Some(client);

        Ok(())
    }

    pub async fn recv(&mut self) -> Result<Vec<u8>> {
        let mut buffer = vec![0; 512];
        let mut client = self.client.as_mut().unwrap();

        client.read(&mut buffer).await?;
       
        Ok(buffer)
    }

    pub async fn close(&mut self) -> Result<()> {

        if let Some(mut client) = self.client.take() {
            client.shutdown().await?;
        }

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            port: 0,
            ip_address: String::from("127.0.0.1"),
            client: None
        }
    }
}