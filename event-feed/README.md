# Event feed

This is the active working copy. Implement the TODOs here; the untouched starter stays under `exercises/event-feed/`. The starter compiles; tests and the demo intentionally stop at `todo!()`.

An operations dashboard receives timestamp-ordered events from an application and a database. Build a reusable feed that merges the two streams so the dashboard can display events in time order as it requests them. The feed owns its events and yields each event once.

After the job runner's trait objects and the message pipeline's stateful closures, the main new concept is **implementing `Iterator` for your own stateful type**. You will also decide how to inspect the next event without consuming the wrong one and how to return owned values.

## Behavior

- `EventFeed::new(primary, secondary)` takes ownership of two `Vec<Event>` inputs. Each input must have nondecreasing timestamps. Equal timestamps within one input are valid. Reject an out-of-order input with an error naming `primary` or `secondary` and the one-based position of the offending event. Check both inputs, including when the other is empty.
- `next()` returns the earliest available event. On equal timestamps across inputs, return the primary event first. Preserve the original order within each input. Do not sort the inputs or collect the whole merged output before iteration.
- Once both inputs are exhausted, `next()` keeps returning `None`. Callers may pause between calls, use `take`, or consume the feed with a `for` loop.

The standard library is enough. The supplied `EventFeed` fields use `Peekable<IntoIter<Event>>`; you may change the internal representation while preserving the public behavior.

## Build it in stages

1. Implement `next()` using the two already-ordered inputs. The iteration tests construct a feed directly, so you can test this before validation.
2. Implement `new()` to validate each input and create the feed.
3. Run the demo and check that its output agrees with the expected timeline.

From the repository root, run:

```sh
cargo test -p event-feed
cargo run -p event-feed
```

Or after `cd event-feed`, run:

```sh
cargo test -p event-feed
cargo run -p event-feed
```

When complete, `cargo run` prints:

```text
1 [db] connected
2 [app] started
2 [app] ready
2 [db] query finished
5 [app] served request
```

The tests cover ordered input, invalid input from either source, empty inputs, stable ties, and pausing and resuming iteration. The starter's test failures and demo panic at TODOs are expected.

## Optional stretch goals

- Implement `size_hint()` and then `ExactSizeIterator` so callers can know how many events remain.
- Generalize the feed from two inputs to any number of timestamp-ordered inputs.
