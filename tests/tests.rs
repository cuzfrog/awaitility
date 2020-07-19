extern crate awaitility;
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
