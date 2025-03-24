use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub address: IpAddr,
    pub binary_path: PathBuf,
    pub token: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            port: 8000,
            address: "127.0.0.1".parse().unwrap(),
            binary_path: PathBuf::from("binaries/"),
            token: Some("bb123#123".to_string()),
        }
    }
}
