use std::time::SystemTime;

// Helper to get current time in seconds since UNIX epoch
pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
