# Learn Rust

Hands-on Rust projects. `exercises/` holds untouched starters, a project at the repository root is the active working copy while you solve it, and `solutions/` holds completed attempts.

## Request log analyzer

- [Untouched exercise](exercises/request-log-analyzer/README.md)
- [Completed first attempt](solutions/request-log-analyzer/README.md)

The root working copy has been removed. Use `prepare-rust-exercise` to start another attempt. To test and run the completed attempt from the repository root:

```sh
cargo test --manifest-path solutions/request-log-analyzer/Cargo.toml
cargo run --manifest-path solutions/request-log-analyzer/Cargo.toml -- solutions/request-log-analyzer/sample.log
```
