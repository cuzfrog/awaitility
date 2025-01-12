use super::backend::Backend;
use std::{future::Future, time::{Duration, Instant}};

pub struct LeastWait<'a> {
    duration: Duration,
    backend: Backend<'a>,
}

pub fn at_least(duration: Duration) -> LeastWait<'static> {
    LeastWait {
        duration,
        backend: Backend::default(),
    }
}

pub fn at_least_backend<'a>(duration: Duration, backend: Backend<'a>) -> LeastWait<'a> {
    LeastWait {
        duration,
        backend,
    }
}

impl<'a> LeastWait<'a> {
    pub fn poll_interval(&mut self, interval: Duration) -> &mut Self {
        self.backend.set_interval(interval);
        self
    }

    pub fn describe<'b: 'a>(&mut self, desc: &'b str) -> &mut Self {
        self.backend.set_description(desc);
        self
    }

    pub fn always(&mut self, f: impl Fn() -> bool) -> &Self {
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                break;
            }
            if !f() {
                let desc = format!("Condition failed before duration {:?} elapsed.", elapsed);
                self.backend.fail(&desc);
                break;
            }
            std::thread::sleep(self.backend.interval);
        }
        self
    }

    pub async fn always_async<Fut>(&mut self, f: impl Fn() -> Fut) -> &Self where Fut: Future<Output = bool> {
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                break;
            }
            if !f().await {
                let desc = format!("Condition failed before duration {:?} elapsed.", elapsed);
                self.backend.fail(&desc);
                break;
            }
            std::thread::sleep(self.backend.interval);
        }
        self
    }

    pub fn once(&mut self, f: impl Fn() -> bool) -> &Self {
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                let desc = format!("Condition failed before duration {:?} elapsed.", elapsed);
                self.backend.fail(&desc);
                break;
            }
            if f() {
                break;
            }
            std::thread::sleep(self.backend.interval);
        }
        self
    }

    pub async fn once_async<Fut>(&mut self, f: impl Fn() -> Fut) -> &Self where Fut: Future<Output = bool> {
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if elapsed > self.duration {
                let desc = format!("Condition failed before duration {:?} elapsed.", elapsed);
                self.backend.fail(&desc);
                break;
            }
            if f().await {
                break;
            }
            std::thread::sleep(self.backend.interval);
        }
        self
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

    #[tokio::test]
    async fn at_least_async_fn() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(150));
            while tcounter.load(Ordering::SeqCst) < 15 {
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_least(Duration::from_millis(100)).always_async(|| async {
            counter.load(Ordering::SeqCst) < 10
        }).await;
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

    #[tokio::test]
    async fn once_async_fn() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            while tcounter.load(Ordering::SeqCst) < 15 {
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_least(Duration::from_millis(100)).once_async(|| async {
            counter.load(Ordering::SeqCst) < 10
        }).await;    
    }

    #[tokio::test]
    #[should_panic]
    async fn once_async_panic() {
        let counter = Arc::new(AtomicUsize::new(5));
        let tcounter = counter.clone();
        std::thread::spawn(move || {
            while tcounter.load(Ordering::SeqCst) < 15 {
                tcounter.fetch_add(1, Ordering::SeqCst);
            }
        });
        super::at_least(Duration::from_millis(100)).once_async(|| async {
            counter.load(Ordering::SeqCst) < 5
        }).await;    
    }
}
