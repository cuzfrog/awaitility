mod config;
mod least;
mod most;

use crate::config::Config;
use std::time::Duration;

pub trait Configurable<'a> {
    fn get_config(&mut self) -> &mut Config<'a>;

    /// Default poll interval is 50 ms.
    fn poll_interval(&mut self, interval: Duration) -> &mut Self {
        self.get_config().set_interval(interval);
        self
    }

    fn describe<'b: 'a>(&mut self, desc: &'b str) -> &mut Self {
        self.get_config().set_description(desc);
        self
    }
}

pub fn at_most(duration: Duration) -> most::MostWait<'static> {
    most::at_most(duration)
}
pub fn at_least(duration: Duration) -> least::LeastWait<'static> {
    least::at_least(duration)
}
