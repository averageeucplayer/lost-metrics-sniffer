use anyhow::{Ok, Result};
use log::warn;
use windivert::{layer::NetworkLayer, prelude::WinDivertFlags, CloseAction, WinDivert};

pub struct WinDivertWrapper {
    buffer: Vec<u8>,
    windivert: Option<WinDivert<NetworkLayer>>
}

impl WinDivertWrapper {

    pub fn start(&mut self, port: u16) -> Result<()> {

        if self.windivert.is_some() {
            // TO-DO
            warn!("Attempt to start windivert without closing previous handle");
            return Ok(())
        }

        let filter = format!("inbound && tcp.SrcPort == {}", port);
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
        if let Some(mut windivert) = self.windivert.take() {
            windivert.close(CloseAction::Nothing)?;
        }

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