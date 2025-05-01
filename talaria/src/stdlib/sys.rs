use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod sys {

    use std::{
        env::consts,
        io::Error,
        process::{self, Command, Output, Stdio},
        result::Result,
    };
    use whoami; // 24.1 KiB

    #[derive(Clone, PartialEq, Debug)]
    pub enum OsEnum {
        Linux,
        Windows,
        MacOs,
        OpenBSD,
        FreeBSD,
        Other(String),
    }

    /// Returns `OsEnmu` that describes the agents OS.
    ///
    /// > [!INFO]
    /// > If the agent is running something other than: Linux, Windows, MacOs, OpenBSD, or FreeBSD,
    /// > the type of OS will be stored in `OsEnum::Other`
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

    /// Returns the username of the agent
    // TODO: whoes username? the user who is running Hermes?
    pub fn username() -> String {
        whoami::username()
    }

    /// Returns hostname of agent
    ///
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
    ///
    /// > [!CAUTION]
    /// > Only works on linux at the moment
    //  TODO: I think I made god cry
    #[rhai_fn(return_raw)]
    pub fn is_admin() -> Result<bool, Box<EvalAltResult>> {
        match os_name() {
            OsEnum::Linux => match run_command_capture_ouput("id", vec!["-u", username().as_str()])
            {
                Ok(output) => Ok(String::from_utf8_lossy(&output.stdout) == "0"),
                Err(error) => Err(error.to_string().into()),
            },
            // TODO: implement other OS.
            // windows is going to suck.
            _ => Err("Not implmented for this OS".into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn reboot() -> Result<(), Box<EvalAltResult>> {
        match os_name() {
            // OsEnum::Linux => run_command("shutdown", vec![""])?,
            _ => Ok(()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn shutdown() -> Result<(), Box<EvalAltResult>> {
        Ok(())
    }

    #[rhai_fn(return_raw)]
    pub fn uptime() -> Result<u64, Box<EvalAltResult>> {
        Ok(64u64)
    }

    /// Equivalent to getcwd on Unix an GetCurrentDirectoryW on Windows
    ///
    /// > [!CAUTION]
    /// > Can throw exceptions if their is not sufficent permmisions to access
    #[rhai_fn(return_raw)]
    pub fn hermes_dir() -> Result<String, Box<EvalAltResult>> {
        let directory = std::env::current_dir();
        match directory {
            Ok(path) => Ok(path.display().to_string()),
            Err(error) => Err(error.to_string().into()),
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
    ///
    /// > ![CAUTION]
    /// > Most flavors of linux are returned as just "linux".
    pub fn is_linux() -> bool {
        consts::OS == "linux"
    }

    /// Returns `true` if the agent is running MacOs (any version)
    pub fn is_macos() -> bool {
        consts::OS == "macos"
    }

    fn run_command_capture_ouput(cmd: &str, args: Vec<&str>) -> Result<process::Output, Error> {
        Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    }

    // #[rhai_fn(return_raw)]
    // fn run_command(cmd: &str, args: Vec<&str>) -> Result<(), Box<EvalAltResult>> {
    //     let mut cmd = Command::new(cmd).args(args).spawn()?;
    //
    //     cmd.wait()?;
    //
    //     Ok(())
    // }
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
