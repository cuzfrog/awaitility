use crate::config::Config;
use crate::Configurable;
use std::time::{Duration, Instant};

pub struct LeastWait<'a> {
    duration: Duration,
    config: Config<'a>,
}

pub fn at_least(duration: Duration) -> LeastWait<'static> {
    LeastWait {
        duration,
        config: Config::default(),
    }
}

impl LeastWait<'_> {
    pub fn always(&self, f: impl Fn() -> bool) {
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                break;
            }
            if !f() {
                let desc = format!("Condition failed before duration {:?} elapsed.", elapsed);
                self.config.fail(&desc);
            }
            std::thread::sleep(self.config.interval);
        }
    }
}

impl<'a> Configurable<'a> for LeastWait<'a> {
    fn get_config(&mut self) -> &mut Config<'a> {
        &mut self.config
    }
}

#[cfg(test)]
mod least_test {
    use crate::Configurable;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn at_least_test() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(150));
            while tcounter.load(Ordering::SeqCst) < 15 {
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_least(Duration::from_millis(100))
            .poll_interval(Duration::from_millis(45))
            .always(|| counter.load(Ordering::SeqCst) < 10);
    }

    #[test]
    #[should_panic]
    fn at_least_panic() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            while tcounter.load(Ordering::SeqCst) < 15 {
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_least(Duration::from_millis(100)).always(|| counter.load(Ordering::SeqCst) < 10);
    }
}
