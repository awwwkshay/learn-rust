# Export progress

This is an untouched starter under `exercises/export-progress/`. Ask to start it when you want `prepare-rust-exercise` to create a root working copy and add that copy to the Cargo workspace. Work in the root copy. The starter compiles; its tests and demo intentionally stop at `todo!()`.

## The situation

An operator exports several report sections and wants to see progress as each section completes. The export runs on a worker thread, while the caller receives events one at a time. Build the small component that connects those sides. Section rows and failures are simulated so this exercise stays focused on coordination rather than file formats.

Your parallel file audit started threads and waited for their final results. The main new concept here is **message passing with `std::sync::mpsc`**: a worker owns the sections and sends events as work progresses, while the caller owns the receiver. This is a separate practice path from the async background digest starter; it does not require Tokio.

## Required behavior

- `start_export(sections)` creates one `std::sync::mpsc::channel`, starts one `std::thread::spawn` worker, and returns the receiver and join handle immediately. Move the sections and sender into the worker. Do not keep another sender in the caller.
- The worker first sends `Started { sections }`, using the input length. For each successful section, add its `rows` to a `u64` running total and send `SectionDone { name, total_rows }` in input order. A zero-row section still gets a `SectionDone` event.
- If a section has `fail: true`, send `Failed { name, total_rows }` with the total from **earlier successful sections**, then stop. Do not count that section, process later sections, or send `Finished`. If all sections succeed, send `Finished { total_rows }`. Empty input produces `Started { sections: 0 }` followed by `Finished { total_rows: 0 }`.
- `finish_export(receiver, worker)` receives all events until the channel closes, preserving their send order, then joins the worker. Return those events on a successful join or `WorkerPanicked` if the worker panicked. Its tests supply a worker directly, so you can implement this independently of `start_export`.
- `format_event(event)` returns one operator-facing line **without** a newline. Use the exact formats in the demo and tests: `starting N sections`, `done NAME: N rows total`, `failed NAME after N rows`, and `finished: N rows`. The supplied CLI adds newlines as it prints events as they arrive.

The failure case is the small twist: an operator must see the partial total and know that no completion event follows a failed section. Keep section names exactly as supplied; no parser or name validation is needed.

The standard library is enough. [`mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) returns a `Sender<T>` and `Receiver<T>`. `sender.send(event)` transfers an owned event; if the receiver has been dropped, it returns an error. `for event in receiver` waits for events and ends once **all** senders have been dropped. Move the only sender into the worker so the receiving side can detect completion. `JoinHandle::join()` then tells you whether the worker completed or panicked. This channel has no fixed capacity; trying a bounded `sync_channel` is an optional later step.

## Example and commands

From inside this crate, run:

```sh
cargo test
cargo run
```

Once implemented, `cargo run` prints exactly:

```text
starting 2 sections
done orders: 3 rows total
done customers: 5 rows total
finished: 5 rows
```

The tests cover the normal sequence, partial failure, empty input, zero rows, a supplied channel, a panicking worker, and formatting. The starter's test failures and demo panic at TODOs are expected.

## Suggested order

1. Implement `format_event`; its test creates events directly.
2. Implement `finish_export` with the supplied-channel tests.
3. Implement `start_export`, then check successful, failed, and empty exports.
4. Run the demo and review when the receiver loop ends.

## Success criteria

`cargo test` passes and `cargo run` prints the sequence above. Explain who owns the sender and receiver, why the receiver loop eventually ends, and why a failed section has no `Finished` event.

## Optional stretch goals

- Try `sync_channel` with a small capacity and observe how a slow receiver changes the producer's progress.
- Let a caller stop receiving early and make the worker exit cleanly when `send` reports a disconnected receiver.
