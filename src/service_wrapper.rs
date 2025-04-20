use std::{env, fs::File, io::copy, path::Path};

use abi_stable::{external_types::crossbeam_channel::RReceiver, library::lib_header_from_path, std_types::RResult};
use anyhow::*;
use log::debug;
use octocrate::{APIConfig, GitHubAPI, PersonalAccessToken, Release};
use reqwest::Client;
use crate::{models::Packet, service::{PacketSnifferServiceType, ServiceRoot_Ref}, TokioMpscWrapper};

pub struct PacketSnifferServiceWrapper {
    service: PacketSnifferServiceType
}

impl PacketSnifferServiceWrapper {

    pub async fn download() -> Result<()> {
        let api_url = option_env!("API_URL").expect("API_URL not set"   );
        let github_user = option_env!("GITHUB_USER").expect("GITHUB_USER not set");
        let github_project = option_env!("GITHUB_PROJECT").expect("GITHUB_PROJECT not set");
        let personal_access_token_str = option_env!("GITHUB_PAT").expect("GITHUB_PAT not set");

        let personal_access_token = PersonalAccessToken::new(personal_access_token_str);

        let config = APIConfig::with_token(personal_access_token).shared();
      
        let api = GitHubAPI::new(&config);
        let result = api.repos.get_latest_release(github_user, github_project).send().await;
        let client = Client::new();

        match result {
            std::result::Result::Ok(release) => {
                let release: Release = release;
    
                let assets: Vec<_> = release.assets.iter()
                    .filter(|pr| pr.name.ends_with(".dll"))
                    .collect();
    
                let asset = assets.first().unwrap();
                let response = client.get(&asset.browser_download_url).send().await?;
                let mut file = File::create(&asset.name)?;
                debug!("Downloading file: {:?}", asset.name);
                let bytes = response.bytes().await?;
                let mut content = bytes.as_ref();
                copy(&mut content, &mut file)?;
            },
            Err(err) => println!("{:?}", err)
        };

        Ok(())
    }

    pub fn fake_windivert() -> Result<Self> {
        Self::new("fake_windivert_sniffer.dll")
    }

    pub fn fake_tcp() -> Result<Self> {
        Self::new("fake_tcp_sniffer.dll")
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

    pub fn start(&mut self, port: u16) -> Result<TokioMpscWrapper> {
        let rx = self.service.start(port).unwrap();
        Ok(rx)
    }

    pub fn stop(&mut self) -> Result<()> {
        self.service.stop()
            .map_err(|err| err.into())
            .into()
    }
}