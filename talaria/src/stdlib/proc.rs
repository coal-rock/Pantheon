use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod proc {

    use crate::stdlib::{error::error::ScriptError, CastArray};
    use rhai::Array;
    use sysinfo::{Pid, System};

    /// Returns an Array of all process names and their `pid`
    ///
    /// # Example
    /// ```terminal
    /// run rhai `let res = proc::list; for i in res { print(i); }`
    /// ```
    pub fn list() -> Array {
        /* rhai Arrays do not supprot tuples, a structure that makes sense here: (name, pid).
         * I do not love how the name and pid are one, but unsure how to solve this at the moment.
         */
        let sys = System::new_all();
        let mut proc_list: Array = Array::new();

        sys.processes().into_iter().for_each(|x| {
            let format = format!(
                "{} = {}",
                x.1.name().to_string_lossy(),
                x.0.as_u32().to_string()
            );
            proc_list.push(format.into());
        });

        proc_list
    }

    /// Kills a process given a `pid`
    ///
    /// > [!CAUTION]
    /// > Can throw exception if `pid` does not exist, if `pid` did not recieve signal,
    /// > the pid given is bad (negative, for instance), or if `pid` failed to be kill
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `ProcProcessDoesNotExist`
    /// - `ProcFailedToSendSignal`
    /// - `ProcFailedToKill`
    /// - `ProcBadPid`
    /// </details>
    ///
    /// # Example
    /// ```terminal
    /// run rhai `try { proc::kill(123456); } catch (err) { print(err.msg); }`
    /// ```
    #[rhai_fn(return_raw)]
    pub fn kill(pid: i64) -> Result<(), Box<EvalAltResult>> {
        // Explaining => pid: i64
        // take this command attempting to kill the pid 123456:
        // run rhai `try { proc::kill(123456); } catch (err) { print(err.msg); }`
        // 123456 has the type i64 by default, where we need usize for all pids.
        // this allows us to avoid casting in rhai directly.
        if pid <= 0 || pid >= usize::MAX.try_into().unwrap() {
            return ScriptError::ProcBadPid { pid: pid as usize }.into();
        }

        let pid = pid as usize;

        // Attempt to kill process
        if let Err(error) = kill_attempt(pid) {
            return error.into();
        };

        // Interval to check if pid if process is still alive.
        // Max 1.890 Seconds
        let mut check_interval_ms = vec![5, 10, 25, 50, 100, 200, 500, 1000].into_iter();

        let success = check_interval_ms.any(|interval| {
            let sys = System::new_all();
            // Check if process still exists
            if let None = sys.process(Pid::from(pid)) {
                return true;
            }

            // if process still exists, sleep and return false
            std::thread::sleep(std::time::Duration::from_millis(interval));
            return false;
        });

        // if any returned true, process was successfully killed
        match success {
            true => Ok(()),
            false => ScriptError::ProcFailedToKill { pid }.into(),
        }
    }

    // Helper function for killing a process
    // Simply sends the kill signal.
    //
    // Returns true  => successfully send kill sig
    // Returns false => failed to send kill sig (example: bad perms)
    fn kill_attempt(pid: usize) -> Result<(), ScriptError> {
        let sys = System::new_all();
        let proc = sys.process(Pid::from(pid));
        let res = match proc {
            Some(process) => process.kill(),
            None => return Err(ScriptError::ProcProcessDoesNotExist { pid }),
        };

        match res {
            true => Ok(()),
            false => Err(ScriptError::ProcFailedToSendSignal { pid }),
        }
    }

    /// Starts a command with arguments
    ///
    /// > [CAUTION]
    /// > Can throw exceptions if `args` cannot be casted, or if the process failed to start
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `ProcBadArguments`
    /// - `ProcFailedToStartProcess`
    /// </details>
    ///
    /// # Example
    /// ```terminal
    /// run rhai `try { proc::start("yes", ["pantheon is the best"]); } catch (err) { print(err.msg); }`
    /// ```
    #[rhai_fn(return_raw)]
    pub fn start(command: &str, args: Array) -> Result<u32, Box<EvalAltResult>> {
        let args = args.try_cast::<String>();
        let child = match args {
            Ok(vec_of_args) => std::process::Command::new(command)
                .args(vec_of_args)
                .spawn(),
            Err(_) => return ScriptError::ProcBadArguments.into(),
        };

        match child {
            Ok(child_p) => Ok(child_p.id()),
            Err(_) => ScriptError::ProcFailedToStartProcess {
                command: command.into(),
            }
            .into(),
        }
    }

    /// Returns the current `pid` of Hermes
    ///
    /// # Example
    /// ```terminal
    /// run rhai `print(proc::current_pid());`
    /// ```
    pub fn current_pid() -> u32 {
        std::process::id()
    }
}

// TODO:
// tests
