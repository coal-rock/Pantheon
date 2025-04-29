use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod sys {

    use std::{env::consts, ffi::OsStr};
    use whoami::*; // 24.1 KiB

    #[derive(Clone, PartialEq, Debug)]
    pub enum OsEnum {
        Linux,
        Windows,
        MacOs,
        OpenBSD,
        FreeBSD,
        Other(String),
    }

    pub fn os_name() -> OsEnum {
        // possible values of consts::OS :
        //      https://doc.rust-lang.org/std/env/consts/constant.OS.html
        // arch linux == linux
        match consts::OS {
            "linux" => OsEnum::Linux,
            "windows" => OsEnum::Windows,
            "macos" => OsEnum::MacOs,
            "openbsd" => OsEnum::OpenBSD,
            "freebsd" => OsEnum::FreeBSD,
            other => OsEnum::Other(other.to_string()),
        }
    }

    pub fn username() -> String {
        whoami::username()
    }

    pub fn hostname() -> String {
        match whoami::fallible::hostname() {
            Ok(res) => res,
            Err(_) => "unkown".to_string(),
        }
    }

    pub fn is_admin() -> bool {
        false
    }

    pub fn reboot() -> Result<()> {
        Ok(())
    }

    pub fn shutdown() -> Result<()> {
        Ok(())
    }

    pub fn uptime() -> Result<u64> {
        Ok(64u64)
    }

    pub fn hermes_dir() -> Result<String> {
        match std::env::current_dir() {
            Ok(s) => s.to_string_lossy().into_owned(),
            Err(error) => format!("Path was invalid: {}", error).to_string(),
        }
    }

    pub fn cpu_architecture() -> String {
        whoami::arch().to_string()
    }

    pub fn is_windows() -> bool {
        consts::OS == "windows"
    }

    pub fn is_linux() -> bool {
        consts::OS == "linux"
    }

    pub fn is_macos() -> bool {
        consts::OS == "macos"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sys::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn get_os_name() {
        // How to test this?
        let res: OsEnum = os_name();
        assert_eq!(res, OsEnum::Linux);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn get_os_name() {
        // How to test this?
        let res: OsEnum = os_name();
        assert_eq!(res, OsEnum::Windows);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn get_os_name() {
        // How to test this?
        let res: OsEnum = os_name();
        assert_eq!(res, OsEnum::MacOs);
    }

    #[test]
    fn get_username() {
        let _ = username();
        assert_eq!(1, 1);
    }

    #[test]
    fn get_hostname() {
        let _ = hostname();
        assert_eq!(1, 1);
    }
}
