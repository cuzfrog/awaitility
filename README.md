# Awaitility for Rust

Fast, Simple, Straightforward Test utility for async functionalities.

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

### Licence

Apache License 2.0

### Author

Cause Chung (cuzfrog@gmail.com, cuzfrog@139.com)
