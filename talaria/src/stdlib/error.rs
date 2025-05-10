use rhai::plugin::*;
use strum::EnumProperty;
use strum_macros::{EnumProperty, IntoStaticStr};

#[export_module]
pub mod error {
    #[derive(Debug, Clone, IntoStaticStr, EnumProperty)]
    pub enum Error {
        #[strum(props(class = "fs"))]
        FsError(String),

        #[strum(props(class = "fs"))]
        FsNotFound(String),

        #[strum(props(class = "fs"))]
        FsPermissionDenied(String),

        #[strum(props(class = "fs"))]
        FsFilenameTooLong(String),

        #[strum(props(class = "fs"))]
        FsIsADirectory(String),

        #[strum(props(class = "fs"))]
        FsNotADirectory(String),

        #[strum(props(class = "fs"))]
        FsMalformedPath(String),

        #[strum(props(class = "fs"))]
        FsInvalidUTF8(String),

        #[strum(props(class = "fs"))]
        FsStorageFull(String),

        #[strum(props(class = "fs"))]
        FsReadOnlyFilesystem(String),

        #[strum(props(class = "sys"))]
        SysError(String),

        #[strum(props(class = "sys"))]
        SysUnsupportedError(String),
    }

    impl<T> Into<Result<T, Box<EvalAltResult>>> for Error {
        fn into(self) -> Result<T, Box<EvalAltResult>> {
            Err(Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from(self),
                Position::NONE,
            )))
        }
    }

    /// Returns the class that this error belongs to
    #[rhai_fn(get = "class", pure)]
    pub fn get_error_type(error: &mut Error) -> String {
        String::from(match error.get_str("class") {
            Some(class) => class,
            None => "unknown",
        })
    }

    /// Returns a deterministic string that can be matched upon to identify error type
    #[rhai_fn(get = "name", pure)]
    pub fn get_error_name(error: &mut Error) -> String {
        let error: &'static str = error.clone().into();
        error.to_string()
    }

    /// Returns message including both error context and message
    #[rhai_fn(get = "msg", pure)]
    pub fn get_error_msg(error: &mut Error) -> String {
        format!("{:?}", error)
    }
}
