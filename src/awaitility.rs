use crate::config::Config;
use std::time::Duration;
use super::{most, least};

pub struct Awaitility<'a> {
    config: Config<'a>,
}

pub const fn new() -> Awaitility<'static> {
    Awaitility {
        config: Config::default(),
    }
}

impl<'a> Awaitility<'a> {
    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.config.set_interval(interval);
        self
    }

    #[inline]
    pub fn at_most(&self, duration: Duration) -> most::MostWait<'a> {
        most::at_most_config(duration, self.config.clone())
    }

    #[inline]
    pub fn at_least(&self, duration: Duration) -> least::LeastWait<'a> {
        least::at_least_config(duration, self.config.clone())
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
}
