# Message pipeline

This is an untouched starter. Ask to start this exercise when you want `prepare-rust-exercise` to create a working copy at the repository root. The crate compiles; its tests and demo intentionally stop at `todo!()`.

An application sends messages through a series of transformations and checks before delivery. Build a reusable pipeline that applies those steps in registration order. A step can keep state between messages, such as a limit on how many it will accept.

Your job runner stored different `Job` implementations behind trait objects. This exercise takes one step further: it stores **closures with captured mutable state** using `FnMut`. It reuses familiar `Result` and ownership choices, but messages now flow through stages rather than leaving a queue as independent jobs.

## Behavior

- `new` creates an empty pipeline. `step_count` reports the number of registered steps.
- `add_step` accepts closures with different capture types. A step takes ownership of a `String` and returns a replacement `String` or an error.
- `process` passes a message through the steps in order. With no steps, it returns the input unchanged. On the first error, it returns that error and skips the remaining steps for that message. State changes in steps already called remain in effect.
- `process_batch` processes every supplied message in order and returns one result per message. One failure does not stop later messages. Captured state persists across calls to `process` and `process_batch`.

The small twist is the stateful rule: a closure may reject a later message based on earlier calls. Preserve the order and short-circuit behavior without resetting its captured state. The supplied tests cover ordinary transformation, empty inputs, registration order, failure, and state across a batch.

## Build it in stages

1. Construct the pipeline and count registered steps.
2. Store closures of different types behind the `Step` trait object.
3. Pass one owned message through every step, stopping on an error.
4. Process a batch while preserving order and allowing later messages after a failure.

From inside this crate, run:

```sh
cargo test
cargo run
```

When complete, the demo prints:

```text
1: HELLO
2: ERROR empty message
3: RUST
4: ERROR limit reached
```

The starter's test failures and demo panic at TODOs are expected. The standard library is enough.

## Optional stretch goals

- Include the failing step's index in an error without changing successful outputs.
- Add a `process_one_at_a_time` iterator interface that does not collect all batch results at once.
