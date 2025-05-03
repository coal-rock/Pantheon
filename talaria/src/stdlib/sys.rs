use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod sys {

    // TODO: clean up imports.
    use elevated_command; // 13.2 KiB (for elevation status)
    use std::{
        env::consts,
        io::{Error, ErrorKind},
        process::{self},
        result::Result,
    };
    use whoami; // 24.1 KiB (for username and hostname)

    #[derive(Clone, PartialEq, Debug)]
    pub enum OsEnum {
        Linux,
        Windows,
        MacOs,
        OpenBSD,
        FreeBSD,
        Other(String),
    }

    /// Returns `OsEnum` that describes the agents OS.
    ///
    /// > [!INFO]
    /// > If the agent is running something other than: Linux, Windows, MacOs, OpenBSD, or FreeBSD,
    /// > the type of OS will be stored in `OsEnum::Other`
    ///
    /// > [!INFO]
    /// > Most flavors of linux (of course, not all tested) show up as "linux"
    // NOTE: consts::FAMILY has been more useful than consts::OS
    // consider switing this out?
    pub fn os_name() -> OsEnum {
        // possible values of consts::OS :
        //      https://doc.rust-lang.org/std/env/consts/constant.OS.html
        match consts::OS {
            "linux" => OsEnum::Linux,
            "windows" => OsEnum::Windows,
            "macos" => OsEnum::MacOs,
            "openbsd" => OsEnum::OpenBSD,
            "freebsd" => OsEnum::FreeBSD,
            other => OsEnum::Other(other.to_string()),
        }
    }

    /// Returns the username of the agent
    // TODO: whoes username? the user who is running Hermes?
    pub fn username() -> String {
        whoami::username()
    }

    /// Returns hostname of agent
    /// > [!CAUTION]
    /// > This is failable
    #[rhai_fn(return_raw)]
    pub fn hostname() -> Result<String, Box<EvalAltResult>> {
        match whoami::fallible::hostname() {
            Ok(res) => Ok(res),
            Err(error) => Err(error.to_string().into()),
        }
    }

    /// Checks if user has admin privilidges or admin like privilidges
    /// > [!CAUTION]
    /// > Untested on Windows machines.
    // TODO: Change this to return just a bool, however the #[export_module]
    // requies the returns to be an iterator.. unsure what to do tbh - ruby
    #[rhai_fn(return_raw)]
    pub fn is_admin() -> Result<bool, Box<EvalAltResult>> {
        #[cfg(target_family = "unix")]
        {
            unsafe extern "C" {
                fn getuid() -> u32;
            }

            unsafe {
                return Ok(getuid() == 0);
            }
        }

        // WARNING: UNTESTED on windows
        #[cfg(target_family = "windows")]
        {
            return Ok(elevated_command::Command::is_elevated());
        }
    }

    /// Runs the `reboot` command on a given machine
    /// > [!CAUTION]
    /// > Untested on Windows machines.
    // NOTE: For unix, this is not the most correct means of going about this.
    // Would instead target the d-bus, but this should suffice.
    #[rhai_fn(return_raw)]
    pub fn reboot() -> Result<(), Box<EvalAltResult>> {
        let res = match consts::FAMILY {
            "unix" => process::Command::new("shutdown")
                .args(vec!["-r", "now"])
                .status(),
            // WARNING: UNTESTED
            "windows" => process::Command::new("shutdown").args(vec!["/r"]).status(),
            _ => Err(Error::new(
                ErrorKind::Unsupported,
                format!("unsupported OS {}", consts::FAMILY),
            )),
        };

        // QUESTION:
        // Do we return an enum of the status to determine success? Might be more useful
        // So on _exit_status == 0, we would return CMD_STATUS.ok
        // Or    _exit_status != 0, we would return CMD_STATUS.not_ok
        match res {
            Ok(_exit_status) => Ok(()),
            Err(error) => Err(error.to_string().into()),
        }
    }

    /// Runs the `shutdown` command on a given machine
    /// > [!CAUTION]
    /// > Untested on Windows machines.
    // NOTE: For unix, this is not the most correct means of going about this.
    // Would instead target the d-bus, but this should suffice.
    #[rhai_fn(return_raw)]
    pub fn shutdown() -> Result<(), Box<EvalAltResult>> {
        let res = match consts::FAMILY {
            "unix" => process::Command::new("shutdown").args(vec!["now"]).status(),
            // WARNING: UNTESTED
            "windows" => process::Command::new("shutdown").args(vec!["/s"]).status(),
            _ => Err(Error::new(
                ErrorKind::Unsupported,
                format!("unsupported OS {}", consts::FAMILY),
            )),
        };

        // QUESTION:
        // Do we return an enum of the status to determine success? Might be more useful
        // So on _exit_status == 0, we would return CMD_STATUS.ok
        // Or    _exit_status != 0, we would return CMD_STATUS.not_ok
        match res {
            Ok(_exit_status) => Ok(()),
            Err(error) => Err(error.to_string().into()),
        }
    }

    /// Checks the uptime a of the machine in seconds
    /// > [!CAUTION}
    /// > Only supports unix machines. Windows will come later
    #[rhai_fn(return_raw)]
    pub fn uptime() -> Result<f32, Box<EvalAltResult>> {
        let time_vec;
        let uptime;
        let init_result;
        let family = consts::FAMILY;

        // idiomatic? unsure.
        if family == "unix" {
            // attempt to read from `/prov/uptime`
            init_result = std::fs::read_to_string("/proc/uptime");

            // if `init_result` is Ok and is not empty, split into vector
            if init_result.as_ref().is_ok_and(|s| !s.is_empty()) {
                time_vec = init_result
                    .as_ref()
                    .unwrap()
                    .trim()
                    .split(" ")
                    .collect::<Vec<&str>>();
            } else {
                return Err(init_result.unwrap_err().to_string().into());
            }

            // `/proc/uptime` carries both idle time and uptime.
            // Our vec, then, must have lenght of 2, else something is wrong
            if time_vec.len() == 2 {
                // uptime lives in index 0
                uptime = time_vec[0].parse::<f32>();
            } else {
                return Err("uptime is malformed. Expected [<uptime>, <idletime>]"
                    .to_string()
                    .into());
            }

            match uptime {
                Ok(time) => Ok(time),
                Err(error) => Err(error.to_string().into()),
            }
        } else {
            // Working on Windows next. Don't have it in me at the moment - ruby
            // NOTE: Not in love that this error format is different than the rest - ruby
            return Err(Box::new(format!("unsupported OS: {}", family).into()));
        }
    }

    /// Returns CPU architecture
    pub fn cpu_architecture() -> String {
        whoami::arch().to_string()
    }

    /// Return `true` if the agent is running Windows (any version)
    pub fn is_windows() -> bool {
        consts::OS == "windows"
    }

    /// Return `true` if the agent is running Linux
    /// > ![INFO]
    /// > Most flavors of linux are returned as just "linux".
    pub fn is_linux() -> bool {
        consts::OS == "linux"
    }

    /// Returns `true` if the agent is running MacOs (any version)
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

    #[cfg(target_family = "unix")]
    #[test]
    fn test_uptime() {
        let uptime = uptime();
        match uptime {
            Ok(res) => assert_ne!(res, 0.0f32),
            Err(error) => panic!("uptime() failed for other reasons: {}", error),
        };
    }
}
