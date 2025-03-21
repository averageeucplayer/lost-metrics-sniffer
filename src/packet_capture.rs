use anyhow::*;

#[cfg(test)]
use mockall::automock;
    
#[cfg_attr(test, automock)]
pub trait PacketCapture : Send + Sync + 'static {
    fn start(&mut self, port: u16) -> Result<()>;
    fn recv(&mut self) -> Result<Vec<u8>>;
    fn close(&mut self) -> Result<()>;
}