use crate::config::Config;
use crate::config::DEFAULT_INTERVAL;
use std::time::{Duration, Instant};

pub struct LeastWait {
    duration: Duration,
    config: Config,
}

pub fn at_least(duration: Duration) -> LeastWait {
    LeastWait {
        duration,
        config: Config {
            interval: DEFAULT_INTERVAL,
        },
    }
}

impl LeastWait {
    pub fn always(&self, f: impl Fn() -> bool) {
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                break;
            }
            if !f() {
                panic!("Condition failed before duration {:?} elapsed.", elapsed);
            }
            std::thread::sleep(self.config.interval);
        }
    }
}
