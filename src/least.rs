use crate::config::Config;
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

pub fn at_least_config<'a>(duration: Duration, config: Config<'a>) -> LeastWait<'a> {
    LeastWait {
        duration,
        config,
    }
}

impl<'a> LeastWait<'a> {
    pub fn poll_interval(&mut self, interval: Duration) -> &mut Self {
        self.config.set_interval(interval);
        self
    }

    pub fn describe<'b: 'a>(&mut self, desc: &'b str) -> &mut Self {
        self.config.set_description(desc);
        self
    }

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

    pub fn once(&self, f: impl Fn() -> bool) {
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                let desc = format!("Condition failed before duration {:?} elapsed.", elapsed);
                self.config.fail(&desc);
            }
            if f() {
                break;
            }
            std::thread::sleep(self.config.interval);
        }
    }
}

#[cfg(test)]
mod least_test {
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
        super::at_least(Duration::from_millis(100)).always(|| counter.load(Ordering::SeqCst) < 10);
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

    #[test]
    fn once_test() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            while tcounter.load(Ordering::SeqCst) < 15 {
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_least(Duration::from_millis(100)).once(|| counter.load(Ordering::SeqCst) < 10);
    }
}
