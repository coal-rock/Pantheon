use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub address: IpAddr,
    pub binary_path: PathBuf,
    pub token: Option<String>,
    pub history_buf_len: Option<usize>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            port: 8000,
            address: "0.0.0.0".parse().unwrap(),
            binary_path: PathBuf::from("binaries/"),
            token: Some("bb123#123".to_string()),
            history_buf_len: Some(1000),
        }
    }
}

impl Config {
    pub fn new(file_path: &str) -> Config {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(file_path))
            .merge(Env::prefixed(""))
            .extract()
            .unwrap()
    }
}
