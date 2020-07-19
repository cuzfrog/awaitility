[![Crates.io][crates-badge]][crates-url]
[![Apache-2.0 licensed][license-badge]][license-url]
[![Build status][gh-actions-badge]][gh-actions-url]

[crates-badge]: https://img.shields.io/crates/v/awaitility.svg
[crates-url]: https://crates.io/crates/awaitility
[license-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[license-url]: LICENSE
[gh-actions-badge]: https://github.com/cuzfrog/awaitility/workflows/Test/badge.svg
[gh-actions-url]: https://github.com/cuzfrog/awaitility/actions

# Awaitility for Rust

Fast, Simple, Straightforward Test utility for async functionalities with 0 dependencies.

## Getting Started

```toml
[dev-dependencies]
awaitility = "0.1"
```

### Examples

```rust
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
  awaitility::at_most(Duration::from_millis(100)).until(|| counter.load(Ordering::SeqCst) > 10);
}

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
  awaitility::at_least(Duration::from_millis(100)).always(|| counter.load(Ordering::SeqCst) < 10);
}
```

### Config

```rust
use awaitility::Configurable;

awaitility::at_most(Duration::from_millis(100))
            .poll_interval(Duration::from_millis(45))
            .describe("Becomes sunny..")
            ...
```

## Licence

Apache License 2.0

## Author

Cause Chung (cuzfrog@gmail.com, cuzfrog@139.com)