use rhai::plugin::*;
use strum_macros::IntoStaticStr;

#[export_module]
pub mod error {
    #[derive(Debug, Clone, IntoStaticStr)]
    pub enum Error {
        FsError(String),
    }

    impl<T> Into<Result<T, Box<EvalAltResult>>> for Error {
        fn into(self) -> Result<T, Box<EvalAltResult>> {
            Err(Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from(self),
                Position::NONE,
            )))
        }
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
