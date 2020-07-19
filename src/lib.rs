mod config;
mod least;
mod most;

use std::time::Duration;

pub fn at_most(duration: Duration) -> most::MostWait {
    most::at_most(duration)
}
pub fn at_least(duration: Duration) -> least::LeastWait {
    least::at_least(duration)
}
