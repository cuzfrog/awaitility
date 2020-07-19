use std::time::Duration;

pub static DEFAULT_INTERVAL: Duration = Duration::from_millis(50);

pub struct Config {
    pub interval: Duration,
}
