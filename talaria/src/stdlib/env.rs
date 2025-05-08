use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod env {
    use crate::stdlib::error::error::Error as script_error;

    use std::{env, num::NonZeroUsize};

    pub fn get(key: &str) -> Option<String> {
        match env::var(key) {
            Ok(ok) => Some(ok),
            Err(_) => None,
        }
    }

    #[rhai_fn(return_raw)]
    pub fn remove(key: &str) -> Result<(), Box<EvalAltResult>> {
        let _ = match env::consts::FAMILY {
            "itron" | "wasm" | "" => {
                return script_error::EnvUnsupprotedError(format!(
                    "Unsupported family: {}",
                    env::consts::FAMILY
                ))
                .into();
            }
            "unix" => match num_threads() {
                Some(current_threads) => {
                    if current_threads > NonZeroUsize::new(1).unwrap() {
                        return script_error::EnvMultiThreadedError(
                            "Hermes cannot remove env variable while multi threaded".to_string(),
                        )
                        .into();
                    }
                }
                None => (),
            },
            _ => (),
        };

        unsafe {
            env::remove_var(key);
        }

        match get(key) {
            Some(_) => {
                script_error::EnvFailedError(format!("Failed to remove ENV variable: {}", key))
                    .into()
            }
            None => Ok(()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn set(key: &str, value: &str) -> Result<(), Box<EvalAltResult>> {
        let _ = match env::consts::FAMILY {
            "itron" | "wasm" | "" => {
                return script_error::EnvUnsupprotedError(format!(
                    "Unsupported family: {}",
                    env::consts::FAMILY
                ))
                .into();
            }
            "unix" => match num_threads() {
                Some(current_threads) => {
                    if current_threads > NonZeroUsize::new(1).unwrap() {
                        return script_error::EnvMultiThreadedError(
                            "Hermes cannot remove env variable while multi threaded".to_string(),
                        )
                        .into();
                    }
                }
                None => (),
            },
            _ => (),
        };

        unsafe {
            env::set_var(key, value);
        }

        match get(key) {
            Some(set_value) => {
                if set_value != value {
                    // this is a "ask god" when you get there kind of error
                    script_error::EnvFailedError(format!(
                        "Failed to set ENV variable: ({}: {}). \
                        Unexpected variable. Current setting has \"{}\" as: {}",
                        key, value, key, set_value
                    ))
                    .into()
                } else {
                    Ok(())
                }
            }
            None => script_error::EnvFailedError(format!(
                "Failed to set ENV variable: ({}: {})",
                key, value
            ))
            .into(),
        }
    }

    // Does not protect agains invalid UTF-8
    pub fn list() -> Vec<(String, String)> {
        // TODO: Add supported family thing? Probably just add a function...
        let mut key_value: Vec<(String, String)> = Vec::new();
        let _ = env::vars().for_each(|x| key_value.push((x.0, x.1)));

        key_value
    }

    fn num_threads() -> Option<NonZeroUsize> {
        std::fs::read_to_string("/proc/self/stat")
            .ok()
            .as_ref()
            .and_then(|x| x.rsplit(')').next())
            .and_then(|x| x.split_whitespace().nth(17))
            .and_then(|x| x.parse::<usize>().ok())
            .and_then(NonZeroUsize::new)
    }
}

// TODO:
#[cfg(test)]
pub mod test {}
