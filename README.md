[![Crates.io][crates-badge]][crates-url]
[![Apache-2.0 licensed][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/awaitility.svg
[crates-url]: https://crates.io/crates/awaitility
[license-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[license-url]: LICENSE
[gh-actions-badge]: https://github.com/cuzfrog/awaitility/workflows/Release-CI/badge.svg
[gh-actions-url]: https://github.com/cuzfrog/awaitility/actions

# Awaitility for Rust

Fast, Simple, Straightforward Test utility for async functionalities.

## Getting Started

```toml
[dev-dependencies]
awaitility = "0.4"
```
Awaitility has dev-dependencies of tokio used for its own async fn testing.

### Basic Usage

```rust
awaitility::at_most(Duration::from_millis(100)).until(|| {test something is true});
awaitility::at_least(Duration::from_millis(100)).always(|| {test something is true});
awaitility::at_most(Duration::from_millis(100)).until_no_panic(|| {assert_eq!(1, 1)});

#[tokio::test]
async fn test() {
  awaitility::at_most(Duration::from_millis(100)).until_async(|| async {test something is true}).await;
}
// ...
```

See [RustDoc](https://docs.rs/awaitility) for more examples.

## Licence

Apache License 2.0

## Author

Cause Chung (cuzfrog@gmail.com, cuzfrog@139.com)
