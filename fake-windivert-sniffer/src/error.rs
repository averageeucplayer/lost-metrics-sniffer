
use std::{error::Error, fmt::Display};

use abi_stable::{std_types::RString, StableAbi};

#[repr(u8)]
#[derive(Debug, StableAbi)]
pub enum PacketSnifferServiceError {
    NotAdmin(RString),
    Recv(RString),
    Send(RString),
    Stop(RString),
    Close(RString),
}

impl Display for PacketSnifferServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketSnifferServiceError::NotAdmin(err) =>  writeln!(f, "Error occurred whilst running windivert: {}", err),
            PacketSnifferServiceError::Recv(err) =>  writeln!(f, "Error occurred whilst receiving data: {}", err),
            PacketSnifferServiceError::Send(err) => writeln!(f, "Error occurred whilst sending to mpsc: {}", err),
            PacketSnifferServiceError::Stop(err) => writeln!(f, "Error occurred whilst stopping: {}", err),
            PacketSnifferServiceError::Close(err) => writeln!(f, "Error occurred whilst closing windivert handle: {}", err),
        }
    }
}
impl Error for PacketSnifferServiceError {}