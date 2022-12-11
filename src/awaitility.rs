use super::backend::Backend;
use std::time::Duration;
use super::{most, least};

pub struct Awaitility<'a> {
    backend: Backend<'a>,
}

pub const fn new() -> Awaitility<'static> {
    Awaitility {
        backend: Backend::default(),
    }
}

impl<'a> Awaitility<'a> {
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.backend.set_interval(interval);
        self
    }

    pub fn set_return(mut self) -> Self {
        self.backend.set_return();
        self
    }

    #[inline]
    pub fn at_most(&self, duration: Duration) -> most::MostWait<'a> {
        most::at_most_backend(duration, self.backend.clone())
    }

    #[inline]
    pub fn at_least(&self, duration: Duration) -> least::LeastWait<'a> {
        least::at_least_backend(duration, self.backend.clone())
    }
}

#[cfg(test)]
mod awaitility_test {
    use std::time::Duration;

    #[test]
    fn create_at_most() {
        let aw = super::new().poll_interval(Duration::from_millis(50));
        aw.at_most(Duration::from_millis(10)).until(|| 2 > 1);
    }

    #[test]
    fn create_at_least() {
        let aw = super::new().poll_interval(Duration::from_millis(50));
        aw.at_least(Duration::from_millis(10)).always(|| 2 > 1);
    }

    #[test]
    fn set_return_test() {
        let res = super::new().set_return().at_most(Duration::from_millis(10)).until(|| 3 > 2).result();
        assert!(res.is_ok());
    }
}
