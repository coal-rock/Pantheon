use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod sys {

    use crate::stdlib::error::error::ScriptError;

    use elevated_command; // 13.2 KiB (for elevation status)
    use std::{
        env::consts,
        process::{self},
        result::Result,
    };
    use whoami; // 24.1 KiB (for username and hostname)

    /// Returns `String` that describes the agents OS
    ///
    /// <details>
    /// <summary> Supported operating systems </summary>
    ///
    /// - "linux"
    /// - "windows"
    /// - "macos"
    /// - "openbsd"
    /// - "freebsd"
    /// - "other"
    /// </details>
    pub fn os_name() -> String {
        // possible values of consts::OS :
        //      https://doc.rust-lang.org/std/env/consts/constant.OS.html
        match consts::OS {
            "linux" | "windows" | "macos" | "openbsd" | "freebsd" => String::from(consts::OS),
            _ => String::from("other"),
        }
    }

    /// Returns a `String` that describes the family of the agents' operating system
    /// <details>
    /// <summary> Supported families </summary>
    ///
    /// - "unix"
    /// - "windows"
    /// - "other"
    /// </details>
    pub fn os_family() -> String {
        match consts::FAMILY {
            "unix" | "windows" => String::from(consts::FAMILY),
            _ => String::from("other"),
        }
    }
    /// Returns the username of the agent
    /// > [!CAUTION]
    /// > Raises `SysError` exception if unable to read username
    #[rhai_fn(return_raw)]
    pub fn username() -> Result<String, Box<EvalAltResult>> {
        match whoami::fallible::username() {
            Ok(res) => Ok(res),
            Err(error) => ScriptError::SysError(error.to_string()).into(),
        }
    }

    /// Returns hostname of agent
    /// > [!CAUTION]
    /// > Raises `SysError` exception if unable to obtain hostname.
    #[rhai_fn(return_raw)]
    pub fn hostname() -> Result<String, Box<EvalAltResult>> {
        match whoami::fallible::hostname() {
            Ok(res) => Ok(res),
            Err(error) => ScriptError::SysError(error.to_string()).into(),
        }
    }

    /// Returns `true` if agent has admin or admin like privilidges
    pub fn is_admin() -> bool {
        elevated_command::Command::is_elevated()
    }

    /// Runs the `reboot` command on a given machine.
    /// <details>
    /// <summary> Unix </summary>
    ///
    /// Invokes: `shutdown -r now`
    ///
    /// </details>
    ///
    /// <details>
    /// <summary> Windows </summary>
    ///
    /// Invokes: `shutdown /r`
    ///
    /// </details>
    ///
    /// <details>
    /// <summary> Other </summary>
    ///
    /// Raises `SysUnsupportedError` exception.
    ///
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn reboot() -> Result<(), Box<EvalAltResult>> {
        let res = match consts::FAMILY {
            "unix" => process::Command::new("shutdown")
                .args(vec!["-r", "now"])
                .status(),
            // WARNING: UNTESTED
            "windows" => process::Command::new("shutdown").args(vec!["/r"]).status(),
            _ => {
                // Return when unsupported family
                return ScriptError::SysUnsupportedError(
                    format!("Unsupported family: {}", consts::FAMILY).to_string(),
                )
                .into();
            }
        };

        match res {
            Ok(_exit_status) => Ok(()),
            Err(error) => ScriptError::SysError(error.to_string()).into(),
        }
    }

    /// Runs the `shutdown` command on a given machine.
    /// <details>
    /// <summary> Unix </summary>
    ///
    /// Invokes: `shutdown now`
    ///
    /// </details>
    ///
    /// <details>
    /// <summary> Windows </summary>
    ///
    /// Invokes: `shutdown /s`
    ///
    /// </details>
    ///
    /// <details>
    /// <summary> Other </summary>
    ///
    /// Raises `SysUnsupportedError` exception.
    ///
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn shutdown() -> Result<(), Box<EvalAltResult>> {
        let res = match consts::FAMILY {
            "unix" => process::Command::new("shutdown").args(vec!["now"]).status(),
            // WARNING: UNTESTED
            "windows" => process::Command::new("shutdown").args(vec!["/s"]).status(),
            // Return when unsupported family
            _ => {
                return ScriptError::SysUnsupportedError(
                    format!("Unsupported family: {}", consts::FAMILY).to_string(),
                )
                .into()
            }
        };

        match res {
            Ok(_exit_status) => Ok(()),
            Err(error) => ScriptError::SysError(error.to_string()).into(),
        }
    }

    /// Checks the uptime a of the machine in seconds
    /// > [!WARNING]
    /// > Only supported on Unix machines.
    ///
    /// > [!CAUTION]
    /// > Can raise `SysError` exception if `/proc/uptime` is malformed or inaccessible.
    #[rhai_fn(return_raw)]
    pub fn uptime() -> Result<f32, Box<EvalAltResult>> {
        if os_family() == "unix" {
            let proc_file = match std::fs::read_to_string("/proc/uptime") {
                Ok(ok) => ok,
                Err(error) => return ScriptError::SysError(error.to_string()).into(),
            };

            let time_vec = proc_file.trim().split(" ").collect::<Vec<&str>>();

            let uptime = match time_vec.len() == 2 {
                true => time_vec[0].parse::<f32>(),
                false => return ScriptError::SysError("uptime is malformed".to_string()).into(),
            };

            return match uptime {
                Ok(ok) => Ok(ok),
                Err(error) => ScriptError::SysError(error.to_string()).into(),
            };
        }

        ScriptError::SysUnsupportedError(
            format!("Unsupported family: {}", consts::FAMILY).to_string(),
        )
        .into()
    }

    /// Returns CPU architecture
    ///
    /// > [!NOTE]  
    /// > May return value not present in list if unknown architecture is detected.
    ///
    /// <details>
    /// <summary> Supported architectures </summary>
    ///
    /// - "armv5"
    /// - "armv6"
    /// - "armv7"
    /// - "arm64"
    /// - "i386"
    /// - "i586"
    /// - "i686"
    /// - "mips"
    /// - "mipsel"
    /// - "mips64"
    /// - "mips64el"
    /// - "powerpc"
    /// - "powerpc64"
    /// - "powerpc64le"
    /// - "riscv32"
    /// - "riscv64"
    /// - "s390x"
    /// - "sparc"
    /// - "sparc64"
    /// - "wasm32"
    /// - "wasm64"
    /// - "x86_64"
    /// </details>
    pub fn cpu_architecture() -> String {
        format!("{}", whoami::arch())
    }

    /// Return `true` if the agent is running Windows (any version)
    pub fn is_windows() -> bool {
        consts::OS == "windows"
    }

    /// Return `true` if the agent is running Linux
    pub fn is_linux() -> bool {
        consts::OS == "linux"
    }

    /// Returns `true` if the agent is running MacOs (any version)
    pub fn is_macos() -> bool {
        consts::OS == "macos"
    }

    /// Returns `true` if the agent is running OpenBSD or FreeBSD
    pub fn is_bsd() -> bool {
        match consts::OS {
            "openbsd" | "freebsd" => true,
            _ => false,
        }
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
        let res: String = os_name();
        assert_eq!(res, "linux");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn get_os_name() {
        // How to test this?
        let res: String = os_name();
        assert_eq!(res, "windows");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn get_os_name() {
        // How to test this?
        let res: String = os_name();
        assert_eq!(res, "macos");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn is_really_linux() {
        assert_eq!(is_linux(), true);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn is_not_windows() {
        assert_eq!(is_windows(), false);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn is_not_macos() {
        assert_eq!(is_macos(), false);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn is_not_bsd() {
        assert_eq!(is_bsd(), false);
    }

    #[test]
    fn get_username() {
        assert!(username().is_ok());
    }

    #[test]
    fn get_hostname() {
        assert!(hostname().is_ok());
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
