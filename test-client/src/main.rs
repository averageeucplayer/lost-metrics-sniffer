use anyhow::*;
use log::info;
use lost_metrics_sniffer::PacketSnifferServiceWrapper;
use simple_logger::SimpleLogger;

fn runner() -> Result<()> {
    SimpleLogger::new().env().init().unwrap();

    let mut service = PacketSnifferServiceWrapper::fake()?;

    let port = 80;
    let rx = service.start(port)?;

    while let std::result::Result::Ok(packet) = rx.recv() {
        info!("Received: {:?}", packet);
    }

    Ok(())
}

fn main() {
    match runner() {
        Err(err) => info!("{}", err),
        _ => {}
    }
}