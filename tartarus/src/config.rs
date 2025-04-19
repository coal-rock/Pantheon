use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub address: IpAddr,
    pub token: Option<String>,
    /// A value of "0" has an unlimited history length
    pub history_buf_len: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            port: 8000,
            address: "0.0.0.0".parse().unwrap(),
            token: Some("bb123#123".to_string()),
            history_buf_len: 1000,
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
