use crate::config::Config;
use crate::config::DEFAULT_INTERVAL;
use std::time::{Duration, Instant};

pub struct MostWait {
    duration: Duration,
    config: Config,
}

pub fn at_most(duration: Duration) -> MostWait {
    MostWait {
        duration,
        config: Config {
            interval: DEFAULT_INTERVAL,
        },
    }
}

impl MostWait {
    pub fn until(&self, f: impl Fn() -> bool) {
        let now = Instant::now();
        while !f() {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                panic!("Condition not satisfied after {:?}.", elapsed);
            }
            std::thread::sleep(self.config.interval);
        }
    }
}
