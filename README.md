# Learn Rust

Hands-on Rust projects. `exercises/` holds untouched starters, a project at the repository root is the active working copy while you solve it, and `solutions/` holds completed attempts.

## Job runner

- [Untouched starter](exercises/job-runner/README.md)
- [Completed first attempt](solutions/job-runner/README.md)

The root working copy has been archived. From the repository root:

```sh
cargo test --manifest-path solutions/job-runner/Cargo.toml
cargo run --manifest-path solutions/job-runner/Cargo.toml
```

After `cd solutions/job-runner`:

```sh
cargo test
cargo run
```

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
