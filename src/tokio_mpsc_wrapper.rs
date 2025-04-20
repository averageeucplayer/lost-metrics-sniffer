use std::ops::{Deref, DerefMut};

use abi_stable::{*, library::RootModule, sabi_types::VersionStrings, std_types::{RBox, RBoxError, RResult}, StableAbi};
use abi_stable::external_types::crossbeam_channel::RReceiver;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::models::Packet;

#[derive(StableAbi)]
#[repr(C)]
pub struct TokioMpscWrapper(*mut ());

impl TokioMpscWrapper {
    pub fn new(rx: UnboundedReceiver<Packet>) -> Self {
        Self(Box::into_raw(Box::new(rx)) as *mut ())
    }
}

impl Deref for TokioMpscWrapper {
    type Target = UnboundedReceiver<Packet>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.0 as *const Self::Target) }
    }
}

impl DerefMut for TokioMpscWrapper {

    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.0 as *mut Self::Target) }
    }
}