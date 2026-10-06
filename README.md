# Learn Rust

Hands-on Rust projects. `exercises/` holds untouched starters, a project at the repository root is the active working copy while you solve it, and `solutions/` holds completed attempts.

## Active exercise: job runner

- [Working copy](job-runner/README.md)
- [Untouched starter](exercises/job-runner/README.md)

From the repository root:

```sh
cargo test -p job-runner
cargo run -p job-runner
```

After `cd job-runner`:

```sh
cargo test -p job-runner
cargo run -p job-runner
```

The demo creates its jobs in `main`; no path argument is needed. Tests and the demo currently stop at TODOs.

## LRU cache

- [Untouched exercise](exercises/lru-cache/README.md)
- [Completed first attempt](solutions/lru-cache/README.md)

The root working copy has been archived. To test and run the completed attempt from the repository root:

```sh
cargo test --manifest-path solutions/lru-cache/Cargo.toml
cargo run --manifest-path solutions/lru-cache/Cargo.toml
```

## Inventory ledger

- [Untouched exercise](exercises/inventory-ledger/README.md)
- [Completed first attempt](solutions/inventory-ledger/README.md)

The root working copy has been archived. To test and run the completed attempt from the repository root:

```sh
cargo test --manifest-path solutions/inventory-ledger/Cargo.toml
cargo run --manifest-path solutions/inventory-ledger/Cargo.toml -- solutions/inventory-ledger/sample.ledger
```

## Request log analyzer

- [Untouched exercise](exercises/request-log-analyzer/README.md)
- [Completed first attempt](solutions/request-log-analyzer/README.md)

The root working copy has been removed. Use `prepare-rust-exercise` to start another attempt. To test and run the completed attempt from the repository root:

```sh
cargo test --manifest-path solutions/request-log-analyzer/Cargo.toml
cargo run --manifest-path solutions/request-log-analyzer/Cargo.toml -- solutions/request-log-analyzer/sample.log
```
