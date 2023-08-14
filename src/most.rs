use crate::error::Timeout;
use super::backend::Backend;
use std::time::{Duration, Instant};
use std::panic::{AssertUnwindSafe, self};

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

    pub fn until_no_panic(&mut self, f: impl Fn()) -> &Self {
        let now = Instant::now();
        let mut result = panic::catch_unwind(AssertUnwindSafe(|| f()));
        while result.is_err() {
            result = panic::catch_unwind(AssertUnwindSafe(|| f()));
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                f();
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
    use std::{thread, time};
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
    fn at_most_using_assert() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            while tcounter.load(Ordering::SeqCst) < 15 {
                let ten_millis = time::Duration::from_millis(10);
                thread::sleep(ten_millis);
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_most(Duration::from_millis(1000)).until_no_panic(|| assert!(counter.load(Ordering::SeqCst) < 3));
    }

    #[test]
    #[should_panic]
    fn at_most_panic() {
        super::at_most(Duration::from_millis(30)).until(|| 1 > 2);
    }
}
