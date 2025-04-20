use abi_stable::std_types::ROption::{self, RSome};
use rand::{distr::Alphanumeric, rng, rngs::ThreadRng, Rng};
use lost_metrics_sniffer::{models::{Packet, SkillDamageEvent, SkillMoveOptionData}, packet_capture::PacketCapture};
use anyhow::*;
use windivert::{layer::NetworkLayer, prelude::WinDivertFlags, WinDivert};

pub struct FakePacketCapturer {
    buffer: Vec<u8>,
    windivert: Option<WinDivert<NetworkLayer>>
}

impl FakePacketCapturer {

    pub fn start(&mut self, port: u16) -> Result<()> {
        let filter = format!("tcp.SrcPort == {}", port);
        let flags = WinDivertFlags::new().set_recv_only().set_sniff();
        let windivert = WinDivert::network(&filter, 0, flags)?;
        self.windivert = Some(windivert);

        Ok(())
    }

    pub fn recv(&mut self) -> Result<Vec<u8>> {
        
        let windivert = self.windivert.as_ref().unwrap();
        let data = windivert.recv(Some(&mut self.buffer))?;
        Ok(data.data.to_vec())
    }

    pub fn close(&mut self) -> Result<()> {
        

        Ok(())
    }

    pub fn new() -> Self {
        let buffer = vec![0u8; 65535];

        Self {
            buffer,
            windivert: None
        }
    }
}