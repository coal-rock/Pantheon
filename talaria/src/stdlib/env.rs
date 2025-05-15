use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod env {
    use crate::stdlib::error::error::ScriptError as script_error;

    use std::env;

    /// Returns the value of a environment variable
    ///
    /// > [INFO]
    /// > It is perfectly legal to have an empty environment variable.
    #[rhai_fn(return_raw)]
    pub fn get(key: &str) -> Result<String, Box<EvalAltResult>> {
        if let Err(e) = supported_by_family() {
            return e.into();
        }

        match env::var(key) {
            Ok(ok) => {
                // Empty env variables are valid, and we should notify the user
                if ok == "" {
                    // Maybe throw an error? But what if this is intentional? IDK why
                    // Maybe Result<(String, boolean)>, where boolean is is_empty?
                    // gross.. but
                    println!("Empty env");
                }
                Ok(ok)
            }
            Err(error) => script_error::EnvFailedError(error.to_string()).into(),
        }
    }

    /// Removes an environment variable
    /// > [!CAUTION]
    /// > Race conditions can occur when calling `env::remove` and `env::set` at the same time.
    /// > This is due to lack of thread safety.
    #[rhai_fn(return_raw)]
    pub fn remove(key: &str) -> Result<(), Box<EvalAltResult>> {
        if let Err(e) = supported_by_family() {
            return e.into();
        }

        unsafe {
            env::remove_var(key);
        }

        match get(key) {
            Ok(value) => script_error::EnvFailedToRemoveVariable {
                key: key.into(),
                value: value.into(),
            }
            .into(),
            // this should return an error
            Err(_) => Ok(()),
        }
    }

    /// Sets and environment variable
    /// > [!CAUTION]
    /// > Race conditions can occur when calling `env::remove` and `env::set` at the same time.
    /// > This is due to lack of thread safety.
    #[rhai_fn(return_raw)]
    pub fn set(key: &str, value: &str) -> Result<(), Box<EvalAltResult>> {
        // TODO: Add checks for \0, =, and one other that I cannot remember...
        if let Err(e) = supported_by_family() {
            return e.into();
        }

        unsafe {
            env::set_var(key, value);
        }

        match get(key) {
            Ok(set_value) => {
                if set_value != value {
                    // Can occur when calling concurrent env::set and env::remove calls.
                    // Race condition
                    script_error::EnvFailedError(format!(
                        "Failed to set ENV variable: ({}: {}). \
                        Unexpected value. Current setting has \"{}\" as: {}. \
                        Most likely a race condition.",
                        key, value, key, set_value
                    ))
                    .into()
                } else {
                    Ok(())
                }
            }
            Err(error) => script_error::EnvFailedError(format!(
                "Failed to set ENV variable: ({}: {}). ERROR: {}",
                key, value, error
            ))
            .into(),
        }
    }

    /// Returns environment keys and values
    /// > [!CAUTION]
    /// > Does not protect against invalid UTF-8 characters.
    #[rhai_fn(return_raw)]
    pub fn list() -> Result<Vec<Dynamic>, Box<EvalAltResult>> {
        if let Err(e) = supported_by_family() {
            return e.into();
        }

        let mut key_value: Vec<Dynamic> = Vec::new();
        let _ = env::vars().for_each(|x| key_value.push(format!("{}={}", x.0, x.1).into()));

        Ok(key_value)
    }

    fn supported_by_family() -> Result<(), crate::stdlib::error::error::ScriptError> {
        match env::consts::FAMILY {
            "itron" | "wasm" | "" => Err(script_error::EnvUnsupprotedError(format!(
                "Unsupported family: {}",
                env::consts::FAMILY
            ))),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use env::*;

    #[test]
    fn set_var() {
        assert!(set("TEST_KEY_1", "TEST_VALUE_1").is_ok());
    }

    #[test]
    fn set_and_get_var() {
        let _ = set("TEST_KEY_2", "TEST_VALUE_2");
        assert_eq!(get("TEST_KEY_2").unwrap(), "TEST_VALUE_2");
    }

    #[test]
    fn set_get_and_remove() {
        let _ = set("TEST_KEY_3", "TEST_VALUE_3");
        assert_eq!(get("TEST_KEY_3").unwrap(), "TEST_VALUE_3");
        let _ = remove("TEST_KEY_3");
        assert!(!get("TEST_KEY_3").is_ok());
    }

    #[test]
    fn list_env_vars() {
        let res = list();
        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }
}
