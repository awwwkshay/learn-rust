# Background digest

This is an untouched starter under `exercises/background-digest/`. Ask to start it when you want `prepare-rust-exercise` to make a root working copy and add it to the Cargo workspace. Work in that root copy. The starter compiles; its tests and demo intentionally stop at `todo!()`.

## The situation

A support operator wants a keyword snapshot of incoming ticket messages. Building the snapshot may take time, so an application should be able to start it, keep handling other work, and collect the result later. Build a small in-memory digest with that background lifecycle. The messages are local fixtures; no network service is needed.

In checkout preview, `tokio::join!` polled two futures while the caller awaited both. Here the main new concept is **spawning a task that owns its input and has a separate lifetime from the caller's current future**. You will choose where ownership changes hands and how the caller learns whether the task completed.

## Required behavior

- `build_digest(messages)` borrows the messages and counts whitespace-separated terms across all of them. Convert each term with `str::to_lowercase()` before counting. Keep punctuation as part of a term: `Rust.` and `rust` have separate counts. Empty or whitespace-only messages add nothing. Do not change the input messages.
- `Digest::count(term)` applies the same lowercasing to the query and returns its count, or zero when absent.
- `start_digest(messages, delay)` moves the owned `Vec<String>` into one `tokio::spawn` task and returns its `JoinHandle<Digest>` immediately. Inside the task, use `tokio::time::sleep(delay).await` to simulate an asynchronous dependency, then build the digest. The task must finish even if the caller does other async work before awaiting the handle. Do not use `std::thread::sleep` or a new OS thread.
- `finish_digest(handle)` awaits the handle and returns the completed digest. If the task failed to join, return `DigestError::TaskFailed` rather than panicking. The error covers a task panic or cancellation; implementing cancellation is outside the core task.

The core twist is the failed task: an application waiting for a background result needs a typed failure instead of a panic in its own task. The supplied test creates a panicking task to exercise this path without making `build_digest` panic on valid input.

Tokio is the only dependency. [`tokio::spawn`](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) starts an async task and returns a [`JoinHandle`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html). The future and its output must be `Send + 'static`, so the task needs owned data rather than a borrow of a caller's local `Vec`. Awaiting the handle gives `Result<Digest, JoinError>`; a successful join is different from a failed join. Dropping a handle detaches the task, so keep it until you can await it. Spawning on the supplied current-thread runtime lets waiting overlap with other async work; it does not make CPU indexing parallel. `#[tokio::test(start_paused = true)]` uses a virtual clock so the delay test runs quickly. The enabled Tokio features are `rt`, `macros`, `time`, and `test-util`.

## Example and commands

From inside this crate, run:

```sh
cargo test
cargo run
```

When implemented, `cargo run` prints exactly:

```text
digest started
rust: 3
tokio: 1
```

The tests cover ordinary counting, empty input, case-insensitive lookup, delayed background work, a completed task, and a task panic. Tests for `build_digest`, `Digest::count`, and `finish_digest` set up their own inputs so you can work on them independently. The starter's test failures and demo panic at TODOs are expected.

## Suggested order

1. Implement `build_digest` and `Digest::count` using the borrowed messages and the map.
2. Implement `finish_digest` with the tests that supply their own task handles.
3. Implement `start_digest`, then run the background test and demo.

## Success criteria

`cargo test` passes and `cargo run` prints the lines above. Explain why the task needs to own its messages, and what is different between awaiting a future directly and awaiting a spawned task's handle.

## Optional stretch goals

- Add a way to request cancellation and report it separately from a task panic.
- Add an incremental update operation that builds a new snapshot without changing an earlier completed digest.
