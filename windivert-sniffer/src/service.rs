use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread::{sleep, JoinHandle}, time::Duration};

use abi_stable::{export_root_module, external_types::crossbeam_channel::{self, RReceiver, RSender}, sabi_extern_fn, sabi_trait::TD_Opaque, std_types::{RBoxError, RResult::{self, RErr, ROk}, RString}, StableAbi};
use lost_metrics_sniffer::{PacketSnifferService, PacketSnifferServiceType, PacketSnifferService_TO, ServiceRoot, ServiceRoot_Ref};
use lost_metrics_sniffer::models::Packet;
use abi_stable::prefix_type::PrefixTypeTrait;
use windivert::{prelude::WinDivertFlags, CloseAction, WinDivert};
use crate::{error::PacketSnifferServiceError, windivert_wrapper::WinDivertWrapper};

#[export_root_module]
fn instantiate_root_module() -> ServiceRoot_Ref {
    ServiceRoot { new }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn new() -> RResult<PacketSnifferServiceType, RBoxError> {
    let this = WindivertService {
        handle: None,
        close_flag: Arc::new(AtomicBool::new(false))
    };
    ROk(PacketSnifferService_TO::from_value(this, TD_Opaque))
}

struct WindivertService {
    handle: Option<JoinHandle<RResult<(), RBoxError>>>,
    close_flag: Arc<AtomicBool>,
}

impl PacketSnifferService for WindivertService {
    
    fn start(&mut self, port: u16) -> RResult<RReceiver<Packet>, RBoxError> {

        let (tx, rx) = crossbeam_channel::unbounded::<Packet>();
        let close_flag = self.close_flag.clone();
        let handle = std::thread::spawn(move || Self::listen(port, tx, close_flag));

        self.handle = Some(handle);

        ROk(rx)
    }

    fn stop(&mut self) -> RResult<(), RBoxError> {

        if let Some(handle) = self.handle.take() {
            self.close_flag.store(true, Ordering::Relaxed);
            let result = handle.join();

            match result {
                Ok(result) => return result,
                Err(err) => return RErr(RBoxError::new(PacketSnifferServiceError::Stop(format!("{:?}", err).into()))),
            }
        }

        ROk(())
    }

}

impl WindivertService {

    fn listen(port: u16,  tx: RSender<Packet>, close_flag: Arc<AtomicBool>,) -> RResult<(), RBoxError> {
        let filter = format!("inbound && tcp.SrcPort == {}", port);
        let flags = WinDivertFlags::new().set_recv_only().set_sniff();
        let mut windivert = WinDivert::network(&filter, 0, flags)?;
        let mut buffer = vec![0u8; 65535];

        loop {
            if close_flag.load(Ordering::Relaxed) {
                break;
            }

            let data = match windivert.recv(Some(&mut buffer)) {
                Ok(data) => data.data,
                Err(err) => return RErr(RBoxError::new(PacketSnifferServiceError::Recv(format!("{:?}", err).into()))),
            };

            if let Some(packet) = Self::process_data(&data) {
                if let Err(err) = tx.send(packet) {
                    return RErr(RBoxError::new(PacketSnifferServiceError::Send(format!("{:?}", err).into())));
                }
            }

        }

        if let Err(err) = windivert.close(CloseAction::Nothing) {
            return RErr(RBoxError::new(PacketSnifferServiceError::Close(format!("{:?}", err).into())));
        }

        ROk(())
    }

    fn process_data(_data: &[u8]) -> Option<Packet> {
        None
    }
}