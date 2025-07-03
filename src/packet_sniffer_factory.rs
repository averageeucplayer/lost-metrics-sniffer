use std::{env, fs::File, io::{copy, Write}, path::{Path, PathBuf}};

use abi_stable::{external_types::crossbeam_channel::RReceiver, library::lib_header_from_path, std_types::RResult};
use anyhow::*;
use log::debug;
use octocrate::{APIConfig, GitHubAPI, PersonalAccessToken, Release};
use reqwest::Client;
use crate::{models::Packet, service::{PacketSnifferServiceType, ServiceRoot_Ref}};

pub struct PacketSnifferFactory {
    
}

impl PacketSnifferFactory {

    pub async fn current() -> Result<PacketSnifferServiceType> {

        let executable_path = env::current_exe()?;
        let executable_directory = executable_path.parent().unwrap();
        let library_path = PathBuf::from("windivert_sniffer.dll");
        let header = lib_header_from_path(&library_path)?;
        let service_root = header.init_root_module::<ServiceRoot_Ref>()?;
        let service = service_root.new()();

        match service {
            RResult::ROk(service) => Ok({
                service
            }),
            RResult::RErr(err) => Err(anyhow!(err)),
        }

    }

    pub async fn get_latest() -> Result<PacketSnifferServiceType> {

        let url = "https://github.com/averageeucplayer/lost-metrics-sniffer/latest";
        let file_name = "sniffer.dll";
        let executable_path = env::current_exe()?;
        let executable_directory = executable_path.parent().unwrap();
        let library_path = executable_directory.join(file_name);

        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(anyhow!("Could not download latest sniffer"));
        }

        let content =  response.bytes().await?;
        let mut file = File::create(&library_path)?;
        file.write_all(&content)?;

        let header = lib_header_from_path(&library_path)?;
        //header.version_strings().version
        let service_root = header.init_root_module::<ServiceRoot_Ref>()?;
        let service = service_root.new()();

        match service {
            RResult::ROk(service) => Ok({
                service
            }),
            RResult::RErr(err) => Err(anyhow!(err)),
        }
   }
}