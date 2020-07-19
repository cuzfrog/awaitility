//! # Awaitility for Rust
//! Fast, Simple, Straightforward Test utility for async functionalities.
//! ## Basic Usage
//! ```rust
//! # use std::time::Duration;
//! # fn test_something_is_true() -> bool { true }
//! awaitility::at_most(Duration::from_millis(100)).until(|| {test_something_is_true()});
//! awaitility::at_least(Duration::from_millis(50)).always(|| {test_something_is_true()});
//! // ...
//! ```
//! ## Examples
//! ```rust
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::sync::Arc;
//! use std::time::Duration;
//! 
//! fn at_most_test() {
//!  let counter = Arc::new(AtomicUsize::new(5));
//!  let tcounter = counter.clone();
//!  std::thread::spawn(move || {
//!    while tcounter.load(Ordering::SeqCst) < 15 {
//!      tcounter.fetch_add(1, Ordering::SeqCst);
//!    }
//!  });
//!  awaitility::at_most(Duration::from_millis(100)).until(|| counter.load(Ordering::SeqCst) > 10);
//! }
//! ```
//! ## Config
//! ```rust
//! # use std::time::Duration;
//! awaitility::at_most(Duration::from_millis(100))
//!            .poll_interval(Duration::from_millis(45))
//!            .describe("Becomes sunny..")
//!            .until(|| 2 > 1);
//! ```
//! 
//! ## Share configured instance
//! ```rust
//! use std::time::Duration;
//! 
//! let aw = awaitility::new().poll_interval(Duration::from_millis(45));
//! aw.at_least(Duration::from_millis(10)).always(|| 2 > 1);
//! aw.at_least(Duration::from_millis(10)).once(|| 2 > 1);
//! ```
//! Further configs made after `at_least` will not be reflected on instance `aw`.

mod awaitility;
mod backend;
mod error;
mod least;
mod most;

use std::time::Duration;

/// Create an at-most wait directive with default configs.
#[inline]
pub fn at_most(duration: Duration) -> most::MostWait<'static> {
    most::at_most(duration)
}

/// Create an at-least wait directive with default configs.
#[inline]
pub fn at_least(duration: Duration) -> least::LeastWait<'static> {
    least::at_least(duration)
}

/// Create an awaitility instance with shared configs.
#[inline]
pub fn new() -> awaitility::Awaitility<'static> {
    awaitility::new()
}

