use rhai::Dynamic;

pub mod crypto;
pub mod env;
pub mod fs;
pub mod http;
pub mod net;
pub mod proc;
pub mod sys;
pub mod time;

trait CastArray {
    type Error;

    fn try_cast<T: 'static>(self) -> Result<Vec<T>, Self::Error>;
}

impl CastArray for Vec<Dynamic> {
    type Error = String;

    fn try_cast<T: 'static>(self) -> Result<Vec<T>, Self::Error> {
        self.into_iter()
            .map(|item| {
                item.try_cast_result::<T>().map_err(|err| {
                    let expected = std::any::type_name::<T>();
                    format!("Expected {}, got: {}", expected, err.type_name())
                })
            })
            .collect()
    }
}
