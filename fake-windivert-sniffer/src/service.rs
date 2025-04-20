use std::{fmt::Display, marker::PhantomData, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread::{sleep, JoinHandle}, time::Duration};

use abi_stable::{export_root_module, external_types::{crossbeam_channel::{self, RReceiver, RSender}}, sabi_extern_fn, sabi_trait::TD_Opaque, std_types::{RBoxError, RResult::{self, RErr, ROk}}, StableAbi};
use rand::{rng, Rng};
use lost_metrics_sniffer::{PacketCapture, PacketSnifferService, PacketSnifferServiceType, PacketSnifferService_TO, ServiceRoot, ServiceRoot_Ref, TokioMpscWrapper};
use lost_metrics_sniffer::models::Packet;
use abi_stable::prefix_type::PrefixTypeTrait;
use tokio::{runtime::Runtime, sync::mpsc::{UnboundedReceiver, UnboundedSender}};

use std::error::Error;

use crate::{error::PacketSnifferServiceError, packet_capturer::FakePacketCapturer};

#[export_root_module]
fn instantiate_root_module() -> ServiceRoot_Ref {
    ServiceRoot { new }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn new() -> RResult<PacketSnifferServiceType, RBoxError> {
    let this: FakeService = FakeService {
        handle: None,
        close_flag: Arc::new(AtomicBool::new(false)),
    };
    ROk(PacketSnifferService_TO::from_value(this, TD_Opaque))
}

struct FakeService {
    handle: Option<JoinHandle<anyhow::Result<()>>>,
    close_flag: Arc<AtomicBool>,
}

impl PacketSnifferService for FakeService {
    
    fn start(&mut self, port: u16) -> RResult<TokioMpscWrapper, RBoxError> {

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Packet>();
        let close_flag = self.close_flag.clone();
        let handle = std::thread::spawn(move || Self::listen(port, close_flag, tx));

        self.handle = Some(handle);

        ROk(TokioMpscWrapper::new(rx))
    }

    fn stop(&mut self) -> RResult<(), RBoxError> {

        if let Some(handle) = self.handle.take() {
            self.close_flag.store(true, Ordering::Relaxed);
            let result = handle.join();

            match result {
                Ok(result) => return ROk(()),
                Err(err) => return RErr(RBoxError::new(PacketSnifferServiceError::Stop(format!("{:?}", err).into()))),
            }
        }

        ROk(())
    }
    
}

impl FakeService {

    fn listen(port:u16, close_flag: Arc<AtomicBool>, tx: UnboundedSender<Packet>) -> anyhow::Result<()> {
        let rt = Runtime::new().unwrap();
        let mut packet_capturer = FakePacketCapturer::new();
        packet_capturer.start(port);
        let config = bincode::config::standard();

        loop {
            if close_flag.load(Ordering::Relaxed) {
                break;
            }

            let data = packet_capturer.recv()?;
            let (packet, _)  = bincode::decode_from_slice(&data, config)?;
            tx.send(packet)?;
        }

        packet_capturer.close()?;

        anyhow::Ok(())
    }
}