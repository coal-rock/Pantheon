/// Wrapper around println!() macro that
/// only runs if the binary was compiled in
/// debug mode
macro_rules! devlog{
    ($($rest:expr),+) => {
        {
            #[cfg(debug_assertions)]
            println!($($rest),+);
        }
    };
}

pub(crate) use devlog;
