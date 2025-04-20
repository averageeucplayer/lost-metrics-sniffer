use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread::{sleep, JoinHandle}, time::Duration};

use abi_stable::{export_root_module, external_types::crossbeam_channel::{self, RReceiver, RSender}, sabi_extern_fn, sabi_trait::TD_Opaque, std_types::{RBoxError, RResult::{self, RErr, ROk}, RString}, StableAbi};
use lost_metrics_sniffer::{PacketCapture, PacketSnifferService, PacketSnifferServiceType, PacketSnifferService_TO, ServiceRoot, ServiceRoot_Ref, TokioMpscWrapper};
use lost_metrics_sniffer::models::Packet;
use abi_stable::prefix_type::PrefixTypeTrait;
use tokio::sync::mpsc::UnboundedSender;
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
    
    fn start(&mut self, port: u16) -> RResult<TokioMpscWrapper, RBoxError> {

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Packet>();
        let close_flag = self.close_flag.clone();
        let handle = std::thread::spawn(move || Self::listen(close_flag, tx));

        self.handle = Some(handle);

        ROk(TokioMpscWrapper::new(rx))
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

    fn listen(close_flag: Arc<AtomicBool>, tx: UnboundedSender<Packet>) -> RResult<(), RBoxError> {
        let mut packet_capturer = WinDivertWrapper::new();

        loop {
            if close_flag.load(Ordering::Relaxed) {
                break;
            }

            let data = match packet_capturer.recv() {
                Ok(data) => data,
                Err(err) => return RErr(RBoxError::new(PacketSnifferServiceError::Recv(format!("{:?}", err).into()))),
            };

            if let Some(packet) = Self::process_data(&data) {
                if let Err(err) = tx.send(packet) {
                    return RErr(RBoxError::new(PacketSnifferServiceError::Send(format!("{:?}", err).into())));
                }
            }

            #[cfg(test)]
            sleep(Duration::from_secs(1));
        }

        if let Err(err) = packet_capturer.close() {
            return RErr(RBoxError::new(PacketSnifferServiceError::Close(format!("{:?}", err).into())));
        }

        ROk(())
    }

    fn process_data(_data: &[u8]) -> Option<Packet> {
        None
    }
}

#[cfg(test)]
mod tests {
    use abi_stable::std_types::vec;
    use mockall::{mock, predicate::eq};
    use super::*;
    use anyhow::*;

    // #[cfg(test)]
    // mock! {
    //     pub PacketCapture {}
    //     impl PacketCapture for PacketCapture {
    //         fn start(&mut self, port: u16) -> Result<()>;
    //         fn recv(&mut self) -> Result<Vec<u8>>;
    //         fn close(&mut self) -> Result<()>;
    //     }
    // }

    // #[test]
    // fn should_call_packet_capture() {
    //     let port = 1234;
    //     let mut packet_capturer= MockPacketCapture::new();

    //     packet_capturer
    //         .expect_start()
    //         .with(eq(port))
    //         .returning(|_| Ok(()));
        
    //     packet_capturer
    //         .expect_recv()
    //         .returning(|| Ok(vec![]));

    //     packet_capturer
    //         .expect_close()
    //         .returning(|| Ok(()));

    //     let packet_capturer=  Arc::new(Mutex::new(packet_capturer));

    //     let mut service = WindivertService {
    //         handle: None,
    //         close_flag: Arc::new(AtomicBool::new(false)),
    //         packet_capturer,
    //     };

    //     service.start(port).unwrap();
    //     sleep(Duration::from_secs(1));
    //     service.stop().unwrap();
    // }
}