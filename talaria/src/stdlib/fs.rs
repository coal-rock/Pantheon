use rhai::plugin::*;

use std::fs as std_fs;
use std::path::Path as std_Path;

/// Exposes cross-platform bindings for interacting with the filesystem
#[export_module]
pub mod fs {
    use crate::stdlib::error::error::ScriptError;
    use rhai::Array;
    use rhai::Dynamic;

    use crate::stdlib::CastArray;

    /// Reads the contents of a file to a string
    /// > [!CAUTION]
    /// > Can throw exception if `file_path` does not exist, or isn't readable
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `FileNotFound`
    /// - `PermissionDenied`
    /// - `IsADirectory`
    /// - `MalformedPath`
    /// - `InvalidUTF8`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn read(file_path: &str) -> Result<String, Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        match std_fs::read_to_string(path) {
            Ok(file_contents) => Ok(file_contents),
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => ScriptError::FsFileNotFound {
                    file_path: file_path.into(),
                },
                std::io::ErrorKind::PermissionDenied => ScriptError::FsPermissionDenied {
                    path: file_path.into(),
                    permission: "read".into(),
                },
                std::io::ErrorKind::IsADirectory => ScriptError::FsIsADirectory {
                    path: file_path.into(),
                },
                std::io::ErrorKind::InvalidInput => ScriptError::FsMalformedPath {
                    path: file_path.into(),
                },
                std::io::ErrorKind::InvalidData => ScriptError::FsInvalidUTF8 {
                    path: file_path.into(),
                },
                _ => ScriptError::FsError(err.to_string()),
            }
            .into(),
        }
    }

    /// Read the contents of a file into an array of strings, split on newline characters
    /// > [!CAUTION]
    /// > Can throw exception if `file_path` does not exist, or isn't readable
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `FileNotFound`
    /// - `PermissionDenied`
    /// - `IsADirectory`
    /// - `MalformedPath`
    /// - `InvalidUTF8`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn read_lines(file_path: &str) -> Result<Vec<Dynamic>, Box<EvalAltResult>> {
        read(file_path).map(|x| x.split('\n').map(|x| x.to_string().into()).collect())
    }

    /// Writes a string to a specified file
    ///
    /// Will create file and directory recursively if it doesn't exist
    /// > [!CAUTION]
    /// > Can throw exception if `file_path` isn't writable
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `FileNotFound`
    /// - `PermissionDenied`
    /// - `IsADirectory`
    /// - `ReadOnlyFilesystem`
    /// - `StorageFull`
    /// - `InvalidFilename`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn write(file_path: &str, contents: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        match std_fs::write(path, contents) {
            Ok(_) => Ok(()),
            Err(err) => match err.kind() {
                // we really should never have this unless some sort of race condition occurs
                std::io::ErrorKind::NotFound => ScriptError::FsFileNotFound {
                    file_path: file_path.into(),
                },
                std::io::ErrorKind::PermissionDenied => ScriptError::FsPermissionDenied {
                    path: file_path.into(),
                    permission: "write".into(),
                },
                std::io::ErrorKind::IsADirectory => ScriptError::FsIsADirectory {
                    path: file_path.into(),
                },
                std::io::ErrorKind::ReadOnlyFilesystem => ScriptError::FsReadOnlyFilesystem,
                std::io::ErrorKind::StorageFull => ScriptError::FsStorageFull,
                std::io::ErrorKind::InvalidFilename => ScriptError::FsInvalidFilename {
                    file_path: file_path.into(),
                },
                _ => ScriptError::FsError(err.to_string()),
            }
            .into(),
        }
    }

    /// Writes an array of strings to a specified file, adding a newline after each string
    ///
    /// Will create file and directory recursively if it doesn't exist
    /// > [!CAUTION]
    /// > Can throw exception if `file_path` isn't writable
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `FileNotFound`
    /// - `PermissionDenied`
    /// - `IsADirectory`
    /// - `ReadOnlyFilesystem`
    /// - `StorageFull`
    /// - `InvalidFilename`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn write_lines(file_path: &str, lines: Array) -> Result<(), Box<EvalAltResult>> {
        let lines = lines.try_cast::<String>()?;
        let lines = lines.join("\n") + "\n";
        write(file_path, &lines)
    }

    /// Appends a string to a file
    ///
    /// Will create file and directory recursively if it doesn't exist
    /// > [!CAUTION]
    /// > Can throw exception if `file_path` isn't writable
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `FileNotFound`
    /// - `PermissionDenied`
    /// - `IsADirectory`
    /// - `ReadOnlyFilesystem`
    /// - `StorageFull`
    /// - `InvalidFilename`
    /// - `InvalidUTF8`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn append(file_path: &str, content: &str) -> Result<(), Box<EvalAltResult>> {
        let old_content = read(file_path)?;
        write(file_path, &(old_content + content))
    }

    /// Appends an array of strings to a file, adding a newline after each string
    ///
    /// Will create file and directory recursively if it doesn't exist
    /// > [!CAUTION]
    /// > Can throw exception if `file_path` isn't writable
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `FileNotFound`
    /// - `PermissionDenied`
    /// - `IsADirectory`
    /// - `ReadOnlyFilesystem`
    /// - `StorageFull`
    /// - `InvalidFilename`
    /// - `InvalidUTF8`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn append_lines(file_path: &str, lines: Array) -> Result<(), Box<EvalAltResult>> {
        let lines = lines.try_cast::<String>()?;
        let lines = lines.join("\n") + "\n";
        append(file_path, &lines)
    }

    /// Removes a file or directory recursively
    /// > [!CAUTION]
    /// > Can throw exception if `path` can not be removed or does not exist
    ///
    /// <details>
    /// <summary> exceptions </summary>
    ///
    /// - `FileNotFound`
    /// - `PermissionDenied`
    /// - `ReadOnlyFilesystem`
    /// - `InvalidFilename`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn remove(file_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        let result = match (path.is_file(), path.is_dir()) {
            (true, false) => std::fs::remove_file(path),
            (false, true) => std::fs::remove_dir_all(path),
            _ => {
                return ScriptError::FsMalformedPath {
                    path: file_path.into(),
                }
                .into()
            }
        };

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => ScriptError::FsFileNotFound {
                    file_path: file_path.into(),
                },
                std::io::ErrorKind::PermissionDenied => ScriptError::FsPermissionDenied {
                    path: file_path.into(),
                    permission: "write".into(),
                },
                std::io::ErrorKind::ReadOnlyFilesystem => ScriptError::FsReadOnlyFilesystem,
                std::io::ErrorKind::InvalidFilename => ScriptError::FsInvalidFilename {
                    file_path: file_path.into(),
                },
                _ => ScriptError::FsError(err.to_string().into()),
            }
            .into(),
        }
    }

    /// Creates a file
    ///
    /// If the parent directory does not exist, it is created recursively
    /// > [!CAUTION]
    /// > Can throw exception if `file_path` can not be created
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `PermissionDenied`
    /// - `IsADirectory`
    /// - `NotADirectory`
    /// - `ReadOnlyFilesystem`
    /// - `InvalidFilename`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn create(file_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        match path.parent() {
            Some(parent) => mkdir(parent.to_str().unwrap())?,
            None => {}
        };

        match std_fs::File::create(file_path) {
            Ok(_) => Ok(()),
            Err(err) => match err.kind() {
                std::io::ErrorKind::PermissionDenied => ScriptError::FsPermissionDenied {
                    path: file_path.into(),
                    permission: "write".into(),
                },
                std::io::ErrorKind::IsADirectory => ScriptError::FsIsADirectory {
                    path: file_path.into(),
                },
                std::io::ErrorKind::NotADirectory => ScriptError::FsNotADirectory {
                    path: file_path.into(),
                },
                std::io::ErrorKind::ReadOnlyFilesystem => ScriptError::FsReadOnlyFilesystem,
                std::io::ErrorKind::StorageFull => ScriptError::FsStorageFull,
                std::io::ErrorKind::InvalidFilename => ScriptError::FsInvalidFilename {
                    file_path: file_path.into(),
                },
                _ => ScriptError::FsError(err.to_string().into()),
            }
            .into(),
        }
    }

    /// Creates a directory recursively
    /// > [!CAUTION]
    /// > Can throw exception if `dir_path` can not be created
    ///
    /// <details>
    /// <summary> Exceptions </summary>
    ///
    /// - `PermissionDenied`
    /// - `ReadOnlyFilesystem`
    /// - `NotADirectory`
    /// - `OtherFs`
    /// </details>
    #[rhai_fn(return_raw)]
    pub fn mkdir(dir_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(dir_path);

        match std_fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(err) => match err.kind() {
                std::io::ErrorKind::PermissionDenied => ScriptError::FsPermissionDenied {
                    path: dir_path.into(),
                    permission: "write".into(),
                },
                std::io::ErrorKind::ReadOnlyFilesystem => ScriptError::FsReadOnlyFilesystem,
                std::io::ErrorKind::NotADirectory => ScriptError::FsNotADirectory {
                    path: dir_path.into(),
                },
                _ => ScriptError::FsError(err.to_string()),
            }
            .into(),
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
