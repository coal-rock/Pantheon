use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub binary_path: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            binary_path: PathBuf::from("binaries/"),
        }
    }
}
