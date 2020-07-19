use crate::config::Config;
use crate::Configurable;
use std::time::{Duration, Instant};

pub struct MostWait<'a> {
    duration: Duration,
    config: Config<'a>,
}

pub fn at_most(duration: Duration) -> MostWait<'static> {
    MostWait {
        duration,
        config: Config::default(),
    }
}

impl<'a> MostWait<'a> {
    pub fn until(&self, f: impl Fn() -> bool) {
        let now = Instant::now();
        while !f() {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                let desc = format!("Condition not satisfied after {:?}.", elapsed);
                self.config.fail(&desc);
            }
            std::thread::sleep(self.config.interval);
        }
    }
}

impl<'a> Configurable<'a> for MostWait<'a> {
    fn get_config(&mut self) -> &mut Config<'a> {
        &mut self.config
    }
}

#[cfg(test)]
mod most_test {
    use crate::Configurable;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn at_most_test() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            while tcounter.load(Ordering::SeqCst) < 15 {
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_most(Duration::from_millis(100))
            .poll_interval(Duration::from_millis(45))
            .until(|| counter.load(Ordering::SeqCst) > 10);
    }

    #[test]
    #[should_panic]
    fn at_most_panic() {
        super::at_most(Duration::from_millis(30)).until(|| 1 > 2);
    }
}
