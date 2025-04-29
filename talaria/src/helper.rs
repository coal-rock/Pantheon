/// Wrapper around println!() macro that
/// only runs if the binary was compiled in
/// debug mode
#[macro_export]
macro_rules! devlog{
        ($($rest:expr),+) => {
            {
                #[cfg(debug_assertions)]
                println!($($rest),+);
            }
        };
    }
pub use devlog;

use std::time::SystemTime;
pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn current_time_micro() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}
