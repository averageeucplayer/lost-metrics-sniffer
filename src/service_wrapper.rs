use std::{env, path::Path};

use abi_stable::{external_types::crossbeam_channel::RReceiver, library::lib_header_from_path, std_types::RResult};
use anyhow::*;
use crate::{models::Packet, service::{PacketSnifferServiceType, ServiceRoot_Ref}};

pub struct PacketSnifferServiceWrapper {
    service: PacketSnifferServiceType
}

impl PacketSnifferServiceWrapper {

    pub fn fake() -> Result<Self> {
        Self::new("fake_sniffer.dll")
    }

    pub fn windivert() -> Result<Self> {
        Self::new("windivert-sniffer.dll")
    }

    pub fn new(dll_name: &str) -> Result<Self> {
        let executable_path = env::current_exe()?;
        let executable_directory = executable_path.parent().unwrap();
        let library_path = executable_directory.join(dll_name);

        let header = lib_header_from_path(&library_path)?;
        let service_root = header.init_root_module::<ServiceRoot_Ref>()?;
        let service = service_root.new()();
        
        match service {
            RResult::ROk(service) => Ok({
                Self {
                    service
                }
            }),
            RResult::RErr(err) => Err(anyhow!(err)),
        }
    }

    pub fn start(&mut self, port: u16) -> Result<RReceiver<Packet>> {
        self.service.start(port)
            .map_err(|err| err.into())
            .into()
    }

    pub fn stop(&mut self) -> Result<()> {
        self.service.stop()
            .map_err(|err| err.into())
            .into()
    }
}