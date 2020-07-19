use crate::error::Timeout;
use super::backend::Backend;
use std::time::{Duration, Instant};

pub struct MostWait<'a> {
    duration: Duration,
    backend: Backend<'a>,
}

pub fn at_most(duration: Duration) -> MostWait<'static> {
    MostWait {
        duration,
        backend: Backend::default(),
    }
}

pub fn at_most_backend<'a>(duration: Duration, backend: Backend<'a>) -> MostWait<'a> {
    MostWait {
        duration,
        backend,
    }
}

impl<'a> MostWait<'a> {
    pub fn poll_interval(&mut self, interval: Duration) -> &mut Self {
        self.backend.set_interval(interval);
        self
    }

    pub fn describe<'b: 'a>(&mut self, desc: &'b str) -> &mut Self {
        self.backend.set_description(desc);
        self
    }

    pub fn result(&self) -> Result<(), Timeout> {
        self.backend.result.clone()
    }

    pub fn until(&mut self, f: impl Fn() -> bool) -> &Self {
        let now = Instant::now();
        while !f() {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                let desc = format!("Condition not satisfied after {:?}.", elapsed);
                self.backend.fail(&desc);
                break;
            }
            std::thread::sleep(self.backend.interval);
        }
        self
    }
}

#[cfg(test)]
mod most_test {
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
        super::at_most(Duration::from_millis(100)).until(|| counter.load(Ordering::SeqCst) > 10);
    }

    #[test]
    #[should_panic]
    fn at_most_panic() {
        super::at_most(Duration::from_millis(30)).until(|| 1 > 2);
    }
}
