# Learn Rust

Hands-on Rust projects. `exercises/` holds untouched starters, a project at the repository root is the active working copy, and `solutions/` holds completed attempts.

## Request log analyzer

- [Untouched exercise](exercises/request-log-analyzer/README.md)
- [Active working copy](request-log-analyzer/README.md)
- [Completed first attempt](solutions/request-log-analyzer/README.md)

From the repository root, test and run the active project:

```sh
cargo test -p request-log-analyzer
cargo run -p request-log-analyzer -- request-log-analyzer/sample.log
```

The active copy is currently a starter with `todo!()` calls, so its tests intentionally fail until the exercise is completed.
