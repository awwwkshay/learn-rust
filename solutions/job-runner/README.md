# Job runner

This is the completed first attempt, archived at `solutions/job-runner/`. The untouched starter remains under `exercises/job-runner/`.

A small application needs to run different kinds of queued work in the order it arrives and record each outcome. Build a reusable, in-memory job runner. It should keep going when one job fails.

The request-log and inventory exercises centered on file parsing, and the LRU cache centered on one data structure. This project changes the shape of the work: you will define behavior through a `Job` trait, store different implementations together as `Box<dyn Job>`, and manage ownership as jobs leave the queue. This is the main new Rust concept; `VecDeque`, `Result`, and enums should be familiar.

## Behavior

- `UppercaseJob` returns its input in uppercase. `WordCountJob` returns `N words`, where `N` is the number of whitespace-separated words. Both return their stored name through `Job::name`.
- `Scheduler` starts empty. `enqueue` accepts any boxed `Job` and adds it to the back of the queue. `len` and `is_empty` describe pending jobs.
- `run_next` removes the oldest job and returns its name and outcome. It returns `None` when empty. A failed job is still removed.
- `run_all` drains the queue in first-in, first-out order, returning one report per job. A failure does not stop later jobs.
- `format_reports` renders each report on its own line: `NAME: OUTPUT` on success or `NAME: ERROR MESSAGE` on failure, with no trailing newline. An empty report list formats as an empty string.

The supplied failure job in the tests shows how another type can implement the same trait. The standard library is enough. Leave the public types and signatures in place, but choose how to coordinate the queue and its reports.

## Build it in stages

1. Implement `Job` for the two concrete job types.
2. Make the scheduler hold and count heterogeneous jobs.
3. Run one job, then drain all jobs while preserving order and failures.
4. Format reports and run the demo.

From the repository root, run:

```sh
cargo test --manifest-path solutions/job-runner/Cargo.toml
cargo run --manifest-path solutions/job-runner/Cargo.toml
```

After `cd solutions/job-runner`, run:

```sh
cargo test
cargo run
```

The demo creates its jobs in `main`, so it needs no file-path argument.

When complete, the demo prints:

```text
headline: READY FOR REVIEW
word count: 5 words
```

The tests cover each job, an empty queue, mixed job types, failure recovery, and formatting.

## Optional stretch goals

- Add a generic `enqueue_job<J: Job + 'static>` convenience method so callers need not write `Box::new`.
- Add `run_n(&mut self, limit: usize)` and test that unrun jobs remain queued.
