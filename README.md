# lite-async-test
An `#![async_test]` wrapper using futures-lite, for easily setting up async tests in your crates/binaries.

This is implemented using a very simple procedural macro which wraps the original test with an synchronous version which uses futures-lite to execute.

This crate is suitable for usage in `no_std` targets.

To use, simply include 
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use lite_async_test::async_test;

    #[async_test]
    async fn some_test() {
        assert_eq!(value, get_value().await);
    }
}

```

Disclaimer: This library is not an official product, use freely at your own risk.

