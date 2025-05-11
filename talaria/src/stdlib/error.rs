use rhai::plugin::*;
use strum::EnumProperty;
use strum_macros::Display;
use strum_macros::{EnumProperty, IntoStaticStr};

#[export_module]
pub mod error {
    #[derive(Display, Debug, Clone, IntoStaticStr, EnumProperty)]
    pub enum ScriptError {
        #[strum(props(class = "fs", name = "otherFs"), to_string = "{0}")]
        FsError(String),

        #[strum(
            props(class = "fs", name = "fileNotFound"),
            to_string = "could not find file at path: \"{file_path}\""
        )]
        FsFileNotFound { file_path: String },

        #[strum(
            props(class = "fs", name = "directoryNotFound"),
            to_string = "could not find directory at path: \"{path}\""
        )]
        FsDirectoryNotFound { path: String },

        #[strum(
            props(class = "fs", name = "permissionDenied"),
            to_string = "user does not have permission to {permission} \"{path}\""
        )]
        FsPermissionDenied { path: String, permission: String },

        #[strum(
            props(class = "fs", name = "filenameTooLong"),
            to_string = "filename \"{filename}\" is too long"
        )]
        FsFilenameTooLong { filename: String },

        #[strum(
            props(class = "fs", name = "isADirectory"),
            to_string = "path: \"{path}\" is a directory"
        )]
        FsIsADirectory { path: String },

        #[strum(
            props(class = "fs", name = "notADirectory"),
            to_string = "path: \"{path}\" is not a directory"
        )]
        FsNotADirectory { path: String },

        #[strum(
            props(class = "fs", name = "malformedPath"),
            to_string = "path: \"{path}\" is malformed"
        )]
        FsMalformedPath { path: String },

        #[strum(
            props(class = "fs", name = "invalidUTF8"),
            to_string = "file: \"{path}\" contains invalid UTF-8"
        )]
        FsInvalidUTF8 { path: String },

        #[strum(
            props(class = "fs", name = "storageDeviceFull"),
            to_string = "storage device is full"
        )]
        FsStorageFull,

        #[strum(
            props(class = "fs", name = "readOnlyFilesystem"),
            to_string = "file system is readonly"
        )]
        FsReadOnlyFilesystem,

        #[strum(props(class = "sys"))]
        SysError(String),

        #[strum(props(class = "sys"))]
        SysUnsupportedError(String),
    }

    impl<T> Into<Result<T, Box<EvalAltResult>>> for ScriptError {
        fn into(self) -> Result<T, Box<EvalAltResult>> {
            Err(Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from(self),
                Position::NONE,
            )))
        }
    }

    /// Returns a pretty print of the error
    #[rhai_fn(get = "pretty", pure)]
    pub fn get_error_pretty(error: &mut ScriptError) -> String {
        let class = get_error_type(error);
        let name = get_error_name(error);
        let message = get_error_msg(error);

        format!("[{}] {} - {}", class, name, message)
    }

    /// Returns the class that this error belongs to
    #[rhai_fn(get = "class", pure)]
    pub fn get_error_type(error: &mut ScriptError) -> String {
        String::from(match error.get_str("class") {
            Some(class) => class,
            None => "unknown",
        })
    }

    /// Returns a deterministic string that can be matched upon to identify error type
    #[rhai_fn(get = "name", pure)]
    pub fn get_error_name(error: &mut ScriptError) -> String {
        String::from(match error.get_str("name") {
            Some(name) => name,
            None => "unknown",
        })
    }

    /// Returns message including both error context and message
    #[rhai_fn(get = "msg", pure)]
    pub fn get_error_msg(error: &mut ScriptError) -> String {
        error.to_string()
    }
}
