//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com
use rhai::plugin::*;

use std::fs as std_fs;
use std::path::Path as std_Path;

// 1. Create a plugin module or any kind of Rhai API that supports documentation on functions and types.

/// My own module.
#[export_module]
pub mod fs {

    use rhai::Array;
    use rhai::Dynamic;
    use std::io::Write;

    use crate::stdlib::CastArray;

    /// Reads the contents of a file to a string
    #[rhai_fn(return_raw)]
    pub fn read(file_path: &str) -> Result<String, Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        match std_fs::read_to_string(path) {
            Ok(file_contents) => Ok(file_contents),
            Err(err) => Err(err.to_string().into()),
        }
    }

    /// Read the contents of a file into an array of strings, split on newline characters
    #[rhai_fn(return_raw)]
    pub fn read_lines(file_path: &str) -> Result<Vec<Dynamic>, Box<EvalAltResult>> {
        read(file_path).map(|x| x.split('\n').map(|x| x.to_string().into()).collect())
    }

    /// Writes a string to a specified file
    ///
    /// Will create file and directory recursively if it doesn't exist
    #[rhai_fn(return_raw)]
    pub fn write(file_path: &str, contents: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        match std_fs::write(path, contents) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    /// Writes an array of strings to a specified file, adding a newline after each string
    ///
    /// Will create file and directory recursively if it doesn't exist
    #[rhai_fn(return_raw)]
    pub fn write_lines(file_path: &str, lines: Array) -> Result<(), Box<EvalAltResult>> {
        let lines = lines.try_cast::<String>()?;
        let lines = lines.join("\n") + "\n";
        write(file_path, &lines)
    }

    /// Appends a string to a file
    ///
    /// Will create file and directory recursively if it doesn't exist
    #[rhai_fn(return_raw)]
    pub fn append(file_path: &str, content: &str) -> Result<(), Box<EvalAltResult>> {
        let mut file = match std_fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
        {
            Ok(file) => file,
            Err(err) => return Err(err.to_string().into()),
        };

        match file.write(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    /// Appends an array of strings to a file, adding a newline after each string
    ///
    /// Will create file and directory recursively if it doesn't exist
    #[rhai_fn(return_raw)]
    pub fn append_lines(file_path: &str, lines: Array) -> Result<(), Box<EvalAltResult>> {
        let lines = lines.try_cast::<String>()?;
        let lines = lines.join("\n") + "\n";
        append(file_path, &lines)
    }

    /// Removes a file or directory recursively
    #[rhai_fn(return_raw)]
    pub fn remove(file_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        if path.is_file() {
            return match std::fs::remove_file(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string().into()),
            };
        }

        if path.is_dir() {
            return match std::fs::remove_dir_all(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string().into()),
            };
        }

        Err("Provided path is neither a file nor a directory".into())
    }

    /// Creates a file
    ///
    /// If the parent directory does not exist, it is created recursively
    #[rhai_fn(return_raw)]
    pub fn create(file_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        let result = match path.parent() {
            Some(parent) => mkdir(parent.to_str().unwrap()),
            None => Ok(()),
        };

        match result {
            Ok(_) => {}
            Err(err) => return Err(err.to_string().into()),
        };

        match std_fs::File::create(file_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    /// Creates a directory recursively
    #[rhai_fn(return_raw)]
    pub fn mkdir(dir_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(dir_path);

        match std_fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    /// Return `true` if directory or file exists at specified path
    pub fn exists(path: &str) -> bool {
        std_Path::new(path).exists()
    }

    /// Returns `true` if provided path points to a file
    ///
    /// Returns `false` under all other conditions
    pub fn is_file(path: &str) -> bool {
        std_Path::new(path).is_file()
    }

    /// Returns `true` if provided path points to a directory
    ///
    /// Returns `false` under all other conditions
    pub fn is_dir(path: &str) -> bool {
        std_Path::new(path).is_dir()
    }
}
